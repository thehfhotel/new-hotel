using System;
using System.CodeDom.Compiler;
using System.Collections;
using System.ComponentModel;
using System.ComponentModel.Design;
using System.Diagnostics;
using System.Reflection;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Windows.Forms;
using iHOTEL2025; // REFERENCE-CODEBASE PATCH: VB My namespace lookup let this resolve unqualified types from the parent iHOTEL2025 namespace; C# needs the explicit using.
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.ApplicationServices;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025.My;

[StandardModule]
[GeneratedCode("MyTemplate", "8.0.0.0")]
[HideModuleName]
internal sealed class MyProject
{
	[EditorBrowsable(EditorBrowsableState.Never)]
	[MyGroupCollection("System.Windows.Forms.Form", "Create__Instance__", "Dispose__Instance__", "My.MyProject.Forms")]
	internal sealed class MyForms
	{
		public AboutBox1 m_AboutBox1;

		public AlertCustom m_AlertCustom;

		public ClickAddmore m_ClickAddmore;

		public ClickAvliable m_ClickAvliable;

		public ClickAvliable_book m_ClickAvliable_book;

		public ClickBook m_ClickBook;

		public ClickBook_book m_ClickBook_book;

		public ClickClean m_ClickClean;

		public ClickCleanOK m_ClickCleanOK;

		public ClickManternance m_ClickManternance;

		public ClickUSE m_ClickUSE;

		public ClickUSE2 m_ClickUSE2;

		public ClickUSE3 m_ClickUSE3;

		public connect_mssql m_connect_mssql;

		public EMP_Note m_EMP_Note;

		public EMP_Note_Read m_EMP_Note_Read;

		public Form1 m_Form1;

		public FormEN_DE formEN_DE_0;

		public FormART m_FormART;

		public FormBookingInvoice m_FormBookingInvoice;

		public FormBookRooms m_FormBookRooms;

		public FormConfirmOverBill m_FormConfirmOverBill;

		public FormConfirmPay m_FormConfirmPay;

		public FormConfirmRoundBill m_FormConfirmRoundBill;

		public FormEditPay m_FormEditPay;

		public FormFolio m_FormFolio;

		public FormLog m_FormLog;

		public FormManageOrderCust m_FormManageOrderCust;

		public FormManageOrderCustDown m_FormManageOrderCustDown;

		public FormPass m_FormPass;

		public FormReportAll m_FormReportAll;

		public FormReportAll2 m_FormReportAll2;

		public FormRoomMain m_FormRoomMain;

		public FormRoomMain_ViewBook m_FormRoomMain_ViewBook;

		public FormRoomMainClean m_FormRoomMainClean;

		public FormRoomMainKichen m_FormRoomMainKichen;

		public FormRoomMainKichen_old m_FormRoomMainKichen_old;

		public FormSearchBooking m_FormSearchBooking;

		public FormSearchBooking2 m_FormSearchBooking2;

		public FormSearchChechIn m_FormSearchChechIn;

		public FormSearchChechInnotOut m_FormSearchChechInnotOut;

		public GForm1 m_FormSearchChechInVAT;

		public FormSearchCust m_FormSearchCust;

		public FormSearchPro m_FormSearchPro;

		public FormSearchRooms m_FormSearchRooms;

		public FormSearchRooms2 m_FormSearchRooms2;

		public FormSearchRooms2_old m_FormSearchRooms2_old;

		public FormSearchRoomsCin m_FormSearchRoomsCin;

		public FormSearchRoomsCin2 m_FormSearchRoomsCin2;

		public FormSelectDB m_FormSelectDB;

		public FormSelectDB_old m_FormSelectDB_old;

		public FormSelectRoom m_FormSelectRoom;

		public FormSETelec m_FormSETelec;

		public FormShowDEP m_FormShowDEP;

		public FormShowDEPBack m_FormShowDEPBack;

		public FormShowSAVEout2 m_FormShowSAVEout2;

		public FormShowVAT m_FormShowVAT;

		public FormSMS_DEBT m_FormSMS_DEBT;

		public FormSMSLog m_FormSMSLog;

		public FormSMSSendManual m_FormSMSSendManual;

		public FormUPDATE m_FormUPDATE;

		public FormUpdateDateRoomAll m_FormUpdateDateRoomAll;

		public FormVatOver m_FormVatOver;

		public FrmAddBook m_FrmAddBook;

		public FrmAddBook2 m_FrmAddBook2;

		public FrmAddBook2copy m_FrmAddBook2copy;

		public FrmAddDep m_FrmAddDep;

		public FrmAddEditServer m_FrmAddEditServer;

		public FrmAddInvoiceSale m_FrmAddInvoiceSale;

		public FrmAddPay m_FrmAddPay;

		public FrmAddReg m_FrmAddReg;

		public FrmAddSale m_FrmAddSale;

		public FrmAddSale2 m_FrmAddSale2;

		public FrmAddSale2_Credit m_FrmAddSale2_Credit;

		public FrmAddSaveImage m_FrmAddSaveImage;

		public FrmBookMain m_FrmBookMain;

		public FrmBookMain2 m_FrmBookMain2;

		public FrmBookRooms m_FrmBookRooms;

		public frmCapture m_frmCapture;

		public FrmCheckIn m_FrmCheckIn;

		public FrmCheckIn_EditOnly m_FrmCheckIn_EditOnly;

		public FrmCheckOut m_FrmCheckOut;

		public FrmCuponMain m_FrmCuponMain;

		public FrmCustomers m_FrmCustomers;

		public FrmDepositMain m_FrmDepositMain;

		public FrmDueBill m_FrmDueBill;

		public FrmEditDate m_FrmEditDate;

		public FrmInOutMain m_FrmInOutMain;

		public frmMain1 m_frmMain1;

		public FrmManageCustomers m_FrmManageCustomers;

		public FrmManageCustomersNew m_FrmManageCustomersNew;

		public FrmManageCustomersSearch m_FrmManageCustomersSearch;

		public FrmManageProduct m_FrmManageProduct;

		public FrmManageRoom m_FrmManageRoom;

		public FrmPayAdd m_FrmPayAdd;

		public FrmPayAddDebt m_FrmPayAddDebt;

		public FrmPayAddPro m_FrmPayAddPro;

		public FrmPayDebt m_FrmPayDebt;

		public FrmPayDebt2 m_FrmPayDebt2;

		public FrmPayMain m_FrmPayMain;

		public FrmPermission m_FrmPermission;

		public FrmPriceHistory m_FrmPriceHistory;

		public FrmPrint m_FrmPrint;

		public FrmReceiptInvoice m_FrmReceiptInvoice;

		public FrmReceiptMain m_FrmReceiptMain;

		public FrmReceiptMain_invoice m_FrmReceiptMain_invoice;

		public frmReg m_frmReg;

		public FrmRegMain m_FrmRegMain;

		public FrmReportBook m_FrmReportBook;

		public FrmReportBook2 m_FrmReportBook2;

		public FrmReportCancel m_FrmReportCancel;

		public FrmReportCancelSale m_FrmReportCancelSale;

		public FrmReportCoupon m_FrmReportCoupon;

		public FrmReportHousewife m_FrmReportHousewife;

		public FrmReportImcome m_FrmReportImcome;

		public FrmReportImcome2 m_FrmReportImcome2;

		public FrmReportMudjumBack m_FrmReportMudjumBack;

		public FrmReportMudjumRec m_FrmReportMudjumRec;

		public FrmReportPaybooking m_FrmReportPaybooking;

		public FrmReportPower m_FrmReportPower;

		public FrmReportProducts m_FrmReportProducts;

		public FrmReportProductsSale m_FrmReportProductsSale;

		public FrmReportRecPay m_FrmReportRecPay;

		public FrmReportrepair m_FrmReportrepair;

		public FrmReportRR4 m_FrmReportRR4;

		public FrmReportSale1 m_FrmReportSale1;

		public FrmReportSale2 m_FrmReportSale2;

		public FrmReportShift m_FrmReportShift;

		public FrmReportShiftCash m_FrmReportShiftCash;

		public FrmSaleMain2 m_FrmSaleMain2;

		public FrmSearchBook m_FrmSearchBook;

		public FrmSearchCustomers m_FrmSearchCustomers;

		public FrmSearchStock m_FrmSearchStock;

		public FrmSETBranch m_FrmSETBranch;

		public FrmSETCsuType m_FrmSETCsuType;

		public FrmSETCsuTypeMain m_FrmSETCsuTypeMain;

		public FrmSETMyType2 m_FrmSETMyType2;

		public FrmSETMyType2_2 m_FrmSETMyType2_2;

		public FrmSETMyType3 m_FrmSETMyType3;

		public FrmSETProType m_FrmSETProType;

		public FrmSETRoomType m_FrmSETRoomType;

		public FrmSETsale m_FrmSETsale;

		public FrmSETTimeContnue m_FrmSETTimeContnue;

		public FrmSettings m_FrmSettings;

		public FrmSettingsSMS m_FrmSettingsSMS;

		public FrmShowBookNotify m_FrmShowBookNotify;

		public FrmShowPreviewSmartCard m_FrmShowPreviewSmartCard;

		public GForm0 gform0_0;

		public frmTimeTable m_frmTimeTable;

		public FrmUpdate m_FrmUpdate;

		public FrmUseCount m_FrmUseCount;

		public FrmUser m_FrmUser;

		public frmWanting m_frmWanting;

		public GENDB m_GENDB;

		public INV_Note m_INV_Note;

		public login m_login;

		public ReportCleanRoom m_ReportCleanRoom;

		public ReportContnueRoom m_ReportContnueRoom;

		public ReportContnueRoom2 m_ReportContnueRoom2;

		public ReportCustChange m_ReportCustChange;

		public ReportCustDays m_ReportCustDays;

		public ReportCustIn m_ReportCustIn;

		public ReportCustOut m_ReportCustOut;

		public ReportCustOutToday m_ReportCustOutToday;

		public ReportCustOutToday2 m_ReportCustOutToday2;

		public ReportDays m_ReportDays;

		public ReportDebt m_ReportDebt;

		public ReportTax m_ReportTax;

		public Room_Note m_Room_Note;

		public Room_Note_Read m_Room_Note_Read;

		public TwainHandler m_TwainHandler;

		[ThreadStatic]
		private static Hashtable m_FormBeingCreated;

		public AboutBox1 AboutBox1
		{
			[DebuggerNonUserCode]
			get
			{
				m_AboutBox1 = Create__Instance__(m_AboutBox1);
				return m_AboutBox1;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_AboutBox1)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_AboutBox1);
				}
			}
		}

		public AlertCustom AlertCustom
		{
			[DebuggerNonUserCode]
			get
			{
				m_AlertCustom = Create__Instance__(m_AlertCustom);
				return m_AlertCustom;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_AlertCustom)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_AlertCustom);
				}
			}
		}

		public ClickAddmore ClickAddmore
		{
			[DebuggerNonUserCode]
			get
			{
				m_ClickAddmore = Create__Instance__(m_ClickAddmore);
				return m_ClickAddmore;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ClickAddmore)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ClickAddmore);
				}
			}
		}

		public ClickAvliable ClickAvliable
		{
			[DebuggerNonUserCode]
			get
			{
				m_ClickAvliable = Create__Instance__(m_ClickAvliable);
				return m_ClickAvliable;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ClickAvliable)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ClickAvliable);
				}
			}
		}

		public ClickAvliable_book ClickAvliable_book
		{
			[DebuggerNonUserCode]
			get
			{
				m_ClickAvliable_book = Create__Instance__(m_ClickAvliable_book);
				return m_ClickAvliable_book;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ClickAvliable_book)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ClickAvliable_book);
				}
			}
		}

		public ClickBook ClickBook
		{
			[DebuggerNonUserCode]
			get
			{
				m_ClickBook = Create__Instance__(m_ClickBook);
				return m_ClickBook;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ClickBook)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ClickBook);
				}
			}
		}

		public ClickBook_book ClickBook_book
		{
			[DebuggerNonUserCode]
			get
			{
				m_ClickBook_book = Create__Instance__(m_ClickBook_book);
				return m_ClickBook_book;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ClickBook_book)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ClickBook_book);
				}
			}
		}

		public ClickClean ClickClean
		{
			[DebuggerNonUserCode]
			get
			{
				m_ClickClean = Create__Instance__(m_ClickClean);
				return m_ClickClean;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ClickClean)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ClickClean);
				}
			}
		}

		public ClickCleanOK ClickCleanOK
		{
			[DebuggerNonUserCode]
			get
			{
				m_ClickCleanOK = Create__Instance__(m_ClickCleanOK);
				return m_ClickCleanOK;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ClickCleanOK)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ClickCleanOK);
				}
			}
		}

		public ClickManternance ClickManternance
		{
			[DebuggerNonUserCode]
			get
			{
				m_ClickManternance = Create__Instance__(m_ClickManternance);
				return m_ClickManternance;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ClickManternance)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ClickManternance);
				}
			}
		}

		public ClickUSE ClickUSE
		{
			[DebuggerNonUserCode]
			get
			{
				m_ClickUSE = Create__Instance__(m_ClickUSE);
				return m_ClickUSE;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ClickUSE)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ClickUSE);
				}
			}
		}

		public ClickUSE2 ClickUSE2_0
		{
			[DebuggerNonUserCode]
			get
			{
				m_ClickUSE2 = Create__Instance__(m_ClickUSE2);
				return m_ClickUSE2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ClickUSE2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ClickUSE2);
				}
			}
		}

		public ClickUSE3 ClickUSE3_0
		{
			[DebuggerNonUserCode]
			get
			{
				m_ClickUSE3 = Create__Instance__(m_ClickUSE3);
				return m_ClickUSE3;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ClickUSE3)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ClickUSE3);
				}
			}
		}

		public connect_mssql connect_mssql
		{
			[DebuggerNonUserCode]
			get
			{
				m_connect_mssql = Create__Instance__(m_connect_mssql);
				return m_connect_mssql;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_connect_mssql)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_connect_mssql);
				}
			}
		}

		public EMP_Note EMP_Note
		{
			[DebuggerNonUserCode]
			get
			{
				m_EMP_Note = Create__Instance__(m_EMP_Note);
				return m_EMP_Note;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_EMP_Note)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_EMP_Note);
				}
			}
		}

		public EMP_Note_Read EMP_Note_Read
		{
			[DebuggerNonUserCode]
			get
			{
				m_EMP_Note_Read = Create__Instance__(m_EMP_Note_Read);
				return m_EMP_Note_Read;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_EMP_Note_Read)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_EMP_Note_Read);
				}
			}
		}

		public Form1 Form1
		{
			[DebuggerNonUserCode]
			get
			{
				m_Form1 = Create__Instance__(m_Form1);
				return m_Form1;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_Form1)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_Form1);
				}
			}
		}

		public FormEN_DE FormEN_DE_0
		{
			[DebuggerNonUserCode]
			get
			{
				formEN_DE_0 = Create__Instance__(formEN_DE_0);
				return formEN_DE_0;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != formEN_DE_0)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref formEN_DE_0);
				}
			}
		}

		public FormART FormART
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormART = Create__Instance__(m_FormART);
				return m_FormART;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormART)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormART);
				}
			}
		}

		public FormBookingInvoice FormBookingInvoice
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormBookingInvoice = Create__Instance__(m_FormBookingInvoice);
				return m_FormBookingInvoice;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormBookingInvoice)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormBookingInvoice);
				}
			}
		}

		public FormBookRooms FormBookRooms
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormBookRooms = Create__Instance__(m_FormBookRooms);
				return m_FormBookRooms;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormBookRooms)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormBookRooms);
				}
			}
		}

		public FormConfirmOverBill FormConfirmOverBill
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormConfirmOverBill = Create__Instance__(m_FormConfirmOverBill);
				return m_FormConfirmOverBill;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormConfirmOverBill)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormConfirmOverBill);
				}
			}
		}

		public FormConfirmPay FormConfirmPay
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormConfirmPay = Create__Instance__(m_FormConfirmPay);
				return m_FormConfirmPay;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormConfirmPay)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormConfirmPay);
				}
			}
		}

		public FormConfirmRoundBill FormConfirmRoundBill
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormConfirmRoundBill = Create__Instance__(m_FormConfirmRoundBill);
				return m_FormConfirmRoundBill;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormConfirmRoundBill)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormConfirmRoundBill);
				}
			}
		}

		public FormEditPay FormEditPay
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormEditPay = Create__Instance__(m_FormEditPay);
				return m_FormEditPay;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormEditPay)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormEditPay);
				}
			}
		}

		public FormFolio FormFolio
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormFolio = Create__Instance__(m_FormFolio);
				return m_FormFolio;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormFolio)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormFolio);
				}
			}
		}

		public FormLog FormLog
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormLog = Create__Instance__(m_FormLog);
				return m_FormLog;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormLog)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormLog);
				}
			}
		}

		public FormManageOrderCust FormManageOrderCust
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormManageOrderCust = Create__Instance__(m_FormManageOrderCust);
				return m_FormManageOrderCust;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormManageOrderCust)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormManageOrderCust);
				}
			}
		}

		public FormManageOrderCustDown FormManageOrderCustDown
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormManageOrderCustDown = Create__Instance__(m_FormManageOrderCustDown);
				return m_FormManageOrderCustDown;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormManageOrderCustDown)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormManageOrderCustDown);
				}
			}
		}

		public FormPass FormPass
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormPass = Create__Instance__(m_FormPass);
				return m_FormPass;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormPass)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormPass);
				}
			}
		}

		public FormReportAll FormReportAll
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormReportAll = Create__Instance__(m_FormReportAll);
				return m_FormReportAll;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormReportAll)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormReportAll);
				}
			}
		}

		public FormReportAll2 FormReportAll2
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormReportAll2 = Create__Instance__(m_FormReportAll2);
				return m_FormReportAll2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormReportAll2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormReportAll2);
				}
			}
		}

		public FormRoomMain FormRoomMain
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormRoomMain = Create__Instance__(m_FormRoomMain);
				return m_FormRoomMain;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormRoomMain)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormRoomMain);
				}
			}
		}

		public FormRoomMain_ViewBook FormRoomMain_ViewBook
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormRoomMain_ViewBook = Create__Instance__(m_FormRoomMain_ViewBook);
				return m_FormRoomMain_ViewBook;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormRoomMain_ViewBook)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormRoomMain_ViewBook);
				}
			}
		}

		public FormRoomMainClean FormRoomMainClean
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormRoomMainClean = Create__Instance__(m_FormRoomMainClean);
				return m_FormRoomMainClean;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormRoomMainClean)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormRoomMainClean);
				}
			}
		}

		public FormRoomMainKichen FormRoomMainKichen
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormRoomMainKichen = Create__Instance__(m_FormRoomMainKichen);
				return m_FormRoomMainKichen;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormRoomMainKichen)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormRoomMainKichen);
				}
			}
		}

		public FormRoomMainKichen_old FormRoomMainKichen_old
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormRoomMainKichen_old = Create__Instance__(m_FormRoomMainKichen_old);
				return m_FormRoomMainKichen_old;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormRoomMainKichen_old)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormRoomMainKichen_old);
				}
			}
		}

		public FormSearchBooking FormSearchBooking
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSearchBooking = Create__Instance__(m_FormSearchBooking);
				return m_FormSearchBooking;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSearchBooking)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSearchBooking);
				}
			}
		}

		public FormSearchBooking2 FormSearchBooking2
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSearchBooking2 = Create__Instance__(m_FormSearchBooking2);
				return m_FormSearchBooking2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSearchBooking2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSearchBooking2);
				}
			}
		}

		public FormSearchChechIn FormSearchChechIn
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSearchChechIn = Create__Instance__(m_FormSearchChechIn);
				return m_FormSearchChechIn;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSearchChechIn)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSearchChechIn);
				}
			}
		}

		public FormSearchChechInnotOut FormSearchChechInnotOut
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSearchChechInnotOut = Create__Instance__(m_FormSearchChechInnotOut);
				return m_FormSearchChechInnotOut;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSearchChechInnotOut)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSearchChechInnotOut);
				}
			}
		}

		public GForm1 FormSearchChechInVAT
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSearchChechInVAT = Create__Instance__(m_FormSearchChechInVAT);
				return m_FormSearchChechInVAT;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSearchChechInVAT)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSearchChechInVAT);
				}
			}
		}

		public FormSearchCust FormSearchCust
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSearchCust = Create__Instance__(m_FormSearchCust);
				return m_FormSearchCust;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSearchCust)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSearchCust);
				}
			}
		}

		public FormSearchPro FormSearchPro
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSearchPro = Create__Instance__(m_FormSearchPro);
				return m_FormSearchPro;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSearchPro)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSearchPro);
				}
			}
		}

		public FormSearchRooms FormSearchRooms
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSearchRooms = Create__Instance__(m_FormSearchRooms);
				return m_FormSearchRooms;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSearchRooms)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSearchRooms);
				}
			}
		}

		public FormSearchRooms2 FormSearchRooms2
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSearchRooms2 = Create__Instance__(m_FormSearchRooms2);
				return m_FormSearchRooms2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSearchRooms2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSearchRooms2);
				}
			}
		}

		public FormSearchRooms2_old FormSearchRooms2_old
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSearchRooms2_old = Create__Instance__(m_FormSearchRooms2_old);
				return m_FormSearchRooms2_old;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSearchRooms2_old)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSearchRooms2_old);
				}
			}
		}

		public FormSearchRoomsCin FormSearchRoomsCin
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSearchRoomsCin = Create__Instance__(m_FormSearchRoomsCin);
				return m_FormSearchRoomsCin;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSearchRoomsCin)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSearchRoomsCin);
				}
			}
		}

		public FormSearchRoomsCin2 FormSearchRoomsCin2
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSearchRoomsCin2 = Create__Instance__(m_FormSearchRoomsCin2);
				return m_FormSearchRoomsCin2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSearchRoomsCin2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSearchRoomsCin2);
				}
			}
		}

		public FormSelectDB FormSelectDB
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSelectDB = Create__Instance__(m_FormSelectDB);
				return m_FormSelectDB;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSelectDB)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSelectDB);
				}
			}
		}

		public FormSelectDB_old FormSelectDB_old
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSelectDB_old = Create__Instance__(m_FormSelectDB_old);
				return m_FormSelectDB_old;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSelectDB_old)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSelectDB_old);
				}
			}
		}

		public FormSelectRoom FormSelectRoom
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSelectRoom = Create__Instance__(m_FormSelectRoom);
				return m_FormSelectRoom;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSelectRoom)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSelectRoom);
				}
			}
		}

		public FormSETelec FormSETelec_0
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSETelec = Create__Instance__(m_FormSETelec);
				return m_FormSETelec;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSETelec)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSETelec);
				}
			}
		}

		public FormShowDEP FormShowDEP_0
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormShowDEP = Create__Instance__(m_FormShowDEP);
				return m_FormShowDEP;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormShowDEP)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormShowDEP);
				}
			}
		}

		public FormShowDEPBack FormShowDEPBack
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormShowDEPBack = Create__Instance__(m_FormShowDEPBack);
				return m_FormShowDEPBack;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormShowDEPBack)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormShowDEPBack);
				}
			}
		}

		public FormShowSAVEout2 FormShowSAVEout2
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormShowSAVEout2 = Create__Instance__(m_FormShowSAVEout2);
				return m_FormShowSAVEout2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormShowSAVEout2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormShowSAVEout2);
				}
			}
		}

		public FormShowVAT FormShowVAT_0
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormShowVAT = Create__Instance__(m_FormShowVAT);
				return m_FormShowVAT;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormShowVAT)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormShowVAT);
				}
			}
		}

		public FormSMS_DEBT FormSMS_DEBT
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSMS_DEBT = Create__Instance__(m_FormSMS_DEBT);
				return m_FormSMS_DEBT;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSMS_DEBT)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSMS_DEBT);
				}
			}
		}

		public FormSMSLog FormSMSLog_0
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSMSLog = Create__Instance__(m_FormSMSLog);
				return m_FormSMSLog;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSMSLog)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSMSLog);
				}
			}
		}

		public FormSMSSendManual FormSMSSendManual
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormSMSSendManual = Create__Instance__(m_FormSMSSendManual);
				return m_FormSMSSendManual;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormSMSSendManual)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormSMSSendManual);
				}
			}
		}

		public FormUPDATE FormUPDATE_0
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormUPDATE = Create__Instance__(m_FormUPDATE);
				return m_FormUPDATE;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormUPDATE)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormUPDATE);
				}
			}
		}

		public FormUpdateDateRoomAll FormUpdateDateRoomAll
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormUpdateDateRoomAll = Create__Instance__(m_FormUpdateDateRoomAll);
				return m_FormUpdateDateRoomAll;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormUpdateDateRoomAll)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormUpdateDateRoomAll);
				}
			}
		}

		public FormVatOver FormVatOver
		{
			[DebuggerNonUserCode]
			get
			{
				m_FormVatOver = Create__Instance__(m_FormVatOver);
				return m_FormVatOver;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FormVatOver)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FormVatOver);
				}
			}
		}

		public FrmAddBook FrmAddBook
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmAddBook = Create__Instance__(m_FrmAddBook);
				return m_FrmAddBook;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmAddBook)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmAddBook);
				}
			}
		}

		public FrmAddBook2 FrmAddBook2
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmAddBook2 = Create__Instance__(m_FrmAddBook2);
				return m_FrmAddBook2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmAddBook2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmAddBook2);
				}
			}
		}

		public FrmAddBook2copy FrmAddBook2copy
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmAddBook2copy = Create__Instance__(m_FrmAddBook2copy);
				return m_FrmAddBook2copy;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmAddBook2copy)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmAddBook2copy);
				}
			}
		}

		public FrmAddDep FrmAddDep
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmAddDep = Create__Instance__(m_FrmAddDep);
				return m_FrmAddDep;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmAddDep)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmAddDep);
				}
			}
		}

		public FrmAddEditServer FrmAddEditServer
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmAddEditServer = Create__Instance__(m_FrmAddEditServer);
				return m_FrmAddEditServer;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmAddEditServer)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmAddEditServer);
				}
			}
		}

		public FrmAddInvoiceSale FrmAddInvoiceSale
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmAddInvoiceSale = Create__Instance__(m_FrmAddInvoiceSale);
				return m_FrmAddInvoiceSale;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmAddInvoiceSale)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmAddInvoiceSale);
				}
			}
		}

		public FrmAddPay FrmAddPay
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmAddPay = Create__Instance__(m_FrmAddPay);
				return m_FrmAddPay;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmAddPay)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmAddPay);
				}
			}
		}

		public FrmAddReg FrmAddReg
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmAddReg = Create__Instance__(m_FrmAddReg);
				return m_FrmAddReg;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmAddReg)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmAddReg);
				}
			}
		}

		public FrmAddSale FrmAddSale
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmAddSale = Create__Instance__(m_FrmAddSale);
				return m_FrmAddSale;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmAddSale)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmAddSale);
				}
			}
		}

		public FrmAddSale2 FrmAddSale2
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmAddSale2 = Create__Instance__(m_FrmAddSale2);
				return m_FrmAddSale2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmAddSale2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmAddSale2);
				}
			}
		}

		public FrmAddSale2_Credit FrmAddSale2_Credit
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmAddSale2_Credit = Create__Instance__(m_FrmAddSale2_Credit);
				return m_FrmAddSale2_Credit;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmAddSale2_Credit)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmAddSale2_Credit);
				}
			}
		}

		public FrmAddSaveImage FrmAddSaveImage
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmAddSaveImage = Create__Instance__(m_FrmAddSaveImage);
				return m_FrmAddSaveImage;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmAddSaveImage)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmAddSaveImage);
				}
			}
		}

		public FrmBookMain FrmBookMain
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmBookMain = Create__Instance__(m_FrmBookMain);
				return m_FrmBookMain;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmBookMain)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmBookMain);
				}
			}
		}

		public FrmBookMain2 FrmBookMain2
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmBookMain2 = Create__Instance__(m_FrmBookMain2);
				return m_FrmBookMain2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmBookMain2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmBookMain2);
				}
			}
		}

		public FrmBookRooms FrmBookRooms
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmBookRooms = Create__Instance__(m_FrmBookRooms);
				return m_FrmBookRooms;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmBookRooms)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmBookRooms);
				}
			}
		}

		public frmCapture frmCapture
		{
			[DebuggerNonUserCode]
			get
			{
				m_frmCapture = Create__Instance__(m_frmCapture);
				return m_frmCapture;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_frmCapture)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_frmCapture);
				}
			}
		}

		public FrmCheckIn FrmCheckIn
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmCheckIn = Create__Instance__(m_FrmCheckIn);
				return m_FrmCheckIn;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmCheckIn)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmCheckIn);
				}
			}
		}

		public FrmCheckIn_EditOnly FrmCheckIn_EditOnly
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmCheckIn_EditOnly = Create__Instance__(m_FrmCheckIn_EditOnly);
				return m_FrmCheckIn_EditOnly;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmCheckIn_EditOnly)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmCheckIn_EditOnly);
				}
			}
		}

		public FrmCheckOut FrmCheckOut
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmCheckOut = Create__Instance__(m_FrmCheckOut);
				return m_FrmCheckOut;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmCheckOut)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmCheckOut);
				}
			}
		}

		public FrmCuponMain FrmCuponMain
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmCuponMain = Create__Instance__(m_FrmCuponMain);
				return m_FrmCuponMain;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmCuponMain)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmCuponMain);
				}
			}
		}

		public FrmCustomers FrmCustomers
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmCustomers = Create__Instance__(m_FrmCustomers);
				return m_FrmCustomers;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmCustomers)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmCustomers);
				}
			}
		}

		public FrmDepositMain FrmDepositMain
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmDepositMain = Create__Instance__(m_FrmDepositMain);
				return m_FrmDepositMain;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmDepositMain)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmDepositMain);
				}
			}
		}

		public FrmDueBill FrmDueBill
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmDueBill = Create__Instance__(m_FrmDueBill);
				return m_FrmDueBill;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmDueBill)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmDueBill);
				}
			}
		}

		public FrmEditDate FrmEditDate
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmEditDate = Create__Instance__(m_FrmEditDate);
				return m_FrmEditDate;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmEditDate)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmEditDate);
				}
			}
		}

		public FrmInOutMain FrmInOutMain
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmInOutMain = Create__Instance__(m_FrmInOutMain);
				return m_FrmInOutMain;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmInOutMain)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmInOutMain);
				}
			}
		}

		public frmMain1 frmMain1
		{
			[DebuggerNonUserCode]
			get
			{
				m_frmMain1 = Create__Instance__(m_frmMain1);
				return m_frmMain1;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_frmMain1)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_frmMain1);
				}
			}
		}

		public FrmManageCustomers FrmManageCustomers
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmManageCustomers = Create__Instance__(m_FrmManageCustomers);
				return m_FrmManageCustomers;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmManageCustomers)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmManageCustomers);
				}
			}
		}

		public FrmManageCustomersNew FrmManageCustomersNew
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmManageCustomersNew = Create__Instance__(m_FrmManageCustomersNew);
				return m_FrmManageCustomersNew;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmManageCustomersNew)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmManageCustomersNew);
				}
			}
		}

		public FrmManageCustomersSearch FrmManageCustomersSearch
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmManageCustomersSearch = Create__Instance__(m_FrmManageCustomersSearch);
				return m_FrmManageCustomersSearch;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmManageCustomersSearch)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmManageCustomersSearch);
				}
			}
		}

		public FrmManageProduct FrmManageProduct
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmManageProduct = Create__Instance__(m_FrmManageProduct);
				return m_FrmManageProduct;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmManageProduct)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmManageProduct);
				}
			}
		}

		public FrmManageRoom FrmManageRoom
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmManageRoom = Create__Instance__(m_FrmManageRoom);
				return m_FrmManageRoom;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmManageRoom)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmManageRoom);
				}
			}
		}

		public FrmPayAdd FrmPayAdd
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmPayAdd = Create__Instance__(m_FrmPayAdd);
				return m_FrmPayAdd;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmPayAdd)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmPayAdd);
				}
			}
		}

		public FrmPayAddDebt FrmPayAddDebt
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmPayAddDebt = Create__Instance__(m_FrmPayAddDebt);
				return m_FrmPayAddDebt;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmPayAddDebt)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmPayAddDebt);
				}
			}
		}

		public FrmPayAddPro FrmPayAddPro
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmPayAddPro = Create__Instance__(m_FrmPayAddPro);
				return m_FrmPayAddPro;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmPayAddPro)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmPayAddPro);
				}
			}
		}

		public FrmPayDebt FrmPayDebt
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmPayDebt = Create__Instance__(m_FrmPayDebt);
				return m_FrmPayDebt;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmPayDebt)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmPayDebt);
				}
			}
		}

		public FrmPayDebt2 FrmPayDebt2
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmPayDebt2 = Create__Instance__(m_FrmPayDebt2);
				return m_FrmPayDebt2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmPayDebt2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmPayDebt2);
				}
			}
		}

		public FrmPayMain FrmPayMain
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmPayMain = Create__Instance__(m_FrmPayMain);
				return m_FrmPayMain;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmPayMain)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmPayMain);
				}
			}
		}

		public FrmPermission FrmPermission
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmPermission = Create__Instance__(m_FrmPermission);
				return m_FrmPermission;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmPermission)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmPermission);
				}
			}
		}

		public FrmPriceHistory FrmPriceHistory
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmPriceHistory = Create__Instance__(m_FrmPriceHistory);
				return m_FrmPriceHistory;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmPriceHistory)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmPriceHistory);
				}
			}
		}

		public FrmPrint FrmPrint
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmPrint = Create__Instance__(m_FrmPrint);
				return m_FrmPrint;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmPrint)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmPrint);
				}
			}
		}

		public FrmReceiptInvoice FrmReceiptInvoice
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReceiptInvoice = Create__Instance__(m_FrmReceiptInvoice);
				return m_FrmReceiptInvoice;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReceiptInvoice)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReceiptInvoice);
				}
			}
		}

		public FrmReceiptMain FrmReceiptMain
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReceiptMain = Create__Instance__(m_FrmReceiptMain);
				return m_FrmReceiptMain;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReceiptMain)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReceiptMain);
				}
			}
		}

		public FrmReceiptMain_invoice FrmReceiptMain_invoice
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReceiptMain_invoice = Create__Instance__(m_FrmReceiptMain_invoice);
				return m_FrmReceiptMain_invoice;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReceiptMain_invoice)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReceiptMain_invoice);
				}
			}
		}

		public frmReg frmReg
		{
			[DebuggerNonUserCode]
			get
			{
				m_frmReg = Create__Instance__(m_frmReg);
				return m_frmReg;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_frmReg)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_frmReg);
				}
			}
		}

		public FrmRegMain FrmRegMain
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmRegMain = Create__Instance__(m_FrmRegMain);
				return m_FrmRegMain;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmRegMain)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmRegMain);
				}
			}
		}

		public FrmReportBook FrmReportBook
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportBook = Create__Instance__(m_FrmReportBook);
				return m_FrmReportBook;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportBook)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportBook);
				}
			}
		}

		public FrmReportBook2 FrmReportBook2
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportBook2 = Create__Instance__(m_FrmReportBook2);
				return m_FrmReportBook2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportBook2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportBook2);
				}
			}
		}

		public FrmReportCancel FrmReportCancel
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportCancel = Create__Instance__(m_FrmReportCancel);
				return m_FrmReportCancel;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportCancel)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportCancel);
				}
			}
		}

		public FrmReportCancelSale FrmReportCancelSale
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportCancelSale = Create__Instance__(m_FrmReportCancelSale);
				return m_FrmReportCancelSale;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportCancelSale)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportCancelSale);
				}
			}
		}

		public FrmReportCoupon FrmReportCoupon
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportCoupon = Create__Instance__(m_FrmReportCoupon);
				return m_FrmReportCoupon;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportCoupon)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportCoupon);
				}
			}
		}

		public FrmReportHousewife FrmReportHousewife
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportHousewife = Create__Instance__(m_FrmReportHousewife);
				return m_FrmReportHousewife;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportHousewife)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportHousewife);
				}
			}
		}

		public FrmReportImcome FrmReportImcome
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportImcome = Create__Instance__(m_FrmReportImcome);
				return m_FrmReportImcome;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportImcome)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportImcome);
				}
			}
		}

		public FrmReportImcome2 FrmReportImcome2
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportImcome2 = Create__Instance__(m_FrmReportImcome2);
				return m_FrmReportImcome2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportImcome2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportImcome2);
				}
			}
		}

		public FrmReportMudjumBack FrmReportMudjumBack
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportMudjumBack = Create__Instance__(m_FrmReportMudjumBack);
				return m_FrmReportMudjumBack;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportMudjumBack)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportMudjumBack);
				}
			}
		}

		public FrmReportMudjumRec FrmReportMudjumRec
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportMudjumRec = Create__Instance__(m_FrmReportMudjumRec);
				return m_FrmReportMudjumRec;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportMudjumRec)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportMudjumRec);
				}
			}
		}

		public FrmReportPaybooking FrmReportPaybooking
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportPaybooking = Create__Instance__(m_FrmReportPaybooking);
				return m_FrmReportPaybooking;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportPaybooking)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportPaybooking);
				}
			}
		}

		public FrmReportPower FrmReportPower
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportPower = Create__Instance__(m_FrmReportPower);
				return m_FrmReportPower;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportPower)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportPower);
				}
			}
		}

		public FrmReportProducts FrmReportProducts
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportProducts = Create__Instance__(m_FrmReportProducts);
				return m_FrmReportProducts;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportProducts)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportProducts);
				}
			}
		}

		public FrmReportProductsSale FrmReportProductsSale
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportProductsSale = Create__Instance__(m_FrmReportProductsSale);
				return m_FrmReportProductsSale;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportProductsSale)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportProductsSale);
				}
			}
		}

		public FrmReportRecPay FrmReportRecPay
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportRecPay = Create__Instance__(m_FrmReportRecPay);
				return m_FrmReportRecPay;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportRecPay)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportRecPay);
				}
			}
		}

		public FrmReportrepair FrmReportrepair
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportrepair = Create__Instance__(m_FrmReportrepair);
				return m_FrmReportrepair;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportrepair)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportrepair);
				}
			}
		}

		public FrmReportRR4 FrmReportRR4
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportRR4 = Create__Instance__(m_FrmReportRR4);
				return m_FrmReportRR4;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportRR4)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportRR4);
				}
			}
		}

		public FrmReportSale1 FrmReportSale1
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportSale1 = Create__Instance__(m_FrmReportSale1);
				return m_FrmReportSale1;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportSale1)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportSale1);
				}
			}
		}

		public FrmReportSale2 FrmReportSale2
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportSale2 = Create__Instance__(m_FrmReportSale2);
				return m_FrmReportSale2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportSale2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportSale2);
				}
			}
		}

		public FrmReportShift FrmReportShift
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportShift = Create__Instance__(m_FrmReportShift);
				return m_FrmReportShift;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportShift)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportShift);
				}
			}
		}

		public FrmReportShiftCash FrmReportShiftCash
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmReportShiftCash = Create__Instance__(m_FrmReportShiftCash);
				return m_FrmReportShiftCash;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmReportShiftCash)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmReportShiftCash);
				}
			}
		}

		public FrmSaleMain2 FrmSaleMain2
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmSaleMain2 = Create__Instance__(m_FrmSaleMain2);
				return m_FrmSaleMain2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmSaleMain2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmSaleMain2);
				}
			}
		}

		public FrmSearchBook FrmSearchBook
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmSearchBook = Create__Instance__(m_FrmSearchBook);
				return m_FrmSearchBook;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmSearchBook)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmSearchBook);
				}
			}
		}

		public FrmSearchCustomers FrmSearchCustomers
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmSearchCustomers = Create__Instance__(m_FrmSearchCustomers);
				return m_FrmSearchCustomers;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmSearchCustomers)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmSearchCustomers);
				}
			}
		}

		public FrmSearchStock FrmSearchStock
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmSearchStock = Create__Instance__(m_FrmSearchStock);
				return m_FrmSearchStock;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmSearchStock)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmSearchStock);
				}
			}
		}

		public FrmSETBranch FrmSETBranch
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmSETBranch = Create__Instance__(m_FrmSETBranch);
				return m_FrmSETBranch;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmSETBranch)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmSETBranch);
				}
			}
		}

		public FrmSETCsuType FrmSETCsuType
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmSETCsuType = Create__Instance__(m_FrmSETCsuType);
				return m_FrmSETCsuType;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmSETCsuType)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmSETCsuType);
				}
			}
		}

		public FrmSETCsuTypeMain FrmSETCsuTypeMain
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmSETCsuTypeMain = Create__Instance__(m_FrmSETCsuTypeMain);
				return m_FrmSETCsuTypeMain;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmSETCsuTypeMain)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmSETCsuTypeMain);
				}
			}
		}

		public FrmSETMyType2 FrmSETMyType2
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmSETMyType2 = Create__Instance__(m_FrmSETMyType2);
				return m_FrmSETMyType2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmSETMyType2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmSETMyType2);
				}
			}
		}

		public FrmSETMyType2_2 FrmSETMyType2_2
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmSETMyType2_2 = Create__Instance__(m_FrmSETMyType2_2);
				return m_FrmSETMyType2_2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmSETMyType2_2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmSETMyType2_2);
				}
			}
		}

		public FrmSETMyType3 FrmSETMyType3
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmSETMyType3 = Create__Instance__(m_FrmSETMyType3);
				return m_FrmSETMyType3;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmSETMyType3)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmSETMyType3);
				}
			}
		}

		public FrmSETProType FrmSETProType
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmSETProType = Create__Instance__(m_FrmSETProType);
				return m_FrmSETProType;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmSETProType)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmSETProType);
				}
			}
		}

		public FrmSETRoomType FrmSETRoomType
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmSETRoomType = Create__Instance__(m_FrmSETRoomType);
				return m_FrmSETRoomType;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmSETRoomType)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmSETRoomType);
				}
			}
		}

		public FrmSETsale FrmSETsale_0
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmSETsale = Create__Instance__(m_FrmSETsale);
				return m_FrmSETsale;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmSETsale)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmSETsale);
				}
			}
		}

		public FrmSETTimeContnue FrmSETTimeContnue
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmSETTimeContnue = Create__Instance__(m_FrmSETTimeContnue);
				return m_FrmSETTimeContnue;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmSETTimeContnue)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmSETTimeContnue);
				}
			}
		}

		public FrmSettings FrmSettings
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmSettings = Create__Instance__(m_FrmSettings);
				return m_FrmSettings;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmSettings)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmSettings);
				}
			}
		}

		public FrmSettingsSMS FrmSettingsSMS
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmSettingsSMS = Create__Instance__(m_FrmSettingsSMS);
				return m_FrmSettingsSMS;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmSettingsSMS)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmSettingsSMS);
				}
			}
		}

		public FrmShowBookNotify FrmShowBookNotify
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmShowBookNotify = Create__Instance__(m_FrmShowBookNotify);
				return m_FrmShowBookNotify;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmShowBookNotify)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmShowBookNotify);
				}
			}
		}

		public FrmShowPreviewSmartCard FrmShowPreviewSmartCard
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmShowPreviewSmartCard = Create__Instance__(m_FrmShowPreviewSmartCard);
				return m_FrmShowPreviewSmartCard;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmShowPreviewSmartCard)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmShowPreviewSmartCard);
				}
			}
		}

		public GForm0 GForm0_0
		{
			[DebuggerNonUserCode]
			get
			{
				gform0_0 = Create__Instance__(gform0_0);
				return gform0_0;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != gform0_0)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref gform0_0);
				}
			}
		}

		public frmTimeTable frmTimeTable
		{
			[DebuggerNonUserCode]
			get
			{
				m_frmTimeTable = Create__Instance__(m_frmTimeTable);
				return m_frmTimeTable;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_frmTimeTable)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_frmTimeTable);
				}
			}
		}

		public FrmUpdate FrmUpdate
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmUpdate = Create__Instance__(m_FrmUpdate);
				return m_FrmUpdate;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmUpdate)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmUpdate);
				}
			}
		}

		public FrmUseCount FrmUseCount
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmUseCount = Create__Instance__(m_FrmUseCount);
				return m_FrmUseCount;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmUseCount)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmUseCount);
				}
			}
		}

		public FrmUser FrmUser
		{
			[DebuggerNonUserCode]
			get
			{
				m_FrmUser = Create__Instance__(m_FrmUser);
				return m_FrmUser;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_FrmUser)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_FrmUser);
				}
			}
		}

		public frmWanting frmWanting
		{
			[DebuggerNonUserCode]
			get
			{
				m_frmWanting = Create__Instance__(m_frmWanting);
				return m_frmWanting;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_frmWanting)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_frmWanting);
				}
			}
		}

		public GENDB GENDB
		{
			[DebuggerNonUserCode]
			get
			{
				m_GENDB = Create__Instance__(m_GENDB);
				return m_GENDB;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_GENDB)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_GENDB);
				}
			}
		}

		public INV_Note INV_Note
		{
			[DebuggerNonUserCode]
			get
			{
				m_INV_Note = Create__Instance__(m_INV_Note);
				return m_INV_Note;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_INV_Note)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_INV_Note);
				}
			}
		}

		public login login
		{
			[DebuggerNonUserCode]
			get
			{
				m_login = Create__Instance__(m_login);
				return m_login;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_login)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_login);
				}
			}
		}

		public ReportCleanRoom ReportCleanRoom
		{
			[DebuggerNonUserCode]
			get
			{
				m_ReportCleanRoom = Create__Instance__(m_ReportCleanRoom);
				return m_ReportCleanRoom;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ReportCleanRoom)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ReportCleanRoom);
				}
			}
		}

		public ReportContnueRoom ReportContnueRoom
		{
			[DebuggerNonUserCode]
			get
			{
				m_ReportContnueRoom = Create__Instance__(m_ReportContnueRoom);
				return m_ReportContnueRoom;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ReportContnueRoom)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ReportContnueRoom);
				}
			}
		}

		public ReportContnueRoom2 ReportContnueRoom2
		{
			[DebuggerNonUserCode]
			get
			{
				m_ReportContnueRoom2 = Create__Instance__(m_ReportContnueRoom2);
				return m_ReportContnueRoom2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ReportContnueRoom2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ReportContnueRoom2);
				}
			}
		}

		public ReportCustChange ReportCustChange
		{
			[DebuggerNonUserCode]
			get
			{
				m_ReportCustChange = Create__Instance__(m_ReportCustChange);
				return m_ReportCustChange;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ReportCustChange)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ReportCustChange);
				}
			}
		}

		public ReportCustDays ReportCustDays
		{
			[DebuggerNonUserCode]
			get
			{
				m_ReportCustDays = Create__Instance__(m_ReportCustDays);
				return m_ReportCustDays;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ReportCustDays)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ReportCustDays);
				}
			}
		}

		public ReportCustIn ReportCustIn
		{
			[DebuggerNonUserCode]
			get
			{
				m_ReportCustIn = Create__Instance__(m_ReportCustIn);
				return m_ReportCustIn;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ReportCustIn)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ReportCustIn);
				}
			}
		}

		public ReportCustOut ReportCustOut
		{
			[DebuggerNonUserCode]
			get
			{
				m_ReportCustOut = Create__Instance__(m_ReportCustOut);
				return m_ReportCustOut;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ReportCustOut)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ReportCustOut);
				}
			}
		}

		public ReportCustOutToday ReportCustOutToday
		{
			[DebuggerNonUserCode]
			get
			{
				m_ReportCustOutToday = Create__Instance__(m_ReportCustOutToday);
				return m_ReportCustOutToday;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ReportCustOutToday)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ReportCustOutToday);
				}
			}
		}

		public ReportCustOutToday2 ReportCustOutToday2
		{
			[DebuggerNonUserCode]
			get
			{
				m_ReportCustOutToday2 = Create__Instance__(m_ReportCustOutToday2);
				return m_ReportCustOutToday2;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ReportCustOutToday2)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ReportCustOutToday2);
				}
			}
		}

		public ReportDays ReportDays
		{
			[DebuggerNonUserCode]
			get
			{
				m_ReportDays = Create__Instance__(m_ReportDays);
				return m_ReportDays;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ReportDays)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ReportDays);
				}
			}
		}

		public ReportDebt ReportDebt
		{
			[DebuggerNonUserCode]
			get
			{
				m_ReportDebt = Create__Instance__(m_ReportDebt);
				return m_ReportDebt;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ReportDebt)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ReportDebt);
				}
			}
		}

		public ReportTax ReportTax
		{
			[DebuggerNonUserCode]
			get
			{
				m_ReportTax = Create__Instance__(m_ReportTax);
				return m_ReportTax;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_ReportTax)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_ReportTax);
				}
			}
		}

		public Room_Note Room_Note
		{
			[DebuggerNonUserCode]
			get
			{
				m_Room_Note = Create__Instance__(m_Room_Note);
				return m_Room_Note;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_Room_Note)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_Room_Note);
				}
			}
		}

		public Room_Note_Read Room_Note_Read
		{
			[DebuggerNonUserCode]
			get
			{
				m_Room_Note_Read = Create__Instance__(m_Room_Note_Read);
				return m_Room_Note_Read;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_Room_Note_Read)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_Room_Note_Read);
				}
			}
		}

		public TwainHandler TwainHandler
		{
			[DebuggerNonUserCode]
			get
			{
				m_TwainHandler = Create__Instance__(m_TwainHandler);
				return m_TwainHandler;
			}
			[DebuggerNonUserCode]
			set
			{
				if (value != m_TwainHandler)
				{
					if (value != null)
					{
						throw new ArgumentException("Property can only be set to Nothing");
					}
					Dispose__Instance__(ref m_TwainHandler);
				}
			}
		}

		[DebuggerHidden]
		private static T Create__Instance__<T>(T Instance) where T : Form, new()
		{
			if (Instance == null || (Instance.IsDisposed ? true : false))
			{
				if (m_FormBeingCreated != null)
				{
					if (m_FormBeingCreated.ContainsKey(typeof(T)))
					{
						throw new InvalidOperationException(Utils.GetResourceString("WinForms_RecursiveFormCreate"));
					}
				}
				else
				{
					m_FormBeingCreated = new Hashtable();
				}
				m_FormBeingCreated.Add(typeof(T), null);
				try
				{
					return new T();
				}
				catch (TargetInvocationException ex) when (((Func<bool>)delegate
				{
					// Could not convert BlockContainer to single expression
					ProjectData.SetProjectError(ex);
					return ex.InnerException != null;
				}).Invoke())
				{
					string resourceString = Utils.GetResourceString("WinForms_SeeInnerException", ex.InnerException.Message);
					throw new InvalidOperationException(resourceString, ex.InnerException);
				}
				finally
				{
					m_FormBeingCreated.Remove(typeof(T));
				}
			}
			return Instance;
		}

		[DebuggerHidden]
		private void Dispose__Instance__<T>(ref T instance) where T : Form
		{
			instance.Dispose();
			instance = null;
		}

		[EditorBrowsable(EditorBrowsableState.Never)]
		[DebuggerHidden]
		public MyForms()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		}

		[EditorBrowsable(EditorBrowsableState.Never)]
		public override bool Equals(object o)
		{
			return base.Equals(RuntimeHelpers.GetObjectValue(o));
		}

		[EditorBrowsable(EditorBrowsableState.Never)]
		public override int GetHashCode()
		{
			return base.GetHashCode();
		}

		[EditorBrowsable(EditorBrowsableState.Never)]
		internal new Type GetType()
		{
			return typeof(MyForms);
		}

		[EditorBrowsable(EditorBrowsableState.Never)]
		public override string ToString()
		{
			return base.ToString();
		}
	}

	[MyGroupCollection("System.Web.Services.Protocols.SoapHttpClientProtocol", "Create__Instance__", "Dispose__Instance__", "")]
	[EditorBrowsable(EditorBrowsableState.Never)]
	internal sealed class MyWebServices
	{
		[EditorBrowsable(EditorBrowsableState.Never)]
		[DebuggerHidden]
		public override bool Equals(object o)
		{
			return base.Equals(RuntimeHelpers.GetObjectValue(o));
		}

		[DebuggerHidden]
		[EditorBrowsable(EditorBrowsableState.Never)]
		public override int GetHashCode()
		{
			return base.GetHashCode();
		}

		[DebuggerHidden]
		[EditorBrowsable(EditorBrowsableState.Never)]
		internal new Type GetType()
		{
			return typeof(MyWebServices);
		}

		[EditorBrowsable(EditorBrowsableState.Never)]
		[DebuggerHidden]
		public override string ToString()
		{
			return base.ToString();
		}

		[DebuggerHidden]
		private static T Create__Instance__<T>(T instance) where T : new()
		{
			if (instance == null)
			{
				return new T();
			}
			return instance;
		}

		[DebuggerHidden]
		private void Dispose__Instance__<T>(ref T instance)
		{
			instance = default(T);
		}

		[DebuggerHidden]
		[EditorBrowsable(EditorBrowsableState.Never)]
		public MyWebServices()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		}
	}

	[EditorBrowsable(EditorBrowsableState.Never)]
	[ComVisible(false)]
	internal sealed class ThreadSafeObjectProvider<T> where T : new()
	{
		[ThreadStatic]
		[CompilerGenerated]
		private static T m_ThreadStaticValue;

		internal T GetInstance
		{
			[DebuggerHidden]
			get
			{
				if (m_ThreadStaticValue == null)
				{
					m_ThreadStaticValue = new T();
				}
				return m_ThreadStaticValue;
			}
		}

		[EditorBrowsable(EditorBrowsableState.Never)]
		[DebuggerHidden]
		public ThreadSafeObjectProvider()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		}
	}

	private static readonly ThreadSafeObjectProvider<MyComputer> m_ComputerObjectProvider;

	private static readonly ThreadSafeObjectProvider<MyApplication> m_AppObjectProvider;

	private static readonly ThreadSafeObjectProvider<User> m_UserObjectProvider;

	private static ThreadSafeObjectProvider<MyForms> m_MyFormsObjectProvider;

	private static readonly ThreadSafeObjectProvider<MyWebServices> m_MyWebServicesObjectProvider;

	[HelpKeyword("My.Computer")]
	internal static MyComputer Computer
	{
		[DebuggerHidden]
		get
		{
			return m_ComputerObjectProvider.GetInstance;
		}
	}

	[HelpKeyword("My.Application")]
	internal static MyApplication Application
	{
		[DebuggerHidden]
		get
		{
			return m_AppObjectProvider.GetInstance;
		}
	}

	[HelpKeyword("My.User")]
	internal static User User
	{
		[DebuggerHidden]
		get
		{
			return m_UserObjectProvider.GetInstance;
		}
	}

	[HelpKeyword("My.Forms")]
	internal static MyForms Forms
	{
		[DebuggerHidden]
		get
		{
			return m_MyFormsObjectProvider.GetInstance;
		}
	}

	[HelpKeyword("My.WebServices")]
	internal static MyWebServices WebServices
	{
		[DebuggerHidden]
		get
		{
			return m_MyWebServicesObjectProvider.GetInstance;
		}
	}

	[DebuggerNonUserCode]
	static MyProject()
	{
		Class2.LH6iGfYz9j3MJ();
		m_ComputerObjectProvider = new ThreadSafeObjectProvider<MyComputer>();
		m_AppObjectProvider = new ThreadSafeObjectProvider<MyApplication>();
		m_UserObjectProvider = new ThreadSafeObjectProvider<User>();
		m_MyFormsObjectProvider = new ThreadSafeObjectProvider<MyForms>();
		m_MyWebServicesObjectProvider = new ThreadSafeObjectProvider<MyWebServices>();
	}
}
