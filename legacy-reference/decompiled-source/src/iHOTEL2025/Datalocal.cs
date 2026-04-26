using System;
using System.CodeDom.Compiler;
using System.Collections;
using System.Collections.Generic;
using System.ComponentModel;
using System.ComponentModel.Design;
using System.Data;
using System.Diagnostics;
using System.IO;
using System.Runtime.CompilerServices;
using System.Runtime.Serialization;
using System.Xml;
using System.Xml.Schema;
using System.Xml.Serialization;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[Serializable]
[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
[DesignerCategory("code")]
[ToolboxItem(true)]
[XmlRoot("Datalocal")]
[HelpKeyword("vs.data.DataSet")]
[XmlSchemaProvider("GetTypedDataSetSchema")]
public class Datalocal : DataSet
{
	public delegate void ReportBillCashRowChangeEventHandler(object sender, ReportBillCashRowChangeEvent e);

	public delegate void Bill_HRowChangeEventHandler(object sender, Bill_HRowChangeEvent e);

	public delegate void ReportRegRowChangeEventHandler(object sender, ReportRegRowChangeEvent e);

	public delegate void ReportDepRowChangeEventHandler(object sender, ReportDepRowChangeEvent e);

	public delegate void ConfigRowChangeEventHandler(object sender, ConfigRowChangeEvent e);

	public delegate void ReportPicRowChangeEventHandler(object sender, ReportPicRowChangeEvent e);

	public delegate void ReportDaysRowChangeEventHandler(object sender, ReportDaysRowChangeEvent e);

	public delegate void ReportCustINRowChangeEventHandler(object sender, ReportCustINRowChangeEvent e);

	public delegate void ReportVatRowChangeEventHandler(object sender, ReportVatRowChangeEvent e);

	public delegate void ReportShiftRowChangeEventHandler(object sender, ReportShiftRowChangeEvent e);

	public delegate void ReportCuponRowChangeEventHandler(object sender, ReportCuponRowChangeEvent e);

	public delegate void TableShiftCashRowChangeEventHandler(object sender, TableShiftCashRowChangeEvent e);

	public delegate void ReportBillCreditRowChangeEventHandler(object sender, ReportBillCreditRowChangeEvent e);

	public delegate void Report_Room_allRowChangeEventHandler(object sender, Report_Room_allRowChangeEvent e);

	public delegate void Report_Debt_INVRowChangeEventHandler(object sender, Report_Debt_INVRowChangeEvent e);

	public delegate void ReportFolio1RowChangeEventHandler(object sender, ReportFolio1RowChangeEvent e);

	public delegate void ReportFolio1_2RowChangeEventHandler(object sender, ReportFolio1_2RowChangeEvent e);

	public delegate void ReportFolio2RowChangeEventHandler(object sender, ReportFolio2RowChangeEvent e);

	public delegate void ReportSaleRowChangeEventHandler(object sender, ReportSaleRowChangeEvent e);

	public delegate void ReportBookingRowChangeEventHandler(object sender, ReportBookingRowChangeEvent e);

	public delegate void ReportBookingINVRowChangeEventHandler(object sender, ReportBookingINVRowChangeEvent e);

	public delegate void GDelegate0(object sender, GEventArgs0 e);

	[Serializable]
	[XmlSchemaProvider("GetTypedTableSchema")]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportBillCashDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn dataColumn_3;

		private DataColumn dataColumn_4;

		private DataColumn dataColumn_5;

		private DataColumn dataColumn_6;

		private DataColumn dataColumn_7;

		private DataColumn dataColumn_8;

		private DataColumn dataColumn_9;

		private DataColumn dataColumn_10;

		private DataColumn dataColumn_11;

		private DataColumn dataColumn_12;

		private DataColumn dataColumn_13;

		private DataColumn dataColumn_14;

		private DataColumn dataColumn_15;

		private DataColumn dataColumn_16;

		private DataColumn dataColumn_17;

		private DataColumn dataColumn_18;

		private DataColumn columnbarcode;

		private DataColumn dataColumn_19;

		private DataColumn dataColumn_20;

		private DataColumn dataColumn_21;

		private DataColumn dataColumn_22;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_4 => dataColumn_4;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_5 => dataColumn_5;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_6 => dataColumn_6;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_7 => dataColumn_7;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_8 => dataColumn_8;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_9 => dataColumn_9;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_10 => dataColumn_10;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_11 => dataColumn_11;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_12 => dataColumn_12;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_13 => dataColumn_13;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_14 => dataColumn_14;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_15 => dataColumn_15;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_16 => dataColumn_16;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_17 => dataColumn_17;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_18 => dataColumn_18;

		[DebuggerNonUserCode]
		public DataColumn barcodeColumn => columnbarcode;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_19 => dataColumn_19;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_20 => dataColumn_20;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_21 => dataColumn_21;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_22 => dataColumn_22;

		[DebuggerNonUserCode]
		[Browsable(false)]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public ReportBillCashRow this[int index] => (ReportBillCashRow)Rows[index];

		[method: DebuggerNonUserCode]
		public event ReportBillCashRowChangeEventHandler ReportBillCashRowChanging;

		[method: DebuggerNonUserCode]
		public event ReportBillCashRowChangeEventHandler ReportBillCashRowChanged;

		[method: DebuggerNonUserCode]
		public event ReportBillCashRowChangeEventHandler ReportBillCashRowDeleting;

		[method: DebuggerNonUserCode]
		public event ReportBillCashRowChangeEventHandler ReportBillCashRowDeleted;

		[DebuggerNonUserCode]
		static ReportBillCashDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public ReportBillCashDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "ReportBillCash";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal ReportBillCashDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected ReportBillCashDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReportBillCashRow(ReportBillCashRow row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public ReportBillCashRow AddReportBillCashRow(string string_0, string string_1, string string_2, string string_3, string string_4, string string_5, string string_6, string string_7, string string_8, string string_9, string string_10, string string_11, string string_12, string string_13, string string_14, string string_15, string string_16, string string_17, string string_18, byte[] barcode, string string_19, string string_20, string string_21, string string_22)
		{
			ReportBillCashRow reportBillCashRow = (ReportBillCashRow)NewRow();
			object[] itemArray = new object[24]
			{
				string_0, string_1, string_2, string_3, string_4, string_5, string_6, string_7, string_8, string_9,
				string_10, string_11, string_12, string_13, string_14, string_15, string_16, string_17, string_18, barcode,
				string_19, string_20, string_21, string_22
			};
			reportBillCashRow.ItemArray = itemArray;
			Rows.Add(reportBillCashRow);
			return reportBillCashRow;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			ReportBillCashDataTable reportBillCashDataTable = (ReportBillCashDataTable)base.Clone();
			reportBillCashDataTable.InitVars();
			return reportBillCashDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new ReportBillCashDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			dataColumn_0 = base.Columns["กร\u0e38\u0e4aป"];
			dataColumn_1 = base.Columns["ว\u0e31นท\u0e35\u0e48"];
			dataColumn_2 = base.Columns["เลขท\u0e35\u0e48"];
			dataColumn_3 = base.Columns["ส\u0e48งของท\u0e35\u0e48"];
			dataColumn_4 = base.Columns["ช\u0e37\u0e48อ"];
			dataColumn_5 = base.Columns["ท\u0e35\u0e48อย\u0e39\u0e48"];
			dataColumn_6 = base.Columns["ลำด\u0e31บ"];
			dataColumn_7 = base.Columns["รายการ"];
			dataColumn_8 = base.Columns["จำนวน"];
			dataColumn_9 = base.Columns["ช\u0e37\u0e48อหน\u0e48วย"];
			dataColumn_10 = base.Columns["หน\u0e48วยละ"];
			dataColumn_11 = base.Columns["จำนวนเง\u0e34น"];
			dataColumn_12 = base.Columns["ราคารวม"];
			dataColumn_13 = base.Columns["ภาษ\u0e35เปอร\u0e4cเซ\u0e49น"];
			dataColumn_14 = base.Columns["ภาษ\u0e35ราคา"];
			dataColumn_15 = base.Columns["ราคารวมภาษ\u0e35"];
			dataColumn_16 = base.Columns["ผ\u0e39\u0e49ออกบ\u0e34ล"];
			dataColumn_17 = base.Columns["ผ\u0e39\u0e49ส\u0e48ง"];
			dataColumn_18 = base.Columns["ราคาtext"];
			columnbarcode = base.Columns["barcode"];
			dataColumn_19 = base.Columns["ส\u0e48วนลดรายการ"];
			dataColumn_20 = base.Columns["ส\u0e48วนลดรวม"];
			dataColumn_21 = base.Columns["ห\u0e31วบ\u0e34ล"];
			dataColumn_22 = base.Columns["หมายเหต\u0e38"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			dataColumn_0 = new DataColumn("กร\u0e38\u0e4aป", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("ว\u0e31นท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("เลขท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			dataColumn_3 = new DataColumn("ส\u0e48งของท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			dataColumn_4 = new DataColumn("ช\u0e37\u0e48อ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_4);
			dataColumn_5 = new DataColumn("ท\u0e35\u0e48อย\u0e39\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_5);
			dataColumn_6 = new DataColumn("ลำด\u0e31บ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_6);
			dataColumn_7 = new DataColumn("รายการ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_7);
			dataColumn_8 = new DataColumn("จำนวน", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_8);
			dataColumn_9 = new DataColumn("ช\u0e37\u0e48อหน\u0e48วย", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_9);
			dataColumn_10 = new DataColumn("หน\u0e48วยละ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_10);
			dataColumn_11 = new DataColumn("จำนวนเง\u0e34น", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_11);
			dataColumn_12 = new DataColumn("ราคารวม", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_12);
			dataColumn_13 = new DataColumn("ภาษ\u0e35เปอร\u0e4cเซ\u0e49น", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_13);
			dataColumn_14 = new DataColumn("ภาษ\u0e35ราคา", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_14);
			dataColumn_15 = new DataColumn("ราคารวมภาษ\u0e35", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_15);
			dataColumn_16 = new DataColumn("ผ\u0e39\u0e49ออกบ\u0e34ล", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_16);
			dataColumn_17 = new DataColumn("ผ\u0e39\u0e49ส\u0e48ง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_17);
			dataColumn_18 = new DataColumn("ราคาtext", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_18);
			columnbarcode = new DataColumn("barcode", typeof(byte[]), null, MappingType.Element);
			base.Columns.Add(columnbarcode);
			dataColumn_19 = new DataColumn("ส\u0e48วนลดรายการ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_19);
			dataColumn_20 = new DataColumn("ส\u0e48วนลดรวม", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_20);
			dataColumn_21 = new DataColumn("ห\u0e31วบ\u0e34ล", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_21);
			dataColumn_22 = new DataColumn("หมายเหต\u0e38", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_22);
		}

		[DebuggerNonUserCode]
		public ReportBillCashRow NewReportBillCashRow()
		{
			return (ReportBillCashRow)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new ReportBillCashRow(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(ReportBillCashRow);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (ReportBillCashRowChanged != null)
			{
				ReportBillCashRowChanged?.Invoke(this, new ReportBillCashRowChangeEvent((ReportBillCashRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (ReportBillCashRowChanging != null)
			{
				ReportBillCashRowChanging?.Invoke(this, new ReportBillCashRowChangeEvent((ReportBillCashRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (ReportBillCashRowDeleted != null)
			{
				ReportBillCashRowDeleted?.Invoke(this, new ReportBillCashRowChangeEvent((ReportBillCashRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (ReportBillCashRowDeleting != null)
			{
				ReportBillCashRowDeleting?.Invoke(this, new ReportBillCashRowChangeEvent((ReportBillCashRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReportBillCashRow(ReportBillCashRow row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "ReportBillCashDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	[XmlSchemaProvider("GetTypedTableSchema")]
	public class Bill_HDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn columnCompany_name;

		private DataColumn columnCompany_address;

		private DataColumn columnCompany_TaxID;

		private DataColumn columnCompany_Tel;

		private DataColumn columnCompany_Fax;

		private DataColumn columnCopany_Logo;

		[DebuggerNonUserCode]
		public DataColumn Company_nameColumn => columnCompany_name;

		[DebuggerNonUserCode]
		public DataColumn Company_addressColumn => columnCompany_address;

		[DebuggerNonUserCode]
		public DataColumn Company_TaxIDColumn => columnCompany_TaxID;

		[DebuggerNonUserCode]
		public DataColumn Company_TelColumn => columnCompany_Tel;

		[DebuggerNonUserCode]
		public DataColumn Company_FaxColumn => columnCompany_Fax;

		[DebuggerNonUserCode]
		public DataColumn Copany_LogoColumn => columnCopany_Logo;

		[Browsable(false)]
		[DebuggerNonUserCode]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public Bill_HRow this[int index] => (Bill_HRow)Rows[index];

		[method: DebuggerNonUserCode]
		public event Bill_HRowChangeEventHandler Bill_HRowChanging;

		[method: DebuggerNonUserCode]
		public event Bill_HRowChangeEventHandler Bill_HRowChanged;

		[method: DebuggerNonUserCode]
		public event Bill_HRowChangeEventHandler Bill_HRowDeleting;

		[method: DebuggerNonUserCode]
		public event Bill_HRowChangeEventHandler Bill_HRowDeleted;

		[DebuggerNonUserCode]
		static Bill_HDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public Bill_HDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "Bill_H";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal Bill_HDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected Bill_HDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddBill_HRow(Bill_HRow row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public Bill_HRow AddBill_HRow(string Company_Name, string Company_address, string Company_TaxID, string Company_Tel, string Company_Fax, byte[] Copany_Logo)
		{
			Bill_HRow bill_HRow = (Bill_HRow)NewRow();
			object[] itemArray = new object[6] { Company_Name, Company_address, Company_TaxID, Company_Tel, Company_Fax, Copany_Logo };
			bill_HRow.ItemArray = itemArray;
			Rows.Add(bill_HRow);
			return bill_HRow;
		}

		[DebuggerNonUserCode]
		public Bill_HRow FindByCompany_Name(string Company_Name)
		{
			return (Bill_HRow)Rows.Find(new object[1] { Company_Name });
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			Bill_HDataTable bill_HDataTable = (Bill_HDataTable)base.Clone();
			bill_HDataTable.InitVars();
			return bill_HDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new Bill_HDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			columnCompany_name = base.Columns["Company_Name"];
			columnCompany_address = base.Columns["Company_address"];
			columnCompany_TaxID = base.Columns["Company_TaxID"];
			columnCompany_Tel = base.Columns["Company_Tel"];
			columnCompany_Fax = base.Columns["Company_Fax"];
			columnCopany_Logo = base.Columns["Copany_Logo"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			columnCompany_name = new DataColumn("Company_Name", typeof(string), null, MappingType.Element);
			columnCompany_name.ExtendedProperties.Add("Generator_ColumnPropNameInTable", "Company_nameColumn");
			columnCompany_name.ExtendedProperties.Add("Generator_ColumnVarNameInTable", "columnCompany_name");
			columnCompany_name.ExtendedProperties.Add("Generator_UserColumnName", "Company_Name");
			base.Columns.Add(columnCompany_name);
			columnCompany_address = new DataColumn("Company_address", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnCompany_address);
			columnCompany_TaxID = new DataColumn("Company_TaxID", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnCompany_TaxID);
			columnCompany_Tel = new DataColumn("Company_Tel", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnCompany_Tel);
			columnCompany_Fax = new DataColumn("Company_Fax", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnCompany_Fax);
			columnCopany_Logo = new DataColumn("Copany_Logo", typeof(byte[]), null, MappingType.Element);
			base.Columns.Add(columnCopany_Logo);
			Constraints.Add(new UniqueConstraint("Constraint1", new DataColumn[1] { columnCompany_name }, isPrimaryKey: true));
			columnCompany_name.AllowDBNull = false;
			columnCompany_name.Unique = true;
		}

		[DebuggerNonUserCode]
		public Bill_HRow NewBill_HRow()
		{
			return (Bill_HRow)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new Bill_HRow(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(Bill_HRow);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (Bill_HRowChanged != null)
			{
				Bill_HRowChanged?.Invoke(this, new Bill_HRowChangeEvent((Bill_HRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (Bill_HRowChanging != null)
			{
				Bill_HRowChanging?.Invoke(this, new Bill_HRowChangeEvent((Bill_HRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (Bill_HRowDeleted != null)
			{
				Bill_HRowDeleted?.Invoke(this, new Bill_HRowChangeEvent((Bill_HRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (Bill_HRowDeleting != null)
			{
				Bill_HRowDeleting?.Invoke(this, new Bill_HRowChangeEvent((Bill_HRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveBill_HRow(Bill_HRow row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "Bill_HDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	[XmlSchemaProvider("GetTypedTableSchema")]
	public class ReportRegDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn dataColumn_3;

		private DataColumn dataColumn_4;

		private DataColumn dataColumn_5;

		private DataColumn dataColumn_6;

		private DataColumn dataColumn_7;

		private DataColumn dataColumn_8;

		private DataColumn dataColumn_9;

		private DataColumn dataColumn_10;

		private DataColumn dataColumn_11;

		private DataColumn dataColumn_12;

		private DataColumn dataColumn_13;

		private DataColumn dataColumn_14;

		private DataColumn dataColumn_15;

		private DataColumn columnpic;

		private DataColumn dataColumn_16;

		private DataColumn dataColumn_17;

		private DataColumn dataColumn_18;

		private DataColumn dataColumn_19;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_4 => dataColumn_4;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_5 => dataColumn_5;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_6 => dataColumn_6;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_7 => dataColumn_7;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_8 => dataColumn_8;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_9 => dataColumn_9;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_10 => dataColumn_10;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_11 => dataColumn_11;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_12 => dataColumn_12;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_13 => dataColumn_13;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_14 => dataColumn_14;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_15 => dataColumn_15;

		[DebuggerNonUserCode]
		public DataColumn picColumn => columnpic;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_16 => dataColumn_16;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_17 => dataColumn_17;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_18 => dataColumn_18;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_19 => dataColumn_19;

		[Browsable(false)]
		[DebuggerNonUserCode]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public ReportRegRow this[int index] => (ReportRegRow)Rows[index];

		[method: DebuggerNonUserCode]
		public event ReportRegRowChangeEventHandler ReportRegRowChanging;

		[method: DebuggerNonUserCode]
		public event ReportRegRowChangeEventHandler ReportRegRowChanged;

		[method: DebuggerNonUserCode]
		public event ReportRegRowChangeEventHandler ReportRegRowDeleting;

		[method: DebuggerNonUserCode]
		public event ReportRegRowChangeEventHandler ReportRegRowDeleted;

		[DebuggerNonUserCode]
		static ReportRegDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public ReportRegDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "ReportReg";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal ReportRegDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected ReportRegDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReportRegRow(ReportRegRow row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public ReportRegRow AddReportRegRow(string string_0, string string_1, string string_2, string string_3, string string_4, string string_5, string string_6, string string_7, string string_8, string string_9, string string_10, string string_11, string string_12, string string_13, string string_14, string string_15, byte[] pic, string string_16, string string_17, string string_18, string string_19)
		{
			ReportRegRow reportRegRow = (ReportRegRow)NewRow();
			object[] itemArray = new object[21]
			{
				string_0, string_1, string_2, string_3, string_4, string_5, string_6, string_7, string_8, string_9,
				string_10, string_11, string_12, string_13, string_14, string_15, pic, string_16, string_17, string_18,
				string_19
			};
			reportRegRow.ItemArray = itemArray;
			Rows.Add(reportRegRow);
			return reportRegRow;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			ReportRegDataTable reportRegDataTable = (ReportRegDataTable)base.Clone();
			reportRegDataTable.InitVars();
			return reportRegDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new ReportRegDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			dataColumn_0 = base.Columns["กร\u0e38\u0e4aป"];
			dataColumn_1 = base.Columns["ว\u0e31นท\u0e35\u0e48"];
			dataColumn_2 = base.Columns["เลขท\u0e35\u0e48"];
			dataColumn_3 = base.Columns["ห\u0e49อง"];
			dataColumn_4 = base.Columns["ช\u0e37\u0e48อ"];
			dataColumn_5 = base.Columns["ท\u0e35\u0e48อย\u0e39\u0e48"];
			dataColumn_6 = base.Columns["อาช\u0e35พ"];
			dataColumn_7 = base.Columns["ส\u0e31ญชาต\u0e34"];
			dataColumn_8 = base.Columns["มาจาก"];
			dataColumn_9 = base.Columns["ไปท\u0e35\u0e48"];
			dataColumn_10 = base.Columns["หมายเลข"];
			dataColumn_11 = base.Columns["โทร"];
			dataColumn_12 = base.Columns["ว\u0e31นท\u0e35\u0e48ออก"];
			dataColumn_13 = base.Columns["ราคา"];
			dataColumn_14 = base.Columns["ราคารวม"];
			dataColumn_15 = base.Columns["ทะเบ\u0e35ยนรถ"];
			columnpic = base.Columns["pic"];
			dataColumn_16 = base.Columns["รห\u0e31สล\u0e39กค\u0e49า"];
			dataColumn_17 = base.Columns["หมายเหต\u0e38"];
			dataColumn_18 = base.Columns["ราคาส\u0e34นค\u0e49า"];
			dataColumn_19 = base.Columns["ราคารวมส\u0e34นค\u0e49า"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			dataColumn_0 = new DataColumn("กร\u0e38\u0e4aป", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("ว\u0e31นท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("เลขท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			dataColumn_3 = new DataColumn("ห\u0e49อง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			dataColumn_4 = new DataColumn("ช\u0e37\u0e48อ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_4);
			dataColumn_5 = new DataColumn("ท\u0e35\u0e48อย\u0e39\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_5);
			dataColumn_6 = new DataColumn("อาช\u0e35พ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_6);
			dataColumn_7 = new DataColumn("ส\u0e31ญชาต\u0e34", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_7);
			dataColumn_8 = new DataColumn("มาจาก", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_8);
			dataColumn_9 = new DataColumn("ไปท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_9);
			dataColumn_10 = new DataColumn("หมายเลข", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_10);
			dataColumn_11 = new DataColumn("โทร", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_11);
			dataColumn_12 = new DataColumn("ว\u0e31นท\u0e35\u0e48ออก", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_12);
			dataColumn_13 = new DataColumn("ราคา", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_13);
			dataColumn_14 = new DataColumn("ราคารวม", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_14);
			dataColumn_15 = new DataColumn("ทะเบ\u0e35ยนรถ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_15);
			columnpic = new DataColumn("pic", typeof(byte[]), null, MappingType.Element);
			base.Columns.Add(columnpic);
			dataColumn_16 = new DataColumn("รห\u0e31สล\u0e39กค\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_16);
			dataColumn_17 = new DataColumn("หมายเหต\u0e38", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_17);
			dataColumn_18 = new DataColumn("ราคาส\u0e34นค\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_18);
			dataColumn_19 = new DataColumn("ราคารวมส\u0e34นค\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_19);
			dataColumn_3.Caption = "ส\u0e48งของท\u0e35\u0e48";
			dataColumn_6.Caption = "ลำด\u0e31บ";
			dataColumn_7.Caption = "รายการ";
			dataColumn_8.Caption = "จำนวน";
			dataColumn_9.Caption = "ช\u0e37\u0e48อหน\u0e48วย";
			dataColumn_10.Caption = "หน\u0e48วยละ";
		}

		[DebuggerNonUserCode]
		public ReportRegRow NewReportRegRow()
		{
			return (ReportRegRow)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new ReportRegRow(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(ReportRegRow);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (ReportRegRowChanged != null)
			{
				ReportRegRowChanged?.Invoke(this, new ReportRegRowChangeEvent((ReportRegRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (ReportRegRowChanging != null)
			{
				ReportRegRowChanging?.Invoke(this, new ReportRegRowChangeEvent((ReportRegRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (ReportRegRowDeleted != null)
			{
				ReportRegRowDeleted?.Invoke(this, new ReportRegRowChangeEvent((ReportRegRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (ReportRegRowDeleting != null)
			{
				ReportRegRowDeleting?.Invoke(this, new ReportRegRowChangeEvent((ReportRegRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReportRegRow(ReportRegRow row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "ReportRegDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[XmlSchemaProvider("GetTypedTableSchema")]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportDepDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn dataColumn_3;

		private DataColumn dataColumn_4;

		private DataColumn dataColumn_5;

		private DataColumn dataColumn_6;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_4 => dataColumn_4;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_5 => dataColumn_5;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_6 => dataColumn_6;

		[Browsable(false)]
		[DebuggerNonUserCode]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public ReportDepRow this[int index] => (ReportDepRow)Rows[index];

		[method: DebuggerNonUserCode]
		public event ReportDepRowChangeEventHandler ReportDepRowChanging;

		[method: DebuggerNonUserCode]
		public event ReportDepRowChangeEventHandler ReportDepRowChanged;

		[method: DebuggerNonUserCode]
		public event ReportDepRowChangeEventHandler ReportDepRowDeleting;

		[method: DebuggerNonUserCode]
		public event ReportDepRowChangeEventHandler ReportDepRowDeleted;

		[DebuggerNonUserCode]
		static ReportDepDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public ReportDepDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "ReportDep";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal ReportDepDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected ReportDepDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReportDepRow(ReportDepRow row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public ReportDepRow AddReportDepRow(string string_0, string string_1, string string_2, string string_3, string string_4, string string_5, string string_6)
		{
			ReportDepRow reportDepRow = (ReportDepRow)NewRow();
			object[] itemArray = new object[7] { string_0, string_1, string_2, string_3, string_4, string_5, string_6 };
			reportDepRow.ItemArray = itemArray;
			Rows.Add(reportDepRow);
			return reportDepRow;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			ReportDepDataTable reportDepDataTable = (ReportDepDataTable)base.Clone();
			reportDepDataTable.InitVars();
			return reportDepDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new ReportDepDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			dataColumn_0 = base.Columns["กร\u0e38\u0e4aป"];
			dataColumn_1 = base.Columns["ว\u0e31นท\u0e35\u0e48"];
			dataColumn_2 = base.Columns["เลขท\u0e35\u0e48"];
			dataColumn_3 = base.Columns["ห\u0e49อง"];
			dataColumn_4 = base.Columns["ช\u0e37\u0e48อ"];
			dataColumn_5 = base.Columns["จำนวนเง\u0e34น"];
			dataColumn_6 = base.Columns["ต\u0e31วอ\u0e31กษร"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			dataColumn_0 = new DataColumn("กร\u0e38\u0e4aป", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("ว\u0e31นท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("เลขท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			dataColumn_3 = new DataColumn("ห\u0e49อง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			dataColumn_4 = new DataColumn("ช\u0e37\u0e48อ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_4);
			dataColumn_5 = new DataColumn("จำนวนเง\u0e34น", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_5);
			dataColumn_6 = new DataColumn("ต\u0e31วอ\u0e31กษร", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_6);
			dataColumn_3.Caption = "ส\u0e48งของท\u0e35\u0e48";
			dataColumn_5.Caption = "ท\u0e35\u0e48อย\u0e39\u0e48";
			dataColumn_6.Caption = "ลำด\u0e31บ";
		}

		[DebuggerNonUserCode]
		public ReportDepRow NewReportDepRow()
		{
			return (ReportDepRow)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new ReportDepRow(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(ReportDepRow);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (ReportDepRowChanged != null)
			{
				ReportDepRowChanged?.Invoke(this, new ReportDepRowChangeEvent((ReportDepRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (ReportDepRowChanging != null)
			{
				ReportDepRowChanging?.Invoke(this, new ReportDepRowChangeEvent((ReportDepRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (ReportDepRowDeleted != null)
			{
				ReportDepRowDeleted?.Invoke(this, new ReportDepRowChangeEvent((ReportDepRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (ReportDepRowDeleting != null)
			{
				ReportDepRowDeleting?.Invoke(this, new ReportDepRowChangeEvent((ReportDepRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReportDepRow(ReportDepRow row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "ReportDepDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[XmlSchemaProvider("GetTypedTableSchema")]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ConfigDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn columnThemeColor;

		private DataColumn columnServerIP;

		private DataColumn columnServerPassword;

		private DataColumn columnStore_camera;

		[DebuggerNonUserCode]
		public DataColumn ThemeColorColumn => columnThemeColor;

		[DebuggerNonUserCode]
		public DataColumn ServerIPColumn => columnServerIP;

		[DebuggerNonUserCode]
		public DataColumn ServerPasswordColumn => columnServerPassword;

		[DebuggerNonUserCode]
		public DataColumn Store_cameraColumn => columnStore_camera;

		[Browsable(false)]
		[DebuggerNonUserCode]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public ConfigRow this[int index] => (ConfigRow)Rows[index];

		[method: DebuggerNonUserCode]
		public event ConfigRowChangeEventHandler ConfigRowChanging;

		[method: DebuggerNonUserCode]
		public event ConfigRowChangeEventHandler ConfigRowChanged;

		[method: DebuggerNonUserCode]
		public event ConfigRowChangeEventHandler ConfigRowDeleting;

		[method: DebuggerNonUserCode]
		public event ConfigRowChangeEventHandler ConfigRowDeleted;

		[DebuggerNonUserCode]
		static ConfigDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public ConfigDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "Config";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal ConfigDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected ConfigDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddConfigRow(ConfigRow row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public ConfigRow AddConfigRow(string ThemeColor, string ServerIP, string ServerPassword, string Store_camera)
		{
			ConfigRow configRow = (ConfigRow)NewRow();
			object[] itemArray = new object[4] { ThemeColor, ServerIP, ServerPassword, Store_camera };
			configRow.ItemArray = itemArray;
			Rows.Add(configRow);
			return configRow;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			ConfigDataTable configDataTable = (ConfigDataTable)base.Clone();
			configDataTable.InitVars();
			return configDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new ConfigDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			columnThemeColor = base.Columns["ThemeColor"];
			columnServerIP = base.Columns["ServerIP"];
			columnServerPassword = base.Columns["ServerPassword"];
			columnStore_camera = base.Columns["Store_camera"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			columnThemeColor = new DataColumn("ThemeColor", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnThemeColor);
			columnServerIP = new DataColumn("ServerIP", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnServerIP);
			columnServerPassword = new DataColumn("ServerPassword", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnServerPassword);
			columnStore_camera = new DataColumn("Store_camera", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnStore_camera);
		}

		[DebuggerNonUserCode]
		public ConfigRow NewConfigRow()
		{
			return (ConfigRow)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new ConfigRow(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(ConfigRow);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (ConfigRowChanged != null)
			{
				ConfigRowChanged?.Invoke(this, new ConfigRowChangeEvent((ConfigRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (ConfigRowChanging != null)
			{
				ConfigRowChanging?.Invoke(this, new ConfigRowChangeEvent((ConfigRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (ConfigRowDeleted != null)
			{
				ConfigRowDeleted?.Invoke(this, new ConfigRowChangeEvent((ConfigRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (ConfigRowDeleting != null)
			{
				ConfigRowDeleting?.Invoke(this, new ConfigRowChangeEvent((ConfigRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveConfigRow(ConfigRow row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "ConfigDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[XmlSchemaProvider("GetTypedTableSchema")]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportPicDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn columnPicData;

		[DebuggerNonUserCode]
		public DataColumn PicDataColumn => columnPicData;

		[DebuggerNonUserCode]
		[Browsable(false)]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public ReportPicRow this[int index] => (ReportPicRow)Rows[index];

		[method: DebuggerNonUserCode]
		public event ReportPicRowChangeEventHandler ReportPicRowChanging;

		[method: DebuggerNonUserCode]
		public event ReportPicRowChangeEventHandler ReportPicRowChanged;

		[method: DebuggerNonUserCode]
		public event ReportPicRowChangeEventHandler ReportPicRowDeleting;

		[method: DebuggerNonUserCode]
		public event ReportPicRowChangeEventHandler ReportPicRowDeleted;

		[DebuggerNonUserCode]
		static ReportPicDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public ReportPicDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "ReportPic";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal ReportPicDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected ReportPicDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReportPicRow(ReportPicRow row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public ReportPicRow AddReportPicRow(byte[] PicData)
		{
			ReportPicRow reportPicRow = (ReportPicRow)NewRow();
			object[] itemArray = new object[1] { PicData };
			reportPicRow.ItemArray = itemArray;
			Rows.Add(reportPicRow);
			return reportPicRow;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			ReportPicDataTable reportPicDataTable = (ReportPicDataTable)base.Clone();
			reportPicDataTable.InitVars();
			return reportPicDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new ReportPicDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			columnPicData = base.Columns["PicData"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			columnPicData = new DataColumn("PicData", typeof(byte[]), null, MappingType.Element);
			base.Columns.Add(columnPicData);
		}

		[DebuggerNonUserCode]
		public ReportPicRow NewReportPicRow()
		{
			return (ReportPicRow)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new ReportPicRow(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(ReportPicRow);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (ReportPicRowChanged != null)
			{
				ReportPicRowChanged?.Invoke(this, new ReportPicRowChangeEvent((ReportPicRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (ReportPicRowChanging != null)
			{
				ReportPicRowChanging?.Invoke(this, new ReportPicRowChangeEvent((ReportPicRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (ReportPicRowDeleted != null)
			{
				ReportPicRowDeleted?.Invoke(this, new ReportPicRowChangeEvent((ReportPicRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (ReportPicRowDeleting != null)
			{
				ReportPicRowDeleting?.Invoke(this, new ReportPicRowChangeEvent((ReportPicRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReportPicRow(ReportPicRow row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "ReportPicDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	[XmlSchemaProvider("GetTypedTableSchema")]
	public class ReportDaysDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn dataColumn_3;

		private DataColumn dataColumn_4;

		private DataColumn dataColumn_5;

		private DataColumn dataColumn_6;

		private DataColumn dataColumn_7;

		private DataColumn dataColumn_8;

		private DataColumn dataColumn_9;

		private DataColumn dataColumn_10;

		private DataColumn dataColumn_11;

		private DataColumn dataColumn_12;

		private DataColumn dataColumn_13;

		private DataColumn dataColumn_14;

		private DataColumn dataColumn_15;

		private DataColumn dataColumn_16;

		private DataColumn dataColumn_17;

		private DataColumn dataColumn_18;

		private DataColumn dataColumn_19;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_4 => dataColumn_4;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_5 => dataColumn_5;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_6 => dataColumn_6;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_7 => dataColumn_7;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_8 => dataColumn_8;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_9 => dataColumn_9;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_10 => dataColumn_10;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_11 => dataColumn_11;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_12 => dataColumn_12;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_13 => dataColumn_13;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_14 => dataColumn_14;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_15 => dataColumn_15;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_16 => dataColumn_16;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_17 => dataColumn_17;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_18 => dataColumn_18;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_19 => dataColumn_19;

		[DebuggerNonUserCode]
		[Browsable(false)]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public ReportDaysRow this[int index] => (ReportDaysRow)Rows[index];

		[method: DebuggerNonUserCode]
		public event ReportDaysRowChangeEventHandler ReportDaysRowChanging;

		[method: DebuggerNonUserCode]
		public event ReportDaysRowChangeEventHandler ReportDaysRowChanged;

		[method: DebuggerNonUserCode]
		public event ReportDaysRowChangeEventHandler ReportDaysRowDeleting;

		[method: DebuggerNonUserCode]
		public event ReportDaysRowChangeEventHandler ReportDaysRowDeleted;

		[DebuggerNonUserCode]
		static ReportDaysDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public ReportDaysDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			ReportDaysRowChanging += ReportDaysDataTable_ReportDaysRowChanging;
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "ReportDays";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal ReportDaysDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			ReportDaysRowChanging += ReportDaysDataTable_ReportDaysRowChanging;
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected ReportDaysDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			ReportDaysRowChanging += ReportDaysDataTable_ReportDaysRowChanging;
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReportDaysRow(ReportDaysRow row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public ReportDaysRow AddReportDaysRow(string string_0, string string_1, string string_2, string string_3, string string_4, string string_5, string string_6, string string_7, string string_8, string string_9, string string_10, string string_11, string string_12, string string_13, string string_14, string string_15, string string_16, string string_17, string string_18, string string_19)
		{
			ReportDaysRow reportDaysRow = (ReportDaysRow)NewRow();
			object[] itemArray = new object[20]
			{
				string_0, string_1, string_2, string_3, string_4, string_5, string_6, string_7, string_8, string_9,
				string_10, string_11, string_12, string_13, string_14, string_15, string_16, string_17, string_18, string_19
			};
			reportDaysRow.ItemArray = itemArray;
			Rows.Add(reportDaysRow);
			return reportDaysRow;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			ReportDaysDataTable reportDaysDataTable = (ReportDaysDataTable)base.Clone();
			reportDaysDataTable.InitVars();
			return reportDaysDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new ReportDaysDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			dataColumn_0 = base.Columns["ว\u0e31นท\u0e35\u0e48"];
			dataColumn_1 = base.Columns["ลำด\u0e31บ"];
			dataColumn_2 = base.Columns["ห\u0e49อง"];
			dataColumn_3 = base.Columns["ชน\u0e34ดห\u0e49อง"];
			dataColumn_4 = base.Columns["เข\u0e49า"];
			dataColumn_5 = base.Columns["ออก"];
			dataColumn_6 = base.Columns["ค\u0e48าห\u0e49อง"];
			dataColumn_7 = base.Columns["ค\u0e48าส\u0e34นค\u0e49า"];
			dataColumn_8 = base.Columns["รวม"];
			dataColumn_9 = base.Columns["รวมห\u0e49อง"];
			dataColumn_10 = base.Columns["รวมราคา"];
			dataColumn_11 = base.Columns["ช\u0e37\u0e48อ"];
			dataColumn_12 = base.Columns["จำนวนว\u0e31น"];
			dataColumn_13 = base.Columns["ว\u0e31ท\u0e35\u0e48ออก"];
			dataColumn_14 = base.Columns["หมายเหต\u0e38"];
			dataColumn_15 = base.Columns["ว\u0e31นท\u0e35\u0e48พ\u0e31กต\u0e48อ"];
			dataColumn_16 = base.Columns["ชำระแล\u0e49ว"];
			dataColumn_17 = base.Columns["คงค\u0e49าง"];
			dataColumn_18 = base.Columns["รวมชำระแล\u0e49ว"];
			dataColumn_19 = base.Columns["รวมคงค\u0e49าง"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			dataColumn_0 = new DataColumn("ว\u0e31นท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("ลำด\u0e31บ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("ห\u0e49อง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			dataColumn_3 = new DataColumn("ชน\u0e34ดห\u0e49อง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			dataColumn_4 = new DataColumn("เข\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_4);
			dataColumn_5 = new DataColumn("ออก", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_5);
			dataColumn_6 = new DataColumn("ค\u0e48าห\u0e49อง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_6);
			dataColumn_7 = new DataColumn("ค\u0e48าส\u0e34นค\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_7);
			dataColumn_8 = new DataColumn("รวม", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_8);
			dataColumn_9 = new DataColumn("รวมห\u0e49อง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_9);
			dataColumn_10 = new DataColumn("รวมราคา", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_10);
			dataColumn_11 = new DataColumn("ช\u0e37\u0e48อ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_11);
			dataColumn_12 = new DataColumn("จำนวนว\u0e31น", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_12);
			dataColumn_13 = new DataColumn("ว\u0e31ท\u0e35\u0e48ออก", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_13);
			dataColumn_14 = new DataColumn("หมายเหต\u0e38", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_14);
			dataColumn_15 = new DataColumn("ว\u0e31นท\u0e35\u0e48พ\u0e31กต\u0e48อ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_15);
			dataColumn_16 = new DataColumn("ชำระแล\u0e49ว", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_16);
			dataColumn_17 = new DataColumn("คงค\u0e49าง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_17);
			dataColumn_18 = new DataColumn("รวมชำระแล\u0e49ว", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_18);
			dataColumn_19 = new DataColumn("รวมคงค\u0e49าง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_19);
			dataColumn_0.Caption = "DataColumn1";
			dataColumn_1.Caption = "DataColumn1";
			dataColumn_2.Caption = "DataColumn1";
			dataColumn_3.Caption = "DataColumn1";
			dataColumn_4.Caption = "DataColumn1";
			dataColumn_5.Caption = "DataColumn1";
			dataColumn_6.Caption = "DataColumn1";
			dataColumn_7.Caption = "DataColumn1";
			dataColumn_8.Caption = "DataColumn1";
			dataColumn_9.Caption = "DataColumn1";
			dataColumn_16.Caption = "DataColumn1";
			dataColumn_17.Caption = "DataColumn1";
			dataColumn_18.Caption = "DataColumn1";
		}

		[DebuggerNonUserCode]
		public ReportDaysRow NewReportDaysRow()
		{
			return (ReportDaysRow)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new ReportDaysRow(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(ReportDaysRow);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (ReportDaysRowChanged != null)
			{
				ReportDaysRowChanged?.Invoke(this, new ReportDaysRowChangeEvent((ReportDaysRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (ReportDaysRowChanging != null)
			{
				ReportDaysRowChanging?.Invoke(this, new ReportDaysRowChangeEvent((ReportDaysRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (ReportDaysRowDeleted != null)
			{
				ReportDaysRowDeleted?.Invoke(this, new ReportDaysRowChangeEvent((ReportDaysRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (ReportDaysRowDeleting != null)
			{
				ReportDaysRowDeleting?.Invoke(this, new ReportDaysRowChangeEvent((ReportDaysRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReportDaysRow(ReportDaysRow row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "ReportDaysDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}

		private void ReportDaysDataTable_ReportDaysRowChanging(object sender, ReportDaysRowChangeEvent e)
		{
		}
	}

	[Serializable]
	[XmlSchemaProvider("GetTypedTableSchema")]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportCustINDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn dataColumn_3;

		private DataColumn dataColumn_4;

		private DataColumn dataColumn_5;

		private DataColumn dataColumn_6;

		private DataColumn dataColumn_7;

		private DataColumn dataColumn_8;

		private DataColumn dataColumn_9;

		private DataColumn dataColumn_10;

		private DataColumn dataColumn_11;

		private DataColumn dataColumn_12;

		private DataColumn dataColumn_13;

		private DataColumn dataColumn_14;

		private DataColumn dataColumn_15;

		private DataColumn columnBalance;

		private DataColumn dataColumn_16;

		private DataColumn columnnote2;

		private DataColumn dataColumn_17;

		private DataColumn dataColumn_18;

		private DataColumn dataColumn_19;

		private DataColumn dataColumn_20;

		private DataColumn dataColumn_21;

		private DataColumn dataColumn_22;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_4 => dataColumn_4;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_5 => dataColumn_5;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_6 => dataColumn_6;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_7 => dataColumn_7;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_8 => dataColumn_8;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_9 => dataColumn_9;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_10 => dataColumn_10;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_11 => dataColumn_11;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_12 => dataColumn_12;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_13 => dataColumn_13;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_14 => dataColumn_14;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_15 => dataColumn_15;

		[DebuggerNonUserCode]
		public DataColumn BalanceColumn => columnBalance;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_16 => dataColumn_16;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_17 => columnnote2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_18 => dataColumn_17;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_19 => dataColumn_18;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_20 => dataColumn_19;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_21 => dataColumn_20;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_22 => dataColumn_21;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_23 => dataColumn_22;

		[Browsable(false)]
		[DebuggerNonUserCode]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public ReportCustINRow this[int index] => (ReportCustINRow)Rows[index];

		[method: DebuggerNonUserCode]
		public event ReportCustINRowChangeEventHandler ReportCustINRowChanging;

		[method: DebuggerNonUserCode]
		public event ReportCustINRowChangeEventHandler ReportCustINRowChanged;

		[method: DebuggerNonUserCode]
		public event ReportCustINRowChangeEventHandler ReportCustINRowDeleting;

		[method: DebuggerNonUserCode]
		public event ReportCustINRowChangeEventHandler ReportCustINRowDeleted;

		[DebuggerNonUserCode]
		static ReportCustINDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public ReportCustINDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "ReportCustIN";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal ReportCustINDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected ReportCustINDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReportCustINRow(ReportCustINRow row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public ReportCustINRow AddReportCustINRow(string string_0, string string_1, string string_2, string string_3, string string_4, string string_5, string string_6, string string_7, string string_8, string string_9, string string_10, string string_11, string string_12, string string_13, string string_14, string string_15, string Balance, string string_16, string note2, string string_17, string string_18, string string_19, string string_20, string string_21, string string_22)
		{
			ReportCustINRow reportCustINRow = (ReportCustINRow)NewRow();
			object[] itemArray = new object[25]
			{
				string_0, string_1, string_2, string_3, string_4, string_5, string_6, string_7, string_8, string_9,
				string_10, string_11, string_12, string_13, string_14, string_15, Balance, string_16, note2, string_17,
				string_18, string_19, string_20, string_21, string_22
			};
			reportCustINRow.ItemArray = itemArray;
			Rows.Add(reportCustINRow);
			return reportCustINRow;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			ReportCustINDataTable reportCustINDataTable = (ReportCustINDataTable)base.Clone();
			reportCustINDataTable.InitVars();
			return reportCustINDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new ReportCustINDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			dataColumn_0 = base.Columns["ว\u0e31นท\u0e35\u0e48"];
			dataColumn_1 = base.Columns["ลำด\u0e31บ"];
			dataColumn_2 = base.Columns["เลขท\u0e35\u0e48"];
			dataColumn_3 = base.Columns["เบอร\u0e4cห\u0e49อง"];
			dataColumn_4 = base.Columns["ราคาห\u0e49อง"];
			dataColumn_5 = base.Columns["ราคาส\u0e34นค\u0e49า"];
			dataColumn_6 = base.Columns["รวม"];
			dataColumn_7 = base.Columns["เข\u0e49า"];
			dataColumn_8 = base.Columns["ออก"];
			dataColumn_9 = base.Columns["ส\u0e34นค\u0e49า"];
			dataColumn_10 = base.Columns["รวมห\u0e49อง"];
			dataColumn_11 = base.Columns["รวมส\u0e34นค\u0e49า"];
			dataColumn_12 = base.Columns["รวมท\u0e31\u0e49งหมด"];
			dataColumn_13 = base.Columns["ช\u0e37\u0e48อ"];
			dataColumn_14 = base.Columns["สถานะการจ\u0e48าย"];
			dataColumn_15 = base.Columns["พน\u0e31กงาน"];
			columnBalance = base.Columns["Balance"];
			dataColumn_16 = base.Columns["หมายเหต\u0e38"];
			columnnote2 = base.Columns["note2"];
			dataColumn_17 = base.Columns["ราคาต\u0e48อค\u0e37น"];
			dataColumn_18 = base.Columns["รวมราคาต\u0e48อค\u0e37น"];
			dataColumn_19 = base.Columns["ชำระแล\u0e49ว"];
			dataColumn_20 = base.Columns["คงค\u0e49าง"];
			dataColumn_21 = base.Columns["รวมชำระแล\u0e49ว"];
			dataColumn_22 = base.Columns["รวมคงค\u0e49าง"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			dataColumn_0 = new DataColumn("ว\u0e31นท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("ลำด\u0e31บ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("เลขท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			dataColumn_3 = new DataColumn("เบอร\u0e4cห\u0e49อง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			dataColumn_4 = new DataColumn("ราคาห\u0e49อง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_4);
			dataColumn_5 = new DataColumn("ราคาส\u0e34นค\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_5);
			dataColumn_6 = new DataColumn("รวม", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_6);
			dataColumn_7 = new DataColumn("เข\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_7);
			dataColumn_8 = new DataColumn("ออก", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_8);
			dataColumn_9 = new DataColumn("ส\u0e34นค\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_9);
			dataColumn_10 = new DataColumn("รวมห\u0e49อง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_10);
			dataColumn_11 = new DataColumn("รวมส\u0e34นค\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_11);
			dataColumn_12 = new DataColumn("รวมท\u0e31\u0e49งหมด", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_12);
			dataColumn_13 = new DataColumn("ช\u0e37\u0e48อ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_13);
			dataColumn_14 = new DataColumn("สถานะการจ\u0e48าย", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_14);
			dataColumn_15 = new DataColumn("พน\u0e31กงาน", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_15);
			columnBalance = new DataColumn("Balance", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnBalance);
			dataColumn_16 = new DataColumn("หมายเหต\u0e38", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_16);
			columnnote2 = new DataColumn("note2", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnnote2);
			dataColumn_17 = new DataColumn("ราคาต\u0e48อค\u0e37น", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_17);
			dataColumn_18 = new DataColumn("รวมราคาต\u0e48อค\u0e37น", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_18);
			dataColumn_19 = new DataColumn("ชำระแล\u0e49ว", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_19);
			dataColumn_20 = new DataColumn("คงค\u0e49าง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_20);
			dataColumn_21 = new DataColumn("รวมชำระแล\u0e49ว", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_21);
			dataColumn_22 = new DataColumn("รวมคงค\u0e49าง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_22);
			dataColumn_0.Caption = "DataColumn1";
			dataColumn_1.Caption = "DataColumn1";
			dataColumn_2.Caption = "DataColumn1";
			dataColumn_3.Caption = "DataColumn1";
			dataColumn_4.Caption = "DataColumn1";
			dataColumn_5.Caption = "DataColumn1";
			dataColumn_6.Caption = "DataColumn1";
			dataColumn_7.Caption = "DataColumn1";
			dataColumn_8.Caption = "DataColumn1";
			dataColumn_9.Caption = "DataColumn1";
			dataColumn_10.Caption = "DataColumn1";
			dataColumn_19.Caption = "DataColumn1";
			dataColumn_20.Caption = "DataColumn1";
			dataColumn_21.Caption = "DataColumn1";
		}

		[DebuggerNonUserCode]
		public ReportCustINRow NewReportCustINRow()
		{
			return (ReportCustINRow)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new ReportCustINRow(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(ReportCustINRow);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (ReportCustINRowChanged != null)
			{
				ReportCustINRowChanged?.Invoke(this, new ReportCustINRowChangeEvent((ReportCustINRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (ReportCustINRowChanging != null)
			{
				ReportCustINRowChanging?.Invoke(this, new ReportCustINRowChangeEvent((ReportCustINRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (ReportCustINRowDeleted != null)
			{
				ReportCustINRowDeleted?.Invoke(this, new ReportCustINRowChangeEvent((ReportCustINRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (ReportCustINRowDeleting != null)
			{
				ReportCustINRowDeleting?.Invoke(this, new ReportCustINRowChangeEvent((ReportCustINRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReportCustINRow(ReportCustINRow row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "ReportCustINDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	[XmlSchemaProvider("GetTypedTableSchema")]
	public class ReportVatDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn dataColumn_3;

		private DataColumn dataColumn_4;

		private DataColumn dataColumn_5;

		private DataColumn dataColumn_6;

		private DataColumn dataColumn_7;

		private DataColumn dataColumn_8;

		private DataColumn dataColumn_9;

		private DataColumn dataColumn_10;

		private DataColumn dataColumn_11;

		private DataColumn columnnewpage;

		private DataColumn dataColumn_12;

		private DataColumn dataColumn_13;

		private DataColumn dataColumn_14;

		private DataColumn dataColumn_15;

		private DataColumn dataColumn_16;

		private DataColumn dataColumn_17;

		private DataColumn dataColumn_18;

		private DataColumn dataColumn_19;

		private DataColumn columnvat_name;

		private DataColumn columnvat_tax;

		private DataColumn columnvat_address;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_4 => dataColumn_4;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_5 => dataColumn_5;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_6 => dataColumn_6;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_7 => dataColumn_7;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_8 => dataColumn_8;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_9 => dataColumn_9;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_10 => dataColumn_10;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_11 => dataColumn_11;

		[DebuggerNonUserCode]
		public DataColumn newpageColumn => columnnewpage;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_12 => dataColumn_12;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_13 => dataColumn_13;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_14 => dataColumn_14;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_15 => dataColumn_15;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_16 => dataColumn_16;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_17 => dataColumn_17;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_18 => dataColumn_18;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_19 => dataColumn_19;

		[DebuggerNonUserCode]
		public DataColumn vat_nameColumn => columnvat_name;

		[DebuggerNonUserCode]
		public DataColumn vat_taxColumn => columnvat_tax;

		[DebuggerNonUserCode]
		public DataColumn vat_addressColumn => columnvat_address;

		[DebuggerNonUserCode]
		[Browsable(false)]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public ReportVatRow this[int index] => (ReportVatRow)Rows[index];

		[method: DebuggerNonUserCode]
		public event ReportVatRowChangeEventHandler ReportVatRowChanging;

		[method: DebuggerNonUserCode]
		public event ReportVatRowChangeEventHandler ReportVatRowChanged;

		[method: DebuggerNonUserCode]
		public event ReportVatRowChangeEventHandler ReportVatRowDeleting;

		[method: DebuggerNonUserCode]
		public event ReportVatRowChangeEventHandler ReportVatRowDeleted;

		[DebuggerNonUserCode]
		static ReportVatDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public ReportVatDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "ReportVat";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal ReportVatDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected ReportVatDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReportVatRow(ReportVatRow row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public ReportVatRow AddReportVatRow(string string_0, string string_1, string string_2, string string_3, string string_4, string string_5, string string_6, string string_7, string string_8, string string_9, string string_10, string string_11, string newpage, string string_12, string string_13, string string_14, string string_15, string string_16, string string_17, string string_18, string string_19, string vat_name, string vat_tax, string vat_address)
		{
			ReportVatRow reportVatRow = (ReportVatRow)NewRow();
			object[] itemArray = new object[24]
			{
				string_0, string_1, string_2, string_3, string_4, string_5, string_6, string_7, string_8, string_9,
				string_10, string_11, newpage, string_12, string_13, string_14, string_15, string_16, string_17, string_18,
				string_19, vat_name, vat_tax, vat_address
			};
			reportVatRow.ItemArray = itemArray;
			Rows.Add(reportVatRow);
			return reportVatRow;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			ReportVatDataTable reportVatDataTable = (ReportVatDataTable)base.Clone();
			reportVatDataTable.InitVars();
			return reportVatDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new ReportVatDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			dataColumn_0 = base.Columns["ลำด\u0e31บ"];
			dataColumn_1 = base.Columns["ว\u0e31นท\u0e35\u0e48"];
			dataColumn_2 = base.Columns["เลขท\u0e35\u0e48"];
			dataColumn_3 = base.Columns["ช\u0e37\u0e48อ"];
			dataColumn_4 = base.Columns["ม\u0e39ลค\u0e48า1"];
			dataColumn_5 = base.Columns["ม\u0e39ลค\u0e48า2"];
			dataColumn_6 = base.Columns["ภาษ\u0e351"];
			dataColumn_7 = base.Columns["ภาษ\u0e352"];
			dataColumn_8 = base.Columns["รวมม\u0e39ลค\u0e48า1"];
			dataColumn_9 = base.Columns["รวมม\u0e39ลค\u0e48า2"];
			dataColumn_10 = base.Columns["รวมภาษ\u0e351"];
			dataColumn_11 = base.Columns["รวมภาษ\u0e352"];
			columnnewpage = base.Columns["newpage"];
			dataColumn_12 = base.Columns["เด\u0e37อน"];
			dataColumn_13 = base.Columns["ป\u0e35"];
			dataColumn_14 = base.Columns["ม\u0e39ลค\u0e48า3"];
			dataColumn_15 = base.Columns["ภาษ\u0e353"];
			dataColumn_16 = base.Columns["รวมม\u0e39ลค\u0e48า3"];
			dataColumn_17 = base.Columns["รวมภาษ\u0e3523"];
			dataColumn_18 = base.Columns["เลขประจำต\u0e31ว"];
			dataColumn_19 = base.Columns["สาขา"];
			columnvat_name = base.Columns["vat_name"];
			columnvat_tax = base.Columns["vat_tax"];
			columnvat_address = base.Columns["vat_address"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			dataColumn_0 = new DataColumn("ลำด\u0e31บ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("ว\u0e31นท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("เลขท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			dataColumn_3 = new DataColumn("ช\u0e37\u0e48อ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			dataColumn_4 = new DataColumn("ม\u0e39ลค\u0e48า1", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_4);
			dataColumn_5 = new DataColumn("ม\u0e39ลค\u0e48า2", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_5);
			dataColumn_6 = new DataColumn("ภาษ\u0e351", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_6);
			dataColumn_7 = new DataColumn("ภาษ\u0e352", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_7);
			dataColumn_8 = new DataColumn("รวมม\u0e39ลค\u0e48า1", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_8);
			dataColumn_9 = new DataColumn("รวมม\u0e39ลค\u0e48า2", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_9);
			dataColumn_10 = new DataColumn("รวมภาษ\u0e351", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_10);
			dataColumn_11 = new DataColumn("รวมภาษ\u0e352", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_11);
			columnnewpage = new DataColumn("newpage", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnnewpage);
			dataColumn_12 = new DataColumn("เด\u0e37อน", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_12);
			dataColumn_13 = new DataColumn("ป\u0e35", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_13);
			dataColumn_14 = new DataColumn("ม\u0e39ลค\u0e48า3", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_14);
			dataColumn_15 = new DataColumn("ภาษ\u0e353", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_15);
			dataColumn_16 = new DataColumn("รวมม\u0e39ลค\u0e48า3", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_16);
			dataColumn_17 = new DataColumn("รวมภาษ\u0e3523", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_17);
			dataColumn_18 = new DataColumn("เลขประจำต\u0e31ว", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_18);
			dataColumn_19 = new DataColumn("สาขา", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_19);
			columnvat_name = new DataColumn("vat_name", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnvat_name);
			columnvat_tax = new DataColumn("vat_tax", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnvat_tax);
			columnvat_address = new DataColumn("vat_address", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnvat_address);
			dataColumn_0.Caption = "DataColumn1";
			dataColumn_1.Caption = "DataColumn1";
			dataColumn_2.Caption = "DataColumn1";
			dataColumn_3.Caption = "DataColumn1";
			dataColumn_4.Caption = "DataColumn1";
			dataColumn_5.Caption = "DataColumn1";
			dataColumn_6.Caption = "DataColumn1";
			dataColumn_8.Caption = "ภาษ\u0e352";
			dataColumn_9.Caption = "ภาษ\u0e352";
			dataColumn_10.Caption = "ภาษ\u0e352";
			dataColumn_14.Caption = "DataColumn1";
			dataColumn_15.Caption = "DataColumn1";
			dataColumn_16.Caption = "DataColumn1";
		}

		[DebuggerNonUserCode]
		public ReportVatRow NewReportVatRow()
		{
			return (ReportVatRow)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new ReportVatRow(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(ReportVatRow);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (ReportVatRowChanged != null)
			{
				ReportVatRowChanged?.Invoke(this, new ReportVatRowChangeEvent((ReportVatRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (ReportVatRowChanging != null)
			{
				ReportVatRowChanging?.Invoke(this, new ReportVatRowChangeEvent((ReportVatRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (ReportVatRowDeleted != null)
			{
				ReportVatRowDeleted?.Invoke(this, new ReportVatRowChangeEvent((ReportVatRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (ReportVatRowDeleting != null)
			{
				ReportVatRowDeleting?.Invoke(this, new ReportVatRowChangeEvent((ReportVatRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReportVatRow(ReportVatRow row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "ReportVatDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[XmlSchemaProvider("GetTypedTableSchema")]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportShiftDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn dataColumn_3;

		private DataColumn dataColumn_4;

		private DataColumn dataColumn_5;

		private DataColumn dataColumn_6;

		private DataColumn dataColumn_7;

		private DataColumn dataColumn_8;

		private DataColumn dataColumn_9;

		private DataColumn dataColumn_10;

		private DataColumn dataColumn_11;

		private DataColumn dataColumn_12;

		private DataColumn dataColumn_13;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_4 => dataColumn_4;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_5 => dataColumn_5;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_6 => dataColumn_6;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_7 => dataColumn_7;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_8 => dataColumn_8;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_9 => dataColumn_9;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_10 => dataColumn_10;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_11 => dataColumn_11;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_12 => dataColumn_12;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_13 => dataColumn_13;

		[DebuggerNonUserCode]
		[Browsable(false)]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public ReportShiftRow this[int index] => (ReportShiftRow)Rows[index];

		[method: DebuggerNonUserCode]
		public event ReportShiftRowChangeEventHandler ReportShiftRowChanging;

		[method: DebuggerNonUserCode]
		public event ReportShiftRowChangeEventHandler ReportShiftRowChanged;

		[method: DebuggerNonUserCode]
		public event ReportShiftRowChangeEventHandler ReportShiftRowDeleting;

		[method: DebuggerNonUserCode]
		public event ReportShiftRowChangeEventHandler ReportShiftRowDeleted;

		[DebuggerNonUserCode]
		static ReportShiftDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public ReportShiftDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "ReportShift";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal ReportShiftDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected ReportShiftDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReportShiftRow(ReportShiftRow row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public ReportShiftRow AddReportShiftRow(string string_0, string string_1, string string_2, string string_3, string string_4, string string_5, string string_6, string string_7, string string_8, string string_9, string string_10, string string_11, string string_12, string string_13)
		{
			ReportShiftRow reportShiftRow = (ReportShiftRow)NewRow();
			object[] itemArray = new object[14]
			{
				string_0, string_1, string_2, string_3, string_4, string_5, string_6, string_7, string_8, string_9,
				string_10, string_11, string_12, string_13
			};
			reportShiftRow.ItemArray = itemArray;
			Rows.Add(reportShiftRow);
			return reportShiftRow;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			ReportShiftDataTable reportShiftDataTable = (ReportShiftDataTable)base.Clone();
			reportShiftDataTable.InitVars();
			return reportShiftDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new ReportShiftDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			dataColumn_0 = base.Columns["ว\u0e31นท\u0e35\u0e48"];
			dataColumn_1 = base.Columns["ลำด\u0e31บ"];
			dataColumn_2 = base.Columns["เลขท\u0e35\u0e48"];
			dataColumn_3 = base.Columns["เบอร\u0e4cห\u0e49อง"];
			dataColumn_4 = base.Columns["ล\u0e39กค\u0e49า"];
			dataColumn_5 = base.Columns["เข\u0e49า"];
			dataColumn_6 = base.Columns["ออก"];
			dataColumn_7 = base.Columns["ล\u0e39กหน\u0e35\u0e49"];
			dataColumn_8 = base.Columns["ม\u0e31ดจำ"];
			dataColumn_9 = base.Columns["เง\u0e34นสด"];
			dataColumn_10 = base.Columns["บ\u0e31ตร"];
			dataColumn_11 = base.Columns["จ\u0e48ายล\u0e48างหน\u0e49า"];
			dataColumn_12 = base.Columns["ค\u0e37นเง\u0e34น"];
			dataColumn_13 = base.Columns["พน\u0e31กงาน"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			dataColumn_0 = new DataColumn("ว\u0e31นท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("ลำด\u0e31บ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("เลขท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			dataColumn_3 = new DataColumn("เบอร\u0e4cห\u0e49อง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			dataColumn_4 = new DataColumn("ล\u0e39กค\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_4);
			dataColumn_5 = new DataColumn("เข\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_5);
			dataColumn_6 = new DataColumn("ออก", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_6);
			dataColumn_7 = new DataColumn("ล\u0e39กหน\u0e35\u0e49", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_7);
			dataColumn_8 = new DataColumn("ม\u0e31ดจำ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_8);
			dataColumn_9 = new DataColumn("เง\u0e34นสด", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_9);
			dataColumn_10 = new DataColumn("บ\u0e31ตร", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_10);
			dataColumn_11 = new DataColumn("จ\u0e48ายล\u0e48างหน\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_11);
			dataColumn_12 = new DataColumn("ค\u0e37นเง\u0e34น", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_12);
			dataColumn_13 = new DataColumn("พน\u0e31กงาน", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_13);
			dataColumn_1.Caption = "DataColumn1";
			dataColumn_2.Caption = "DataColumn1";
			dataColumn_3.Caption = "DataColumn1";
			dataColumn_4.Caption = "DataColumn1";
			dataColumn_5.Caption = "DataColumn1";
			dataColumn_6.Caption = "DataColumn1";
			dataColumn_7.Caption = "DataColumn1";
			dataColumn_8.Caption = "DataColumn1";
			dataColumn_9.Caption = "DataColumn1";
			dataColumn_10.Caption = "DataColumn1";
			dataColumn_11.Caption = "DataColumn1";
		}

		[DebuggerNonUserCode]
		public ReportShiftRow NewReportShiftRow()
		{
			return (ReportShiftRow)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new ReportShiftRow(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(ReportShiftRow);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (ReportShiftRowChanged != null)
			{
				ReportShiftRowChanged?.Invoke(this, new ReportShiftRowChangeEvent((ReportShiftRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (ReportShiftRowChanging != null)
			{
				ReportShiftRowChanging?.Invoke(this, new ReportShiftRowChangeEvent((ReportShiftRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (ReportShiftRowDeleted != null)
			{
				ReportShiftRowDeleted?.Invoke(this, new ReportShiftRowChangeEvent((ReportShiftRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (ReportShiftRowDeleting != null)
			{
				ReportShiftRowDeleting?.Invoke(this, new ReportShiftRowChangeEvent((ReportShiftRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReportShiftRow(ReportShiftRow row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "ReportShiftDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	[XmlSchemaProvider("GetTypedTableSchema")]
	public class ReportCuponDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn dataColumn_3;

		private DataColumn dataColumn_4;

		private DataColumn dataColumn_5;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_4 => dataColumn_4;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_5 => dataColumn_5;

		[Browsable(false)]
		[DebuggerNonUserCode]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public ReportCuponRow this[int index] => (ReportCuponRow)Rows[index];

		[method: DebuggerNonUserCode]
		public event ReportCuponRowChangeEventHandler ReportCuponRowChanging;

		[method: DebuggerNonUserCode]
		public event ReportCuponRowChangeEventHandler ReportCuponRowChanged;

		[method: DebuggerNonUserCode]
		public event ReportCuponRowChangeEventHandler ReportCuponRowDeleting;

		[method: DebuggerNonUserCode]
		public event ReportCuponRowChangeEventHandler ReportCuponRowDeleted;

		[DebuggerNonUserCode]
		static ReportCuponDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public ReportCuponDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "ReportCupon";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal ReportCuponDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected ReportCuponDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReportCuponRow(ReportCuponRow row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public ReportCuponRow AddReportCuponRow(string string_0, string string_1, string string_2, string string_3, string string_4, string string_5)
		{
			ReportCuponRow reportCuponRow = (ReportCuponRow)NewRow();
			object[] itemArray = new object[6] { string_0, string_1, string_2, string_3, string_4, string_5 };
			reportCuponRow.ItemArray = itemArray;
			Rows.Add(reportCuponRow);
			return reportCuponRow;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			ReportCuponDataTable reportCuponDataTable = (ReportCuponDataTable)base.Clone();
			reportCuponDataTable.InitVars();
			return reportCuponDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new ReportCuponDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			dataColumn_0 = base.Columns["กร\u0e38\u0e4aบ"];
			dataColumn_1 = base.Columns["เลขท\u0e35\u0e48"];
			dataColumn_2 = base.Columns["ว\u0e31นท\u0e35\u0e48ทำ"];
			dataColumn_3 = base.Columns["ว\u0e31นท\u0e35\u0e48ใช\u0e49งาน"];
			dataColumn_4 = base.Columns["ห\u0e49อง"];
			dataColumn_5 = base.Columns["ช\u0e37\u0e48อ"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			dataColumn_0 = new DataColumn("กร\u0e38\u0e4aบ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("เลขท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("ว\u0e31นท\u0e35\u0e48ทำ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			dataColumn_3 = new DataColumn("ว\u0e31นท\u0e35\u0e48ใช\u0e49งาน", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			dataColumn_4 = new DataColumn("ห\u0e49อง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_4);
			dataColumn_5 = new DataColumn("ช\u0e37\u0e48อ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_5);
		}

		[DebuggerNonUserCode]
		public ReportCuponRow NewReportCuponRow()
		{
			return (ReportCuponRow)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new ReportCuponRow(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(ReportCuponRow);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (ReportCuponRowChanged != null)
			{
				ReportCuponRowChanged?.Invoke(this, new ReportCuponRowChangeEvent((ReportCuponRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (ReportCuponRowChanging != null)
			{
				ReportCuponRowChanging?.Invoke(this, new ReportCuponRowChangeEvent((ReportCuponRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (ReportCuponRowDeleted != null)
			{
				ReportCuponRowDeleted?.Invoke(this, new ReportCuponRowChangeEvent((ReportCuponRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (ReportCuponRowDeleting != null)
			{
				ReportCuponRowDeleting?.Invoke(this, new ReportCuponRowChangeEvent((ReportCuponRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReportCuponRow(ReportCuponRow row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "ReportCuponDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	[XmlSchemaProvider("GetTypedTableSchema")]
	public class TableShiftCashDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn dataColumn_3;

		private DataColumn dataColumn_4;

		private DataColumn dataColumn_5;

		private DataColumn dataColumn_6;

		private DataColumn dataColumn_7;

		private DataColumn dataColumn_8;

		private DataColumn dataColumn_9;

		private DataColumn dataColumn_10;

		private DataColumn dataColumn_11;

		private DataColumn dataColumn_12;

		private DataColumn dataColumn_13;

		private DataColumn dataColumn_14;

		private DataColumn dataColumn_15;

		private DataColumn dataColumn_16;

		private DataColumn dataColumn_17;

		private DataColumn dataColumn_18;

		private DataColumn dataColumn_19;

		private DataColumn dataColumn_20;

		private DataColumn dataColumn_21;

		private DataColumn dataColumn_22;

		private DataColumn dataColumn_23;

		private DataColumn dataColumn_24;

		private DataColumn dataColumn_25;

		private DataColumn dataColumn_26;

		private DataColumn dataColumn_27;

		private DataColumn dataColumn_28;

		private DataColumn dataColumn_29;

		private DataColumn dataColumn_30;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_4 => dataColumn_4;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_5 => dataColumn_5;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_6 => dataColumn_6;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_7 => dataColumn_7;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_8 => dataColumn_8;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_9 => dataColumn_9;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_10 => dataColumn_10;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_11 => dataColumn_11;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_12 => dataColumn_12;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_13 => dataColumn_13;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_14 => dataColumn_14;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_15 => dataColumn_15;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_16 => dataColumn_16;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_17 => dataColumn_17;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_18 => dataColumn_18;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_19 => dataColumn_19;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_20 => dataColumn_20;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_21 => dataColumn_21;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_22 => dataColumn_22;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_23 => dataColumn_23;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_24 => dataColumn_24;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_25 => dataColumn_25;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_26 => dataColumn_26;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_27 => dataColumn_27;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_28 => dataColumn_28;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_29 => dataColumn_29;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_30 => dataColumn_30;

		[DebuggerNonUserCode]
		[Browsable(false)]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public TableShiftCashRow this[int index] => (TableShiftCashRow)Rows[index];

		[method: DebuggerNonUserCode]
		public event TableShiftCashRowChangeEventHandler TableShiftCashRowChanging;

		[method: DebuggerNonUserCode]
		public event TableShiftCashRowChangeEventHandler TableShiftCashRowChanged;

		[method: DebuggerNonUserCode]
		public event TableShiftCashRowChangeEventHandler TableShiftCashRowDeleting;

		[method: DebuggerNonUserCode]
		public event TableShiftCashRowChangeEventHandler TableShiftCashRowDeleted;

		[DebuggerNonUserCode]
		static TableShiftCashDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public TableShiftCashDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "TableShiftCash";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal TableShiftCashDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected TableShiftCashDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddTableShiftCashRow(TableShiftCashRow row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public TableShiftCashRow AddTableShiftCashRow(string string_0, string string_1, string string_2, string string_3, string string_4, string string_5, string string_6, string string_7, string string_8, string string_9, string string_10, string string_11, string string_12, string string_13, string string_14, string string_15, string string_16, string string_17, string string_18, string string_19, string string_20, string string_21, string string_22, string string_23, string string_24, string string_25, string string_26, string string_27, string string_28, string string_29, string string_30)
		{
			TableShiftCashRow tableShiftCashRow = (TableShiftCashRow)NewRow();
			object[] itemArray = new object[31]
			{
				string_0, string_1, string_2, string_3, string_4, string_5, string_6, string_7, string_8, string_9,
				string_10, string_11, string_12, string_13, string_14, string_15, string_16, string_17, string_18, string_19,
				string_20, string_21, string_22, string_23, string_24, string_25, string_26, string_27, string_28, string_29,
				string_30
			};
			tableShiftCashRow.ItemArray = itemArray;
			Rows.Add(tableShiftCashRow);
			return tableShiftCashRow;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			TableShiftCashDataTable tableShiftCashDataTable = (TableShiftCashDataTable)base.Clone();
			tableShiftCashDataTable.InitVars();
			return tableShiftCashDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new TableShiftCashDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			dataColumn_0 = base.Columns["ห\u0e31ว"];
			dataColumn_1 = base.Columns["ท\u0e35\u0e48"];
			dataColumn_2 = base.Columns["เลขท\u0e35\u0e48"];
			dataColumn_3 = base.Columns["ว\u0e31นท\u0e35\u0e48จ\u0e48าย"];
			dataColumn_4 = base.Columns["ลงทะเบ\u0e35ยน"];
			dataColumn_5 = base.Columns["เบอร\u0e4cห\u0e49อง"];
			dataColumn_6 = base.Columns["ช\u0e37\u0e48อล\u0e39กค\u0e49า"];
			dataColumn_7 = base.Columns["ว\u0e31นท\u0e35\u0e48เข\u0e49า"];
			dataColumn_8 = base.Columns["เรท"];
			dataColumn_9 = base.Columns["ราคาห\u0e49อง"];
			dataColumn_10 = base.Columns["ราคาส\u0e34นค\u0e49า"];
			dataColumn_11 = base.Columns["เง\u0e34นสด"];
			dataColumn_12 = base.Columns["บ\u0e31ตร"];
			dataColumn_13 = base.Columns["พน\u0e31กงาน"];
			dataColumn_14 = base.Columns["สร\u0e38ป"];
			dataColumn_15 = base.Columns["หน\u0e35\u0e49"];
			dataColumn_16 = base.Columns["รวมจ\u0e48ายหน\u0e35\u0e49"];
			dataColumn_17 = base.Columns["รวมสด"];
			dataColumn_18 = base.Columns["รวมบ\u0e31ตร"];
			dataColumn_19 = base.Columns["ฟร\u0e35"];
			dataColumn_20 = base.Columns["ฟร\u0e35ท\u0e31\u0e49งหมด"];
			dataColumn_21 = base.Columns["ร\u0e31บม\u0e31ดจำ"];
			dataColumn_22 = base.Columns["ค\u0e37นม\u0e31ดจำ"];
			dataColumn_23 = base.Columns["รวมเง\u0e34นท\u0e35\u0e48ต\u0e49องส\u0e48ง"];
			dataColumn_24 = base.Columns["ล\u0e34\u0e49นช\u0e31ก"];
			dataColumn_25 = base.Columns["โอนเง\u0e34น"];
			dataColumn_26 = base.Columns["โอนเง\u0e34นท\u0e31\u0e49งหมด"];
			dataColumn_27 = base.Columns["รวมเง\u0e34นท\u0e31\u0e49งหมด"];
			dataColumn_28 = base.Columns["สาขา"];
			dataColumn_29 = base.Columns["เว\u0e47บ"];
			dataColumn_30 = base.Columns["เว\u0e47ปท\u0e31\u0e49งหมด"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			dataColumn_0 = new DataColumn("ห\u0e31ว", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("ท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("เลขท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			dataColumn_3 = new DataColumn("ว\u0e31นท\u0e35\u0e48จ\u0e48าย", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			dataColumn_4 = new DataColumn("ลงทะเบ\u0e35ยน", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_4);
			dataColumn_5 = new DataColumn("เบอร\u0e4cห\u0e49อง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_5);
			dataColumn_6 = new DataColumn("ช\u0e37\u0e48อล\u0e39กค\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_6);
			dataColumn_7 = new DataColumn("ว\u0e31นท\u0e35\u0e48เข\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_7);
			dataColumn_8 = new DataColumn("เรท", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_8);
			dataColumn_9 = new DataColumn("ราคาห\u0e49อง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_9);
			dataColumn_10 = new DataColumn("ราคาส\u0e34นค\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_10);
			dataColumn_11 = new DataColumn("เง\u0e34นสด", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_11);
			dataColumn_12 = new DataColumn("บ\u0e31ตร", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_12);
			dataColumn_13 = new DataColumn("พน\u0e31กงาน", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_13);
			dataColumn_14 = new DataColumn("สร\u0e38ป", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_14);
			dataColumn_15 = new DataColumn("หน\u0e35\u0e49", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_15);
			dataColumn_16 = new DataColumn("รวมจ\u0e48ายหน\u0e35\u0e49", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_16);
			dataColumn_17 = new DataColumn("รวมสด", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_17);
			dataColumn_18 = new DataColumn("รวมบ\u0e31ตร", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_18);
			dataColumn_19 = new DataColumn("ฟร\u0e35", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_19);
			dataColumn_20 = new DataColumn("ฟร\u0e35ท\u0e31\u0e49งหมด", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_20);
			dataColumn_21 = new DataColumn("ร\u0e31บม\u0e31ดจำ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_21);
			dataColumn_22 = new DataColumn("ค\u0e37นม\u0e31ดจำ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_22);
			dataColumn_23 = new DataColumn("รวมเง\u0e34นท\u0e35\u0e48ต\u0e49องส\u0e48ง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_23);
			dataColumn_24 = new DataColumn("ล\u0e34\u0e49นช\u0e31ก", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_24);
			dataColumn_25 = new DataColumn("โอนเง\u0e34น", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_25);
			dataColumn_26 = new DataColumn("โอนเง\u0e34นท\u0e31\u0e49งหมด", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_26);
			dataColumn_27 = new DataColumn("รวมเง\u0e34นท\u0e31\u0e49งหมด", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_27);
			dataColumn_28 = new DataColumn("สาขา", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_28);
			dataColumn_29 = new DataColumn("เว\u0e47บ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_29);
			dataColumn_30 = new DataColumn("เว\u0e47ปท\u0e31\u0e49งหมด", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_30);
			dataColumn_1.Caption = "DataColumn1";
			dataColumn_2.Caption = "DataColumn1";
			dataColumn_3.Caption = "DataColumn1";
			dataColumn_4.Caption = "DataColumn1";
			dataColumn_5.Caption = "DataColumn1";
			dataColumn_6.Caption = "DataColumn1";
			dataColumn_7.Caption = "DataColumn1";
			dataColumn_8.Caption = "DataColumn1";
			dataColumn_9.Caption = "DataColumn1";
			dataColumn_10.Caption = "DataColumn1";
			dataColumn_11.Caption = "DataColumn1";
			dataColumn_12.Caption = "DataColumn1";
			dataColumn_13.Caption = "DataColumn1";
		}

		[DebuggerNonUserCode]
		public TableShiftCashRow NewTableShiftCashRow()
		{
			return (TableShiftCashRow)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new TableShiftCashRow(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(TableShiftCashRow);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (TableShiftCashRowChanged != null)
			{
				TableShiftCashRowChanged?.Invoke(this, new TableShiftCashRowChangeEvent((TableShiftCashRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (TableShiftCashRowChanging != null)
			{
				TableShiftCashRowChanging?.Invoke(this, new TableShiftCashRowChangeEvent((TableShiftCashRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (TableShiftCashRowDeleted != null)
			{
				TableShiftCashRowDeleted?.Invoke(this, new TableShiftCashRowChangeEvent((TableShiftCashRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (TableShiftCashRowDeleting != null)
			{
				TableShiftCashRowDeleting?.Invoke(this, new TableShiftCashRowChangeEvent((TableShiftCashRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveTableShiftCashRow(TableShiftCashRow row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "TableShiftCashDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	[XmlSchemaProvider("GetTypedTableSchema")]
	public class ReportBillCreditDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn dataColumn_3;

		private DataColumn dataColumn_4;

		private DataColumn dataColumn_5;

		private DataColumn dataColumn_6;

		private DataColumn dataColumn_7;

		private DataColumn dataColumn_8;

		private DataColumn dataColumn_9;

		private DataColumn dataColumn_10;

		private DataColumn dataColumn_11;

		private DataColumn dataColumn_12;

		private DataColumn dataColumn_13;

		private DataColumn dataColumn_14;

		private DataColumn dataColumn_15;

		private DataColumn dataColumn_16;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_4 => dataColumn_4;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_5 => dataColumn_5;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_6 => dataColumn_6;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_7 => dataColumn_7;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_8 => dataColumn_8;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_9 => dataColumn_9;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_10 => dataColumn_10;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_11 => dataColumn_11;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_12 => dataColumn_12;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_13 => dataColumn_13;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_14 => dataColumn_14;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_15 => dataColumn_15;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_16 => dataColumn_16;

		[Browsable(false)]
		[DebuggerNonUserCode]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public ReportBillCreditRow this[int index] => (ReportBillCreditRow)Rows[index];

		[method: DebuggerNonUserCode]
		public event ReportBillCreditRowChangeEventHandler ReportBillCreditRowChanging;

		[method: DebuggerNonUserCode]
		public event ReportBillCreditRowChangeEventHandler ReportBillCreditRowChanged;

		[method: DebuggerNonUserCode]
		public event ReportBillCreditRowChangeEventHandler ReportBillCreditRowDeleting;

		[method: DebuggerNonUserCode]
		public event ReportBillCreditRowChangeEventHandler ReportBillCreditRowDeleted;

		[DebuggerNonUserCode]
		static ReportBillCreditDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public ReportBillCreditDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "ReportBillCredit";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal ReportBillCreditDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected ReportBillCreditDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReportBillCreditRow(ReportBillCreditRow row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public ReportBillCreditRow AddReportBillCreditRow(string string_0, string string_1, string string_2, string string_3, string string_4, string string_5, string string_6, string string_7, string string_8, string string_9, string string_10, string string_11, string string_12, string string_13, string string_14, string string_15, string string_16)
		{
			ReportBillCreditRow reportBillCreditRow = (ReportBillCreditRow)NewRow();
			object[] itemArray = new object[17]
			{
				string_0, string_1, string_2, string_3, string_4, string_5, string_6, string_7, string_8, string_9,
				string_10, string_11, string_12, string_13, string_14, string_15, string_16
			};
			reportBillCreditRow.ItemArray = itemArray;
			Rows.Add(reportBillCreditRow);
			return reportBillCreditRow;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			ReportBillCreditDataTable reportBillCreditDataTable = (ReportBillCreditDataTable)base.Clone();
			reportBillCreditDataTable.InitVars();
			return reportBillCreditDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new ReportBillCreditDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			dataColumn_0 = base.Columns["เลขท\u0e35\u0e48"];
			dataColumn_1 = base.Columns["ว\u0e31นท\u0e35\u0e48"];
			dataColumn_2 = base.Columns["รห\u0e31สล\u0e39กค\u0e49า"];
			dataColumn_3 = base.Columns["ช\u0e37\u0e48อ"];
			dataColumn_4 = base.Columns["ท\u0e35\u0e48อย\u0e39\u0e48"];
			dataColumn_5 = base.Columns["โทร"];
			dataColumn_6 = base.Columns["พน\u0e31กงานขาย"];
			dataColumn_7 = base.Columns["ท\u0e35\u0e48"];
			dataColumn_8 = base.Columns["รห\u0e31สส\u0e34นค\u0e49า"];
			dataColumn_9 = base.Columns["รายการ"];
			dataColumn_10 = base.Columns["ขนาดบรรจ\u0e38"];
			dataColumn_11 = base.Columns["จำนวน"];
			dataColumn_12 = base.Columns["ราคาต\u0e48อหน\u0e48วย"];
			dataColumn_13 = base.Columns["จำนวนเง\u0e34น"];
			dataColumn_14 = base.Columns["ราคารวม"];
			dataColumn_15 = base.Columns["ราคาต\u0e31วหน\u0e31งส\u0e37อ"];
			dataColumn_16 = base.Columns["รวมจำนวน"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			dataColumn_0 = new DataColumn("เลขท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("ว\u0e31นท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("รห\u0e31สล\u0e39กค\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			dataColumn_3 = new DataColumn("ช\u0e37\u0e48อ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			dataColumn_4 = new DataColumn("ท\u0e35\u0e48อย\u0e39\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_4);
			dataColumn_5 = new DataColumn("โทร", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_5);
			dataColumn_6 = new DataColumn("พน\u0e31กงานขาย", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_6);
			dataColumn_7 = new DataColumn("ท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_7);
			dataColumn_8 = new DataColumn("รห\u0e31สส\u0e34นค\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_8);
			dataColumn_9 = new DataColumn("รายการ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_9);
			dataColumn_10 = new DataColumn("ขนาดบรรจ\u0e38", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_10);
			dataColumn_11 = new DataColumn("จำนวน", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_11);
			dataColumn_12 = new DataColumn("ราคาต\u0e48อหน\u0e48วย", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_12);
			dataColumn_13 = new DataColumn("จำนวนเง\u0e34น", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_13);
			dataColumn_14 = new DataColumn("ราคารวม", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_14);
			dataColumn_15 = new DataColumn("ราคาต\u0e31วหน\u0e31งส\u0e37อ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_15);
			dataColumn_16 = new DataColumn("รวมจำนวน", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_16);
		}

		[DebuggerNonUserCode]
		public ReportBillCreditRow NewReportBillCreditRow()
		{
			return (ReportBillCreditRow)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new ReportBillCreditRow(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(ReportBillCreditRow);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (ReportBillCreditRowChanged != null)
			{
				ReportBillCreditRowChanged?.Invoke(this, new ReportBillCreditRowChangeEvent((ReportBillCreditRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (ReportBillCreditRowChanging != null)
			{
				ReportBillCreditRowChanging?.Invoke(this, new ReportBillCreditRowChangeEvent((ReportBillCreditRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (ReportBillCreditRowDeleted != null)
			{
				ReportBillCreditRowDeleted?.Invoke(this, new ReportBillCreditRowChangeEvent((ReportBillCreditRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (ReportBillCreditRowDeleting != null)
			{
				ReportBillCreditRowDeleting?.Invoke(this, new ReportBillCreditRowChangeEvent((ReportBillCreditRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReportBillCreditRow(ReportBillCreditRow row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "ReportBillCreditDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[XmlSchemaProvider("GetTypedTableSchema")]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class Report_Room_allDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn columnhead;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn dataColumn_3;

		private DataColumn dataColumn_4;

		private DataColumn dataColumn_5;

		[DebuggerNonUserCode]
		public DataColumn headColumn => columnhead;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_4 => dataColumn_4;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_5 => dataColumn_5;

		[DebuggerNonUserCode]
		[Browsable(false)]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public Report_Room_allRow this[int index] => (Report_Room_allRow)Rows[index];

		[method: DebuggerNonUserCode]
		public event Report_Room_allRowChangeEventHandler Report_Room_allRowChanging;

		[method: DebuggerNonUserCode]
		public event Report_Room_allRowChangeEventHandler Report_Room_allRowChanged;

		[method: DebuggerNonUserCode]
		public event Report_Room_allRowChangeEventHandler Report_Room_allRowDeleting;

		[method: DebuggerNonUserCode]
		public event Report_Room_allRowChangeEventHandler Report_Room_allRowDeleted;

		[DebuggerNonUserCode]
		static Report_Room_allDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public Report_Room_allDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "Report_Room_all";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal Report_Room_allDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected Report_Room_allDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReport_Room_allRow(Report_Room_allRow row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public Report_Room_allRow AddReport_Room_allRow(string head, string string_0, string string_1, string string_2, string string_3, string string_4, string string_5)
		{
			Report_Room_allRow report_Room_allRow = (Report_Room_allRow)NewRow();
			object[] itemArray = new object[7] { head, string_0, string_1, string_2, string_3, string_4, string_5 };
			report_Room_allRow.ItemArray = itemArray;
			Rows.Add(report_Room_allRow);
			return report_Room_allRow;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			Report_Room_allDataTable report_Room_allDataTable = (Report_Room_allDataTable)base.Clone();
			report_Room_allDataTable.InitVars();
			return report_Room_allDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new Report_Room_allDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			columnhead = base.Columns["head"];
			dataColumn_0 = base.Columns["กร\u0e38\u0e4aป"];
			dataColumn_1 = base.Columns["รายการ"];
			dataColumn_2 = base.Columns["ว\u0e31น"];
			dataColumn_3 = base.Columns["เด\u0e37อน"];
			dataColumn_4 = base.Columns["ป\u0e35"];
			dataColumn_5 = base.Columns["ป\u0e35ท\u0e35\u0e48แล\u0e49ว"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			columnhead = new DataColumn("head", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnhead);
			dataColumn_0 = new DataColumn("กร\u0e38\u0e4aป", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("รายการ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("ว\u0e31น", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			dataColumn_3 = new DataColumn("เด\u0e37อน", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			dataColumn_4 = new DataColumn("ป\u0e35", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_4);
			dataColumn_5 = new DataColumn("ป\u0e35ท\u0e35\u0e48แล\u0e49ว", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_5);
			columnhead.Caption = "DataColumn1";
			dataColumn_0.Caption = "DataColumn1";
			dataColumn_1.Caption = "DataColumn1";
			dataColumn_2.Caption = "DataColumn1";
			dataColumn_3.Caption = "DataColumn1";
			dataColumn_4.Caption = "DataColumn1";
		}

		[DebuggerNonUserCode]
		public Report_Room_allRow NewReport_Room_allRow()
		{
			return (Report_Room_allRow)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new Report_Room_allRow(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(Report_Room_allRow);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (Report_Room_allRowChanged != null)
			{
				Report_Room_allRowChanged?.Invoke(this, new Report_Room_allRowChangeEvent((Report_Room_allRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (Report_Room_allRowChanging != null)
			{
				Report_Room_allRowChanging?.Invoke(this, new Report_Room_allRowChangeEvent((Report_Room_allRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (Report_Room_allRowDeleted != null)
			{
				Report_Room_allRowDeleted?.Invoke(this, new Report_Room_allRowChangeEvent((Report_Room_allRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (Report_Room_allRowDeleting != null)
			{
				Report_Room_allRowDeleting?.Invoke(this, new Report_Room_allRowChangeEvent((Report_Room_allRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReport_Room_allRow(Report_Room_allRow row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "Report_Room_allDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	[XmlSchemaProvider("GetTypedTableSchema")]
	public class Report_Debt_INVDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn dataColumn_3;

		private DataColumn dataColumn_4;

		private DataColumn dataColumn_5;

		private DataColumn dataColumn_6;

		private DataColumn dataColumn_7;

		private DataColumn dataColumn_8;

		private DataColumn dataColumn_9;

		private DataColumn dataColumn_10;

		private DataColumn dataColumn_11;

		private DataColumn dataColumn_12;

		private DataColumn dataColumn_13;

		private DataColumn dataColumn_14;

		private DataColumn dataColumn_15;

		private DataColumn dataColumn_16;

		private DataColumn dataColumn_17;

		private DataColumn dataColumn_18;

		private DataColumn dataColumn_19;

		private DataColumn dataColumn_20;

		private DataColumn dataColumn_21;

		private DataColumn dataColumn_22;

		private DataColumn dataColumn_23;

		private DataColumn dataColumn_24;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_4 => dataColumn_4;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_5 => dataColumn_5;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_6 => dataColumn_6;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_7 => dataColumn_7;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_8 => dataColumn_8;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_9 => dataColumn_9;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_10 => dataColumn_10;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_11 => dataColumn_11;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_12 => dataColumn_12;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_13 => dataColumn_13;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_14 => dataColumn_14;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_15 => dataColumn_15;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_16 => dataColumn_16;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_17 => dataColumn_17;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_18 => dataColumn_18;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_19 => dataColumn_19;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_20 => dataColumn_20;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_21 => dataColumn_21;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_22 => dataColumn_22;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_23 => dataColumn_23;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_24 => dataColumn_24;

		[Browsable(false)]
		[DebuggerNonUserCode]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public Report_Debt_INVRow this[int index] => (Report_Debt_INVRow)Rows[index];

		[method: DebuggerNonUserCode]
		public event Report_Debt_INVRowChangeEventHandler Report_Debt_INVRowChanging;

		[method: DebuggerNonUserCode]
		public event Report_Debt_INVRowChangeEventHandler Report_Debt_INVRowChanged;

		[method: DebuggerNonUserCode]
		public event Report_Debt_INVRowChangeEventHandler Report_Debt_INVRowDeleting;

		[method: DebuggerNonUserCode]
		public event Report_Debt_INVRowChangeEventHandler Report_Debt_INVRowDeleted;

		[DebuggerNonUserCode]
		static Report_Debt_INVDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public Report_Debt_INVDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "Report_Debt_INV";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal Report_Debt_INVDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected Report_Debt_INVDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReport_Debt_INVRow(Report_Debt_INVRow row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public Report_Debt_INVRow AddReport_Debt_INVRow(string string_0, string string_1, string string_2, string string_3, string string_4, string string_5, string string_6, string string_7, string string_8, string string_9, string string_10, string string_11, string string_12, string string_13, string string_14, string string_15, string string_16, string string_17, string string_18, string string_19, string string_20, string string_21, string string_22, string string_23, string string_24)
		{
			Report_Debt_INVRow report_Debt_INVRow = (Report_Debt_INVRow)NewRow();
			object[] itemArray = new object[25]
			{
				string_0, string_1, string_2, string_3, string_4, string_5, string_6, string_7, string_8, string_9,
				string_10, string_11, string_12, string_13, string_14, string_15, string_16, string_17, string_18, string_19,
				string_20, string_21, string_22, string_23, string_24
			};
			report_Debt_INVRow.ItemArray = itemArray;
			Rows.Add(report_Debt_INVRow);
			return report_Debt_INVRow;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			Report_Debt_INVDataTable report_Debt_INVDataTable = (Report_Debt_INVDataTable)base.Clone();
			report_Debt_INVDataTable.InitVars();
			return report_Debt_INVDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new Report_Debt_INVDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			dataColumn_0 = base.Columns["เลขท\u0e35\u0e48"];
			dataColumn_1 = base.Columns["ว\u0e31นท\u0e35\u0e48"];
			dataColumn_2 = base.Columns["หมายเหลขห\u0e49อง"];
			dataColumn_3 = base.Columns["ช\u0e37\u0e48อ"];
			dataColumn_4 = base.Columns["ท\u0e35\u0e48อย\u0e39\u0e48"];
			dataColumn_5 = base.Columns["โทร"];
			dataColumn_6 = base.Columns["ท\u0e35\u0e48"];
			dataColumn_7 = base.Columns["รายการ"];
			dataColumn_8 = base.Columns["จำนวน"];
			dataColumn_9 = base.Columns["หน\u0e48วย"];
			dataColumn_10 = base.Columns["ราคา"];
			dataColumn_11 = base.Columns["ราคารวม"];
			dataColumn_12 = base.Columns["ชำระแล\u0e49ว"];
			dataColumn_13 = base.Columns["ค\u0e49างชำระ"];
			dataColumn_14 = base.Columns["รวมค\u0e49างชำระ"];
			dataColumn_15 = base.Columns["รวมราคาท\u0e31\u0e49งหมด"];
			dataColumn_16 = base.Columns["รวมชำระแล\u0e49ว"];
			dataColumn_17 = base.Columns["หมายเหต\u0e38"];
			dataColumn_18 = base.Columns["เง\u0e34นอ\u0e31กษร"];
			dataColumn_19 = base.Columns["ว\u0e31นออก"];
			dataColumn_20 = base.Columns["ภาษ\u0e35per"];
			dataColumn_21 = base.Columns["ราคาภาษ\u0e35"];
			dataColumn_22 = base.Columns["ราคาก\u0e48อนภาษ\u0e35"];
			dataColumn_23 = base.Columns["จ\u0e48ายคร\u0e31\u0e49งน\u0e35\u0e49"];
			dataColumn_24 = base.Columns["รวมจ\u0e48ายคร\u0e31\u0e49งน\u0e35\u0e49"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			dataColumn_0 = new DataColumn("เลขท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("ว\u0e31นท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("หมายเหลขห\u0e49อง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			dataColumn_3 = new DataColumn("ช\u0e37\u0e48อ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			dataColumn_4 = new DataColumn("ท\u0e35\u0e48อย\u0e39\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_4);
			dataColumn_5 = new DataColumn("โทร", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_5);
			dataColumn_6 = new DataColumn("ท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_6);
			dataColumn_7 = new DataColumn("รายการ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_7);
			dataColumn_8 = new DataColumn("จำนวน", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_8);
			dataColumn_9 = new DataColumn("หน\u0e48วย", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_9);
			dataColumn_10 = new DataColumn("ราคา", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_10);
			dataColumn_11 = new DataColumn("ราคารวม", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_11);
			dataColumn_12 = new DataColumn("ชำระแล\u0e49ว", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_12);
			dataColumn_13 = new DataColumn("ค\u0e49างชำระ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_13);
			dataColumn_14 = new DataColumn("รวมค\u0e49างชำระ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_14);
			dataColumn_15 = new DataColumn("รวมราคาท\u0e31\u0e49งหมด", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_15);
			dataColumn_16 = new DataColumn("รวมชำระแล\u0e49ว", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_16);
			dataColumn_17 = new DataColumn("หมายเหต\u0e38", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_17);
			dataColumn_18 = new DataColumn("เง\u0e34นอ\u0e31กษร", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_18);
			dataColumn_19 = new DataColumn("ว\u0e31นออก", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_19);
			dataColumn_20 = new DataColumn("ภาษ\u0e35per", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_20);
			dataColumn_21 = new DataColumn("ราคาภาษ\u0e35", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_21);
			dataColumn_22 = new DataColumn("ราคาก\u0e48อนภาษ\u0e35", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_22);
			dataColumn_23 = new DataColumn("จ\u0e48ายคร\u0e31\u0e49งน\u0e35\u0e49", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_23);
			dataColumn_24 = new DataColumn("รวมจ\u0e48ายคร\u0e31\u0e49งน\u0e35\u0e49", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_24);
			dataColumn_0.Caption = "DataColumn1";
			dataColumn_1.Caption = "DataColumn1";
			dataColumn_3.Caption = "DataColumn1";
			dataColumn_4.Caption = "DataColumn1";
			dataColumn_5.Caption = "DataColumn1";
			dataColumn_7.Caption = "DataColumn1";
			dataColumn_8.Caption = "DataColumn1";
			dataColumn_9.Caption = "DataColumn1";
			dataColumn_10.Caption = "DataColumn1";
			dataColumn_11.Caption = "DataColumn1";
			dataColumn_12.Caption = "DataColumn1";
			dataColumn_13.Caption = "DataColumn1";
			dataColumn_14.Caption = "DataColumn1";
		}

		[DebuggerNonUserCode]
		public Report_Debt_INVRow NewReport_Debt_INVRow()
		{
			return (Report_Debt_INVRow)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new Report_Debt_INVRow(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(Report_Debt_INVRow);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (Report_Debt_INVRowChanged != null)
			{
				Report_Debt_INVRowChanged?.Invoke(this, new Report_Debt_INVRowChangeEvent((Report_Debt_INVRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (Report_Debt_INVRowChanging != null)
			{
				Report_Debt_INVRowChanging?.Invoke(this, new Report_Debt_INVRowChangeEvent((Report_Debt_INVRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (Report_Debt_INVRowDeleted != null)
			{
				Report_Debt_INVRowDeleted?.Invoke(this, new Report_Debt_INVRowChangeEvent((Report_Debt_INVRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (Report_Debt_INVRowDeleting != null)
			{
				Report_Debt_INVRowDeleting?.Invoke(this, new Report_Debt_INVRowChangeEvent((Report_Debt_INVRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReport_Debt_INVRow(Report_Debt_INVRow row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "Report_Debt_INVDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[XmlSchemaProvider("GetTypedTableSchema")]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportFolio1DataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn dataColumn_3;

		private DataColumn dataColumn_4;

		private DataColumn dataColumn_5;

		private DataColumn dataColumn_6;

		private DataColumn dataColumn_7;

		private DataColumn dataColumn_8;

		private DataColumn dataColumn_9;

		private DataColumn dataColumn_10;

		private DataColumn dataColumn_11;

		private DataColumn dataColumn_12;

		private DataColumn dataColumn_13;

		private DataColumn dataColumn_14;

		private DataColumn dataColumn_15;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_4 => dataColumn_4;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_5 => dataColumn_5;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_6 => dataColumn_6;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_7 => dataColumn_7;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_8 => dataColumn_8;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_9 => dataColumn_9;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_10 => dataColumn_10;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_11 => dataColumn_11;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_12 => dataColumn_12;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_13 => dataColumn_13;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_14 => dataColumn_14;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_15 => dataColumn_15;

		[Browsable(false)]
		[DebuggerNonUserCode]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public ReportFolio1Row this[int index] => (ReportFolio1Row)Rows[index];

		[method: DebuggerNonUserCode]
		public event ReportFolio1RowChangeEventHandler ReportFolio1RowChanging;

		[method: DebuggerNonUserCode]
		public event ReportFolio1RowChangeEventHandler ReportFolio1RowChanged;

		[method: DebuggerNonUserCode]
		public event ReportFolio1RowChangeEventHandler ReportFolio1RowDeleting;

		[method: DebuggerNonUserCode]
		public event ReportFolio1RowChangeEventHandler ReportFolio1RowDeleted;

		[DebuggerNonUserCode]
		static ReportFolio1DataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public ReportFolio1DataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "ReportFolio1";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal ReportFolio1DataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected ReportFolio1DataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReportFolio1Row(ReportFolio1Row row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public ReportFolio1Row AddReportFolio1Row(string string_0, string string_1, string string_2, string string_3, string string_4, string string_5, string string_6, string string_7, string string_8, string string_9, string string_10, string string_11, string string_12, string string_13, string string_14, string string_15)
		{
			ReportFolio1Row reportFolio1Row = (ReportFolio1Row)NewRow();
			object[] itemArray = new object[16]
			{
				string_0, string_1, string_2, string_3, string_4, string_5, string_6, string_7, string_8, string_9,
				string_10, string_11, string_12, string_13, string_14, string_15
			};
			reportFolio1Row.ItemArray = itemArray;
			Rows.Add(reportFolio1Row);
			return reportFolio1Row;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			ReportFolio1DataTable reportFolio1DataTable = (ReportFolio1DataTable)base.Clone();
			reportFolio1DataTable.InitVars();
			return reportFolio1DataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new ReportFolio1DataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			dataColumn_0 = base.Columns["ช\u0e37\u0e48อ"];
			dataColumn_1 = base.Columns["จำนวนคน"];
			dataColumn_2 = base.Columns["อ\u0e31ตรค\u0e48าเช\u0e48า"];
			dataColumn_3 = base.Columns["อ\u0e31ตราภาษ\u0e35"];
			dataColumn_4 = base.Columns["เลขห\u0e49อง"];
			dataColumn_5 = base.Columns["ว\u0e31นท\u0e35\u0e48เข\u0e49า"];
			dataColumn_6 = base.Columns["ว\u0e31นท\u0e35\u0e48ออก"];
			dataColumn_7 = base.Columns["จำนวนว\u0e31น"];
			dataColumn_8 = base.Columns["อ\u0e31ตราค\u0e48าเช\u0e48า"];
			dataColumn_9 = base.Columns["จำนวนเง\u0e34น"];
			dataColumn_10 = base.Columns["รวมเง\u0e34นท\u0e31\u0e49งหมด"];
			dataColumn_11 = base.Columns["เลขin"];
			dataColumn_12 = base.Columns["ภาษ\u0e35PER"];
			dataColumn_13 = base.Columns["ราคาภาษ\u0e35"];
			dataColumn_14 = base.Columns["ราคาก\u0e48อนvat"];
			dataColumn_15 = base.Columns["ต\u0e31วอ\u0e31กษร"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			dataColumn_0 = new DataColumn("ช\u0e37\u0e48อ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("จำนวนคน", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("อ\u0e31ตรค\u0e48าเช\u0e48า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			dataColumn_3 = new DataColumn("อ\u0e31ตราภาษ\u0e35", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			dataColumn_4 = new DataColumn("เลขห\u0e49อง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_4);
			dataColumn_5 = new DataColumn("ว\u0e31นท\u0e35\u0e48เข\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_5);
			dataColumn_6 = new DataColumn("ว\u0e31นท\u0e35\u0e48ออก", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_6);
			dataColumn_7 = new DataColumn("จำนวนว\u0e31น", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_7);
			dataColumn_8 = new DataColumn("อ\u0e31ตราค\u0e48าเช\u0e48า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_8);
			dataColumn_9 = new DataColumn("จำนวนเง\u0e34น", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_9);
			dataColumn_10 = new DataColumn("รวมเง\u0e34นท\u0e31\u0e49งหมด", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_10);
			dataColumn_11 = new DataColumn("เลขin", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_11);
			dataColumn_12 = new DataColumn("ภาษ\u0e35PER", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_12);
			dataColumn_13 = new DataColumn("ราคาภาษ\u0e35", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_13);
			dataColumn_14 = new DataColumn("ราคาก\u0e48อนvat", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_14);
			dataColumn_15 = new DataColumn("ต\u0e31วอ\u0e31กษร", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_15);
			dataColumn_0.Caption = "DataColumn1";
			dataColumn_1.Caption = "DataColumn1";
			dataColumn_2.Caption = "DataColumn1";
			dataColumn_3.Caption = "DataColumn1";
			dataColumn_4.Caption = "DataColumn1";
			dataColumn_5.Caption = "DataColumn1";
			dataColumn_6.Caption = "DataColumn1";
			dataColumn_7.Caption = "DataColumn1";
			dataColumn_8.Caption = "DataColumn1";
			dataColumn_9.Caption = "DataColumn1";
		}

		[DebuggerNonUserCode]
		public ReportFolio1Row NewReportFolio1Row()
		{
			return (ReportFolio1Row)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new ReportFolio1Row(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(ReportFolio1Row);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (ReportFolio1RowChanged != null)
			{
				ReportFolio1RowChanged?.Invoke(this, new ReportFolio1RowChangeEvent((ReportFolio1Row)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (ReportFolio1RowChanging != null)
			{
				ReportFolio1RowChanging?.Invoke(this, new ReportFolio1RowChangeEvent((ReportFolio1Row)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (ReportFolio1RowDeleted != null)
			{
				ReportFolio1RowDeleted?.Invoke(this, new ReportFolio1RowChangeEvent((ReportFolio1Row)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (ReportFolio1RowDeleting != null)
			{
				ReportFolio1RowDeleting?.Invoke(this, new ReportFolio1RowChangeEvent((ReportFolio1Row)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReportFolio1Row(ReportFolio1Row row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "ReportFolio1DataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	[XmlSchemaProvider("GetTypedTableSchema")]
	public class ReportFolio1_2DataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		[Browsable(false)]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public ReportFolio1_2Row this[int index] => (ReportFolio1_2Row)Rows[index];

		[method: DebuggerNonUserCode]
		public event ReportFolio1_2RowChangeEventHandler ReportFolio1_2RowChanging;

		[method: DebuggerNonUserCode]
		public event ReportFolio1_2RowChangeEventHandler ReportFolio1_2RowChanged;

		[method: DebuggerNonUserCode]
		public event ReportFolio1_2RowChangeEventHandler ReportFolio1_2RowDeleting;

		[method: DebuggerNonUserCode]
		public event ReportFolio1_2RowChangeEventHandler ReportFolio1_2RowDeleted;

		[DebuggerNonUserCode]
		static ReportFolio1_2DataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public ReportFolio1_2DataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "ReportFolio1_2";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal ReportFolio1_2DataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected ReportFolio1_2DataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReportFolio1_2Row(ReportFolio1_2Row row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public ReportFolio1_2Row AddReportFolio1_2Row(string string_0, string string_1, string string_2, string string_3)
		{
			ReportFolio1_2Row reportFolio1_2Row = (ReportFolio1_2Row)NewRow();
			object[] itemArray = new object[4] { string_0, string_1, string_2, string_3 };
			reportFolio1_2Row.ItemArray = itemArray;
			Rows.Add(reportFolio1_2Row);
			return reportFolio1_2Row;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			ReportFolio1_2DataTable reportFolio1_2DataTable = (ReportFolio1_2DataTable)base.Clone();
			reportFolio1_2DataTable.InitVars();
			return reportFolio1_2DataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new ReportFolio1_2DataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			dataColumn_0 = base.Columns["ส\u0e34นค\u0e49า"];
			dataColumn_1 = base.Columns["จำนวน"];
			dataColumn_2 = base.Columns["ราคา"];
			dataColumn_3 = base.Columns["ราคารวม"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			dataColumn_0 = new DataColumn("ส\u0e34นค\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("จำนวน", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("ราคา", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			dataColumn_3 = new DataColumn("ราคารวม", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			dataColumn_0.Caption = "DataColumn1";
			dataColumn_1.Caption = "DataColumn1";
			dataColumn_2.Caption = "DataColumn1";
			dataColumn_3.Caption = "DataColumn1";
		}

		[DebuggerNonUserCode]
		public ReportFolio1_2Row NewReportFolio1_2Row()
		{
			return (ReportFolio1_2Row)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new ReportFolio1_2Row(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(ReportFolio1_2Row);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (ReportFolio1_2RowChanged != null)
			{
				ReportFolio1_2RowChanged?.Invoke(this, new ReportFolio1_2RowChangeEvent((ReportFolio1_2Row)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (ReportFolio1_2RowChanging != null)
			{
				ReportFolio1_2RowChanging?.Invoke(this, new ReportFolio1_2RowChangeEvent((ReportFolio1_2Row)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (ReportFolio1_2RowDeleted != null)
			{
				ReportFolio1_2RowDeleted?.Invoke(this, new ReportFolio1_2RowChangeEvent((ReportFolio1_2Row)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (ReportFolio1_2RowDeleting != null)
			{
				ReportFolio1_2RowDeleting?.Invoke(this, new ReportFolio1_2RowChangeEvent((ReportFolio1_2Row)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReportFolio1_2Row(ReportFolio1_2Row row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "ReportFolio1_2DataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[XmlSchemaProvider("GetTypedTableSchema")]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportFolio2DataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn dataColumn_3;

		private DataColumn dataColumn_4;

		private DataColumn dataColumn_5;

		private DataColumn dataColumn_6;

		private DataColumn dataColumn_7;

		private DataColumn dataColumn_8;

		private DataColumn dataColumn_9;

		private DataColumn dataColumn_10;

		private DataColumn dataColumn_11;

		private DataColumn dataColumn_12;

		private DataColumn dataColumn_13;

		private DataColumn dataColumn_14;

		private DataColumn dataColumn_15;

		private DataColumn dataColumn_16;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_4 => dataColumn_4;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_5 => dataColumn_5;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_6 => dataColumn_6;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_7 => dataColumn_7;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_8 => dataColumn_8;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_9 => dataColumn_9;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_10 => dataColumn_10;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_11 => dataColumn_11;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_12 => dataColumn_12;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_13 => dataColumn_13;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_14 => dataColumn_14;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_15 => dataColumn_15;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_16 => dataColumn_16;

		[Browsable(false)]
		[DebuggerNonUserCode]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public ReportFolio2Row this[int index] => (ReportFolio2Row)Rows[index];

		[method: DebuggerNonUserCode]
		public event ReportFolio2RowChangeEventHandler ReportFolio2RowChanging;

		[method: DebuggerNonUserCode]
		public event ReportFolio2RowChangeEventHandler ReportFolio2RowChanged;

		[method: DebuggerNonUserCode]
		public event ReportFolio2RowChangeEventHandler ReportFolio2RowDeleting;

		[method: DebuggerNonUserCode]
		public event ReportFolio2RowChangeEventHandler ReportFolio2RowDeleted;

		[DebuggerNonUserCode]
		static ReportFolio2DataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public ReportFolio2DataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "ReportFolio2";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal ReportFolio2DataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected ReportFolio2DataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReportFolio2Row(ReportFolio2Row row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public ReportFolio2Row AddReportFolio2Row(string string_0, string string_1, string string_2, string string_3, string string_4, string string_5, string string_6, string string_7, string string_8, string string_9, string string_10, string string_11, string string_12, string string_13, string string_14, string string_15, string string_16)
		{
			ReportFolio2Row reportFolio2Row = (ReportFolio2Row)NewRow();
			object[] itemArray = new object[17]
			{
				string_0, string_1, string_2, string_3, string_4, string_5, string_6, string_7, string_8, string_9,
				string_10, string_11, string_12, string_13, string_14, string_15, string_16
			};
			reportFolio2Row.ItemArray = itemArray;
			Rows.Add(reportFolio2Row);
			return reportFolio2Row;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			ReportFolio2DataTable reportFolio2DataTable = (ReportFolio2DataTable)base.Clone();
			reportFolio2DataTable.InitVars();
			return reportFolio2DataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new ReportFolio2DataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			dataColumn_0 = base.Columns["ห\u0e31ว1"];
			dataColumn_1 = base.Columns["ห\u0e31ว2"];
			dataColumn_2 = base.Columns["ห\u0e31ว3"];
			dataColumn_3 = base.Columns["ลำด\u0e31บท\u0e35\u0e48"];
			dataColumn_4 = base.Columns["เลขห\u0e49อง"];
			dataColumn_5 = base.Columns["ช\u0e37\u0e48อ"];
			dataColumn_6 = base.Columns["เข\u0e49า"];
			dataColumn_7 = base.Columns["ออก"];
			dataColumn_8 = base.Columns["ค\u0e37น"];
			dataColumn_9 = base.Columns["ราคา"];
			dataColumn_10 = base.Columns["ราคารวม"];
			dataColumn_11 = base.Columns["ราคารวมท\u0e31\u0e49งหมด"];
			dataColumn_12 = base.Columns["เลขin"];
			dataColumn_13 = base.Columns["ภาษ\u0e35PER"];
			dataColumn_14 = base.Columns["ราคาภาษ\u0e35"];
			dataColumn_15 = base.Columns["ราคาก\u0e48อนvat"];
			dataColumn_16 = base.Columns["ต\u0e31วอ\u0e31กษร"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			dataColumn_0 = new DataColumn("ห\u0e31ว1", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("ห\u0e31ว2", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("ห\u0e31ว3", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			dataColumn_3 = new DataColumn("ลำด\u0e31บท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			dataColumn_4 = new DataColumn("เลขห\u0e49อง", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_4);
			dataColumn_5 = new DataColumn("ช\u0e37\u0e48อ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_5);
			dataColumn_6 = new DataColumn("เข\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_6);
			dataColumn_7 = new DataColumn("ออก", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_7);
			dataColumn_8 = new DataColumn("ค\u0e37น", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_8);
			dataColumn_9 = new DataColumn("ราคา", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_9);
			dataColumn_10 = new DataColumn("ราคารวม", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_10);
			dataColumn_11 = new DataColumn("ราคารวมท\u0e31\u0e49งหมด", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_11);
			dataColumn_12 = new DataColumn("เลขin", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_12);
			dataColumn_13 = new DataColumn("ภาษ\u0e35PER", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_13);
			dataColumn_14 = new DataColumn("ราคาภาษ\u0e35", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_14);
			dataColumn_15 = new DataColumn("ราคาก\u0e48อนvat", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_15);
			dataColumn_16 = new DataColumn("ต\u0e31วอ\u0e31กษร", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_16);
			dataColumn_0.Caption = "DataColumn1";
			dataColumn_1.Caption = "DataColumn1";
			dataColumn_2.Caption = "DataColumn1";
			dataColumn_3.Caption = "DataColumn1";
			dataColumn_4.Caption = "DataColumn1";
			dataColumn_5.Caption = "DataColumn1";
			dataColumn_6.Caption = "DataColumn1";
			dataColumn_7.Caption = "DataColumn1";
			dataColumn_8.Caption = "DataColumn1";
			dataColumn_9.Caption = "DataColumn1";
			dataColumn_10.Caption = "รวมเง\u0e34นท\u0e31\u0e49งหมด";
		}

		[DebuggerNonUserCode]
		public ReportFolio2Row NewReportFolio2Row()
		{
			return (ReportFolio2Row)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new ReportFolio2Row(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(ReportFolio2Row);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (ReportFolio2RowChanged != null)
			{
				ReportFolio2RowChanged?.Invoke(this, new ReportFolio2RowChangeEvent((ReportFolio2Row)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (ReportFolio2RowChanging != null)
			{
				ReportFolio2RowChanging?.Invoke(this, new ReportFolio2RowChangeEvent((ReportFolio2Row)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (ReportFolio2RowDeleted != null)
			{
				ReportFolio2RowDeleted?.Invoke(this, new ReportFolio2RowChangeEvent((ReportFolio2Row)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (ReportFolio2RowDeleting != null)
			{
				ReportFolio2RowDeleting?.Invoke(this, new ReportFolio2RowChangeEvent((ReportFolio2Row)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReportFolio2Row(ReportFolio2Row row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "ReportFolio2DataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[XmlSchemaProvider("GetTypedTableSchema")]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportSaleDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn columnName;

		private DataColumn columnnum;

		private DataColumn dataColumn_2;

		private DataColumn dataColumn_3;

		private DataColumn dataColumn_4;

		private DataColumn dataColumn_5;

		private DataColumn dataColumn_6;

		private DataColumn dataColumn_7;

		private DataColumn dataColumn_8;

		private DataColumn dataColumn_9;

		private DataColumn dataColumn_10;

		private DataColumn dataColumn_11;

		private DataColumn dataColumn_12;

		private DataColumn dataColumn_13;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn NameColumn => columnName;

		[DebuggerNonUserCode]
		public DataColumn numColumn => columnnum;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_4 => dataColumn_4;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_5 => dataColumn_5;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_6 => dataColumn_6;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_7 => dataColumn_7;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_8 => dataColumn_8;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_9 => dataColumn_9;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_10 => dataColumn_10;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_11 => dataColumn_11;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_12 => dataColumn_12;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_13 => dataColumn_13;

		[DebuggerNonUserCode]
		[Browsable(false)]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public ReportSaleRow this[int index] => (ReportSaleRow)Rows[index];

		[method: DebuggerNonUserCode]
		public event ReportSaleRowChangeEventHandler ReportSaleRowChanging;

		[method: DebuggerNonUserCode]
		public event ReportSaleRowChangeEventHandler ReportSaleRowChanged;

		[method: DebuggerNonUserCode]
		public event ReportSaleRowChangeEventHandler ReportSaleRowDeleting;

		[method: DebuggerNonUserCode]
		public event ReportSaleRowChangeEventHandler ReportSaleRowDeleted;

		[DebuggerNonUserCode]
		static ReportSaleDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public ReportSaleDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "ReportSale";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal ReportSaleDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected ReportSaleDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReportSaleRow(ReportSaleRow row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public ReportSaleRow AddReportSaleRow(string string_0, string string_1, string Name, string num, string string_2, string string_3, string string_4, string string_5, string string_6, string string_7, string string_8, string string_9, string string_10, string string_11, string string_12, string string_13)
		{
			ReportSaleRow reportSaleRow = (ReportSaleRow)NewRow();
			object[] itemArray = new object[16]
			{
				string_0, string_1, Name, num, string_2, string_3, string_4, string_5, string_6, string_7,
				string_8, string_9, string_10, string_11, string_12, string_13
			};
			reportSaleRow.ItemArray = itemArray;
			Rows.Add(reportSaleRow);
			return reportSaleRow;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			ReportSaleDataTable reportSaleDataTable = (ReportSaleDataTable)base.Clone();
			reportSaleDataTable.InitVars();
			return reportSaleDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new ReportSaleDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			dataColumn_0 = base.Columns["ลำด\u0e31บ"];
			dataColumn_1 = base.Columns["รห\u0e31ส"];
			columnName = base.Columns["Name"];
			columnnum = base.Columns["num"];
			dataColumn_2 = base.Columns["ราคา"];
			dataColumn_3 = base.Columns["ราคารวม"];
			dataColumn_4 = base.Columns["รวมจำนวน"];
			dataColumn_5 = base.Columns["รวมราคาท\u0e31\u0e49งหมด"];
			dataColumn_6 = base.Columns["ห\u0e31ว"];
			dataColumn_7 = base.Columns["ช\u0e37\u0e48อล\u0e39กค\u0e49า"];
			dataColumn_8 = base.Columns["ราคาท\u0e38น"];
			dataColumn_9 = base.Columns["ราคาท\u0e38นค\u0e39ณ"];
			dataColumn_10 = base.Columns["กำไร"];
			dataColumn_11 = base.Columns["รวมท\u0e38น"];
			dataColumn_12 = base.Columns["รวมกำไร"];
			dataColumn_13 = base.Columns["เลขท\u0e35\u0e48บ\u0e34ล"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			dataColumn_0 = new DataColumn("ลำด\u0e31บ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("รห\u0e31ส", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			columnName = new DataColumn("Name", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnName);
			columnnum = new DataColumn("num", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnnum);
			dataColumn_2 = new DataColumn("ราคา", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			dataColumn_3 = new DataColumn("ราคารวม", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			dataColumn_4 = new DataColumn("รวมจำนวน", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_4);
			dataColumn_5 = new DataColumn("รวมราคาท\u0e31\u0e49งหมด", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_5);
			dataColumn_6 = new DataColumn("ห\u0e31ว", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_6);
			dataColumn_7 = new DataColumn("ช\u0e37\u0e48อล\u0e39กค\u0e49า", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_7);
			dataColumn_8 = new DataColumn("ราคาท\u0e38น", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_8);
			dataColumn_9 = new DataColumn("ราคาท\u0e38นค\u0e39ณ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_9);
			dataColumn_10 = new DataColumn("กำไร", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_10);
			dataColumn_11 = new DataColumn("รวมท\u0e38น", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_11);
			dataColumn_12 = new DataColumn("รวมกำไร", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_12);
			dataColumn_13 = new DataColumn("เลขท\u0e35\u0e48บ\u0e34ล", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_13);
			columnnum.Caption = "Stock_old";
		}

		[DebuggerNonUserCode]
		public ReportSaleRow NewReportSaleRow()
		{
			return (ReportSaleRow)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new ReportSaleRow(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(ReportSaleRow);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (ReportSaleRowChanged != null)
			{
				ReportSaleRowChanged?.Invoke(this, new ReportSaleRowChangeEvent((ReportSaleRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (ReportSaleRowChanging != null)
			{
				ReportSaleRowChanging?.Invoke(this, new ReportSaleRowChangeEvent((ReportSaleRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (ReportSaleRowDeleted != null)
			{
				ReportSaleRowDeleted?.Invoke(this, new ReportSaleRowChangeEvent((ReportSaleRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (ReportSaleRowDeleting != null)
			{
				ReportSaleRowDeleting?.Invoke(this, new ReportSaleRowChangeEvent((ReportSaleRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReportSaleRow(ReportSaleRow row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "ReportSaleDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[XmlSchemaProvider("GetTypedTableSchema")]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportBookingDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn columnBookingNO;

		private DataColumn columnBooking_NAME;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn columnROOM_TYPE;

		private DataColumn columnROOM_RATE;

		private DataColumn columnROOM_NUM;

		private DataColumn columnROOM_NIGHT;

		private DataColumn columnROOM_TOTAL;

		private DataColumn dataColumn_3;

		private DataColumn columnCONFIRM_BY;

		private DataColumn columnBOOKING_DATE;

		private DataColumn columnDatain;

		private DataColumn columnDataout;

		private DataColumn columntotal;

		[DebuggerNonUserCode]
		public DataColumn BookingNOColumn => columnBookingNO;

		[DebuggerNonUserCode]
		public DataColumn Booking_NAMEColumn => columnBooking_NAME;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn ROOM_TYPEColumn => columnROOM_TYPE;

		[DebuggerNonUserCode]
		public DataColumn ROOM_RATEColumn => columnROOM_RATE;

		[DebuggerNonUserCode]
		public DataColumn ROOM_NUMColumn => columnROOM_NUM;

		[DebuggerNonUserCode]
		public DataColumn ROOM_NIGHTColumn => columnROOM_NIGHT;

		[DebuggerNonUserCode]
		public DataColumn ROOM_TOTALColumn => columnROOM_TOTAL;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn CONFIRM_BYColumn => columnCONFIRM_BY;

		[DebuggerNonUserCode]
		public DataColumn BOOKING_DATEColumn => columnBOOKING_DATE;

		[DebuggerNonUserCode]
		public DataColumn DatainColumn => columnDatain;

		[DebuggerNonUserCode]
		public DataColumn DataoutColumn => columnDataout;

		[DebuggerNonUserCode]
		public DataColumn totalColumn => columntotal;

		[DebuggerNonUserCode]
		[Browsable(false)]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public ReportBookingRow this[int index] => (ReportBookingRow)Rows[index];

		[method: DebuggerNonUserCode]
		public event ReportBookingRowChangeEventHandler ReportBookingRowChanging;

		[method: DebuggerNonUserCode]
		public event ReportBookingRowChangeEventHandler ReportBookingRowChanged;

		[method: DebuggerNonUserCode]
		public event ReportBookingRowChangeEventHandler ReportBookingRowDeleting;

		[method: DebuggerNonUserCode]
		public event ReportBookingRowChangeEventHandler ReportBookingRowDeleted;

		[DebuggerNonUserCode]
		static ReportBookingDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public ReportBookingDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			base.ColumnChanging += ReportBookingDataTable_ColumnChanging;
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "ReportBooking";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal ReportBookingDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			base.ColumnChanging += ReportBookingDataTable_ColumnChanging;
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected ReportBookingDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			base.ColumnChanging += ReportBookingDataTable_ColumnChanging;
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReportBookingRow(ReportBookingRow row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public ReportBookingRow AddReportBookingRow(string BookingNO, string Booking_NAME, string CIN, string COUT, string NIGHT, string ROOM_TYPE, string ROOM_RATE, string ROOM_NUM, string ROOM_NIGHT, string ROOM_TOTAL, string NOTE, string CONFIRM_BY, string BOOKING_DATE, string Datain, string Dataout, string total)
		{
			ReportBookingRow reportBookingRow = (ReportBookingRow)NewRow();
			object[] itemArray = new object[16]
			{
				BookingNO, Booking_NAME, CIN, COUT, NIGHT, ROOM_TYPE, ROOM_RATE, ROOM_NUM, ROOM_NIGHT, ROOM_TOTAL,
				NOTE, CONFIRM_BY, BOOKING_DATE, Datain, Dataout, total
			};
			reportBookingRow.ItemArray = itemArray;
			Rows.Add(reportBookingRow);
			return reportBookingRow;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			ReportBookingDataTable reportBookingDataTable = (ReportBookingDataTable)base.Clone();
			reportBookingDataTable.InitVars();
			return reportBookingDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new ReportBookingDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			columnBookingNO = base.Columns["BookingNO"];
			columnBooking_NAME = base.Columns["Booking_NAME"];
			dataColumn_0 = base.Columns["CIN"];
			dataColumn_1 = base.Columns["COUT"];
			dataColumn_2 = base.Columns["NIGHT"];
			columnROOM_TYPE = base.Columns["ROOM_TYPE"];
			columnROOM_RATE = base.Columns["ROOM_RATE"];
			columnROOM_NUM = base.Columns["ROOM_NUM"];
			columnROOM_NIGHT = base.Columns["ROOM_NIGHT"];
			columnROOM_TOTAL = base.Columns["ROOM_TOTAL"];
			dataColumn_3 = base.Columns["NOTE"];
			columnCONFIRM_BY = base.Columns["CONFIRM_BY"];
			columnBOOKING_DATE = base.Columns["BOOKING_DATE"];
			columnDatain = base.Columns["Datain"];
			columnDataout = base.Columns["Dataout"];
			columntotal = base.Columns["total"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			columnBookingNO = new DataColumn("BookingNO", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnBookingNO);
			columnBooking_NAME = new DataColumn("Booking_NAME", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnBooking_NAME);
			dataColumn_0 = new DataColumn("CIN", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("COUT", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("NIGHT", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			columnROOM_TYPE = new DataColumn("ROOM_TYPE", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnROOM_TYPE);
			columnROOM_RATE = new DataColumn("ROOM_RATE", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnROOM_RATE);
			columnROOM_NUM = new DataColumn("ROOM_NUM", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnROOM_NUM);
			columnROOM_NIGHT = new DataColumn("ROOM_NIGHT", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnROOM_NIGHT);
			columnROOM_TOTAL = new DataColumn("ROOM_TOTAL", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnROOM_TOTAL);
			dataColumn_3 = new DataColumn("NOTE", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			columnCONFIRM_BY = new DataColumn("CONFIRM_BY", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnCONFIRM_BY);
			columnBOOKING_DATE = new DataColumn("BOOKING_DATE", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnBOOKING_DATE);
			columnDatain = new DataColumn("Datain", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnDatain);
			columnDataout = new DataColumn("Dataout", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnDataout);
			columntotal = new DataColumn("total", typeof(string), null, MappingType.Element);
			base.Columns.Add(columntotal);
			dataColumn_2.Caption = "DataColumn3";
			columnROOM_TYPE.Caption = "DataColumn3";
			columnROOM_RATE.Caption = "DataColumn3";
			columnROOM_NUM.Caption = "DataColumn3";
			columnROOM_NIGHT.Caption = "DataColumn3";
			columnROOM_TOTAL.Caption = "DataColumn3";
			dataColumn_3.Caption = "DataColumn3";
			columnCONFIRM_BY.Caption = "DataColumn3";
		}

		[DebuggerNonUserCode]
		public ReportBookingRow NewReportBookingRow()
		{
			return (ReportBookingRow)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new ReportBookingRow(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(ReportBookingRow);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (ReportBookingRowChanged != null)
			{
				ReportBookingRowChanged?.Invoke(this, new ReportBookingRowChangeEvent((ReportBookingRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (ReportBookingRowChanging != null)
			{
				ReportBookingRowChanging?.Invoke(this, new ReportBookingRowChangeEvent((ReportBookingRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (ReportBookingRowDeleted != null)
			{
				ReportBookingRowDeleted?.Invoke(this, new ReportBookingRowChangeEvent((ReportBookingRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (ReportBookingRowDeleting != null)
			{
				ReportBookingRowDeleting?.Invoke(this, new ReportBookingRowChangeEvent((ReportBookingRow)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReportBookingRow(ReportBookingRow row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "ReportBookingDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}

		private void ReportBookingDataTable_ColumnChanging(object sender, DataColumnChangeEventArgs e)
		{
			if (Operators.CompareString(e.Column.ColumnName, ROOM_NUMColumn.ColumnName, TextCompare: false) == 0)
			{
			}
		}
	}

	[Serializable]
	[XmlSchemaProvider("GetTypedTableSchema")]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportBookingINVDataTable : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn columnBookingNO;

		private DataColumn columnBooking_NAME;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn columnROOM_TYPE;

		private DataColumn columnROOM_RATE;

		private DataColumn columnROOM_NUM;

		private DataColumn columnROOM_NIGHT;

		private DataColumn columnROOM_TOTAL;

		private DataColumn dataColumn_3;

		private DataColumn columnCONFIRM_BY;

		private DataColumn columnBOOKING_DATE;

		private DataColumn columnDatain;

		private DataColumn columnDataout;

		private DataColumn columntotal;

		private DataColumn columnINV_DATE;

		private DataColumn columnINV_BY;

		private DataColumn columnINV_TITLE;

		private DataColumn columnINV_NAME;

		private DataColumn columnINV_COMPANY;

		private DataColumn columnINV_ADDRESS;

		private DataColumn columnINV_TEL;

		private DataColumn columnINV_NIGHT;

		private DataColumn columnINV_PAX;

		private DataColumn columnINV_PAX_CHILD;

		private DataColumn columnINV_PAYMENT;

		private DataColumn columnINV_DUEDATE;

		private DataColumn columnINV_NOTE;

		private DataColumn columnINV_NO;

		private DataColumn columnpay;

		private DataColumn columnbalance;

		private DataColumn columninv_stay;

		[DebuggerNonUserCode]
		public DataColumn BookingNOColumn => columnBookingNO;

		[DebuggerNonUserCode]
		public DataColumn Booking_NAMEColumn => columnBooking_NAME;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn ROOM_TYPEColumn => columnROOM_TYPE;

		[DebuggerNonUserCode]
		public DataColumn ROOM_RATEColumn => columnROOM_RATE;

		[DebuggerNonUserCode]
		public DataColumn ROOM_NUMColumn => columnROOM_NUM;

		[DebuggerNonUserCode]
		public DataColumn ROOM_NIGHTColumn => columnROOM_NIGHT;

		[DebuggerNonUserCode]
		public DataColumn ROOM_TOTALColumn => columnROOM_TOTAL;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn CONFIRM_BYColumn => columnCONFIRM_BY;

		[DebuggerNonUserCode]
		public DataColumn BOOKING_DATEColumn => columnBOOKING_DATE;

		[DebuggerNonUserCode]
		public DataColumn DatainColumn => columnDatain;

		[DebuggerNonUserCode]
		public DataColumn DataoutColumn => columnDataout;

		[DebuggerNonUserCode]
		public DataColumn totalColumn => columntotal;

		[DebuggerNonUserCode]
		public DataColumn INV_DATEColumn => columnINV_DATE;

		[DebuggerNonUserCode]
		public DataColumn INV_BYColumn => columnINV_BY;

		[DebuggerNonUserCode]
		public DataColumn INV_TITLEColumn => columnINV_TITLE;

		[DebuggerNonUserCode]
		public DataColumn INV_NAMEColumn => columnINV_NAME;

		[DebuggerNonUserCode]
		public DataColumn INV_COMPANYColumn => columnINV_COMPANY;

		[DebuggerNonUserCode]
		public DataColumn INV_ADDRESSColumn => columnINV_ADDRESS;

		[DebuggerNonUserCode]
		public DataColumn INV_TELColumn => columnINV_TEL;

		[DebuggerNonUserCode]
		public DataColumn INV_NIGHTColumn => columnINV_NIGHT;

		[DebuggerNonUserCode]
		public DataColumn INV_PAXColumn => columnINV_PAX;

		[DebuggerNonUserCode]
		public DataColumn INV_PAX_CHILDColumn => columnINV_PAX_CHILD;

		[DebuggerNonUserCode]
		public DataColumn INV_PAYMENTColumn => columnINV_PAYMENT;

		[DebuggerNonUserCode]
		public DataColumn INV_DUEDATEColumn => columnINV_DUEDATE;

		[DebuggerNonUserCode]
		public DataColumn INV_NOTEColumn => columnINV_NOTE;

		[DebuggerNonUserCode]
		public DataColumn INV_NOColumn => columnINV_NO;

		[DebuggerNonUserCode]
		public DataColumn payColumn => columnpay;

		[DebuggerNonUserCode]
		public DataColumn balanceColumn => columnbalance;

		[DebuggerNonUserCode]
		public DataColumn inv_stayColumn => columninv_stay;

		[DebuggerNonUserCode]
		[Browsable(false)]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public GClass1 this[int index] => (GClass1)Rows[index];

		[method: DebuggerNonUserCode]
		public event ReportBookingINVRowChangeEventHandler ReportBookingINVRowChanging;

		[method: DebuggerNonUserCode]
		public event ReportBookingINVRowChangeEventHandler ReportBookingINVRowChanged;

		[method: DebuggerNonUserCode]
		public event ReportBookingINVRowChangeEventHandler ReportBookingINVRowDeleting;

		[method: DebuggerNonUserCode]
		public event ReportBookingINVRowChangeEventHandler ReportBookingINVRowDeleted;

		[DebuggerNonUserCode]
		static ReportBookingINVDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public ReportBookingINVDataTable()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "ReportBookingINV";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal ReportBookingINVDataTable(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected ReportBookingINVDataTable(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void AddReportBookingINVRow(GClass1 row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public GClass1 AddReportBookingINVRow(string BookingNO, string Booking_NAME, string CIN, string COUT, string NIGHT, string ROOM_TYPE, string ROOM_RATE, string ROOM_NUM, string ROOM_NIGHT, string ROOM_TOTAL, string NOTE, string CONFIRM_BY, string BOOKING_DATE, string Datain, string Dataout, string total, string INV_DATE, string INV_BY, string INV_TITLE, string INV_NAME, string INV_COMPANY, string INV_ADDRESS, string INV_TEL, string INV_NIGHT, string INV_PAX, string INV_PAX_CHILD, string INV_PAYMENT, string INV_DUEDATE, string INV_NOTE, string INV_NO, string pay, string balance, string inv_stay)
		{
			GClass1 gClass = (GClass1)NewRow();
			object[] itemArray = new object[33]
			{
				BookingNO, Booking_NAME, CIN, COUT, NIGHT, ROOM_TYPE, ROOM_RATE, ROOM_NUM, ROOM_NIGHT, ROOM_TOTAL,
				NOTE, CONFIRM_BY, BOOKING_DATE, Datain, Dataout, total, INV_DATE, INV_BY, INV_TITLE, INV_NAME,
				INV_COMPANY, INV_ADDRESS, INV_TEL, INV_NIGHT, INV_PAX, INV_PAX_CHILD, INV_PAYMENT, INV_DUEDATE, INV_NOTE, INV_NO,
				pay, balance, inv_stay
			};
			gClass.ItemArray = itemArray;
			Rows.Add(gClass);
			return gClass;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			ReportBookingINVDataTable reportBookingINVDataTable = (ReportBookingINVDataTable)base.Clone();
			reportBookingINVDataTable.InitVars();
			return reportBookingINVDataTable;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new ReportBookingINVDataTable();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			columnBookingNO = base.Columns["BookingNO"];
			columnBooking_NAME = base.Columns["Booking_NAME"];
			dataColumn_0 = base.Columns["CIN"];
			dataColumn_1 = base.Columns["COUT"];
			dataColumn_2 = base.Columns["NIGHT"];
			columnROOM_TYPE = base.Columns["ROOM_TYPE"];
			columnROOM_RATE = base.Columns["ROOM_RATE"];
			columnROOM_NUM = base.Columns["ROOM_NUM"];
			columnROOM_NIGHT = base.Columns["ROOM_NIGHT"];
			columnROOM_TOTAL = base.Columns["ROOM_TOTAL"];
			dataColumn_3 = base.Columns["NOTE"];
			columnCONFIRM_BY = base.Columns["CONFIRM_BY"];
			columnBOOKING_DATE = base.Columns["BOOKING_DATE"];
			columnDatain = base.Columns["Datain"];
			columnDataout = base.Columns["Dataout"];
			columntotal = base.Columns["total"];
			columnINV_DATE = base.Columns["INV_DATE"];
			columnINV_BY = base.Columns["INV_BY"];
			columnINV_TITLE = base.Columns["INV_TITLE"];
			columnINV_NAME = base.Columns["INV_NAME"];
			columnINV_COMPANY = base.Columns["INV_COMPANY"];
			columnINV_ADDRESS = base.Columns["INV_ADDRESS"];
			columnINV_TEL = base.Columns["INV_TEL"];
			columnINV_NIGHT = base.Columns["INV_NIGHT"];
			columnINV_PAX = base.Columns["INV_PAX"];
			columnINV_PAX_CHILD = base.Columns["INV_PAX_CHILD"];
			columnINV_PAYMENT = base.Columns["INV_PAYMENT"];
			columnINV_DUEDATE = base.Columns["INV_DUEDATE"];
			columnINV_NOTE = base.Columns["INV_NOTE"];
			columnINV_NO = base.Columns["INV_NO"];
			columnpay = base.Columns["pay"];
			columnbalance = base.Columns["balance"];
			columninv_stay = base.Columns["inv_stay"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			columnBookingNO = new DataColumn("BookingNO", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnBookingNO);
			columnBooking_NAME = new DataColumn("Booking_NAME", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnBooking_NAME);
			dataColumn_0 = new DataColumn("CIN", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("COUT", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("NIGHT", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			columnROOM_TYPE = new DataColumn("ROOM_TYPE", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnROOM_TYPE);
			columnROOM_RATE = new DataColumn("ROOM_RATE", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnROOM_RATE);
			columnROOM_NUM = new DataColumn("ROOM_NUM", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnROOM_NUM);
			columnROOM_NIGHT = new DataColumn("ROOM_NIGHT", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnROOM_NIGHT);
			columnROOM_TOTAL = new DataColumn("ROOM_TOTAL", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnROOM_TOTAL);
			dataColumn_3 = new DataColumn("NOTE", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			columnCONFIRM_BY = new DataColumn("CONFIRM_BY", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnCONFIRM_BY);
			columnBOOKING_DATE = new DataColumn("BOOKING_DATE", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnBOOKING_DATE);
			columnDatain = new DataColumn("Datain", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnDatain);
			columnDataout = new DataColumn("Dataout", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnDataout);
			columntotal = new DataColumn("total", typeof(string), null, MappingType.Element);
			base.Columns.Add(columntotal);
			columnINV_DATE = new DataColumn("INV_DATE", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnINV_DATE);
			columnINV_BY = new DataColumn("INV_BY", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnINV_BY);
			columnINV_TITLE = new DataColumn("INV_TITLE", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnINV_TITLE);
			columnINV_NAME = new DataColumn("INV_NAME", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnINV_NAME);
			columnINV_COMPANY = new DataColumn("INV_COMPANY", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnINV_COMPANY);
			columnINV_ADDRESS = new DataColumn("INV_ADDRESS", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnINV_ADDRESS);
			columnINV_TEL = new DataColumn("INV_TEL", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnINV_TEL);
			columnINV_NIGHT = new DataColumn("INV_NIGHT", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnINV_NIGHT);
			columnINV_PAX = new DataColumn("INV_PAX", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnINV_PAX);
			columnINV_PAX_CHILD = new DataColumn("INV_PAX_CHILD", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnINV_PAX_CHILD);
			columnINV_PAYMENT = new DataColumn("INV_PAYMENT", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnINV_PAYMENT);
			columnINV_DUEDATE = new DataColumn("INV_DUEDATE", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnINV_DUEDATE);
			columnINV_NOTE = new DataColumn("INV_NOTE", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnINV_NOTE);
			columnINV_NO = new DataColumn("INV_NO", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnINV_NO);
			columnpay = new DataColumn("pay", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnpay);
			columnbalance = new DataColumn("balance", typeof(string), null, MappingType.Element);
			base.Columns.Add(columnbalance);
			columninv_stay = new DataColumn("inv_stay", typeof(string), null, MappingType.Element);
			base.Columns.Add(columninv_stay);
			dataColumn_2.Caption = "DataColumn3";
			columnROOM_TYPE.Caption = "DataColumn3";
			columnROOM_RATE.Caption = "DataColumn3";
			columnROOM_NUM.Caption = "DataColumn3";
			columnROOM_NIGHT.Caption = "DataColumn3";
			columnROOM_TOTAL.Caption = "DataColumn3";
			dataColumn_3.Caption = "DataColumn3";
			columnCONFIRM_BY.Caption = "DataColumn3";
			columnINV_DATE.Caption = "DataColumn1";
			columnINV_BY.Caption = "DataColumn1";
			columnINV_TITLE.Caption = "DataColumn1";
			columnINV_NAME.Caption = "DataColumn1";
			columnINV_COMPANY.Caption = "DataColumn1";
			columnINV_ADDRESS.Caption = "DataColumn1";
			columnINV_TEL.Caption = "DataColumn1";
			columnINV_NIGHT.Caption = "DataColumn1";
			columnINV_PAX.Caption = "DataColumn1";
			columnINV_PAX_CHILD.Caption = "DataColumn1";
			columnINV_PAYMENT.Caption = "DataColumn1";
			columnINV_DUEDATE.Caption = "DataColumn1";
			columnINV_NO.Caption = "DataColumn1";
			columnpay.Caption = "DataColumn1";
		}

		[DebuggerNonUserCode]
		public GClass1 NewReportBookingINVRow()
		{
			return (GClass1)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new GClass1(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(GClass1);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (ReportBookingINVRowChanged != null)
			{
				ReportBookingINVRowChanged?.Invoke(this, new ReportBookingINVRowChangeEvent((GClass1)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (ReportBookingINVRowChanging != null)
			{
				ReportBookingINVRowChanging?.Invoke(this, new ReportBookingINVRowChangeEvent((GClass1)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (ReportBookingINVRowDeleted != null)
			{
				ReportBookingINVRowDeleted?.Invoke(this, new ReportBookingINVRowChangeEvent((GClass1)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (ReportBookingINVRowDeleting != null)
			{
				ReportBookingINVRowDeleting?.Invoke(this, new ReportBookingINVRowChangeEvent((GClass1)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void RemoveReportBookingINVRow(GClass1 row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "ReportBookingINVDataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[Serializable]
	[XmlSchemaProvider("GetTypedTableSchema")]
	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class GClass0 : DataTable, IEnumerable
	{
		private static List<WeakReference> __ENCList;

		private DataColumn dataColumn_0;

		private DataColumn dataColumn_1;

		private DataColumn dataColumn_2;

		private DataColumn dataColumn_3;

		private DataColumn dataColumn_4;

		private DataColumn dataColumn_5;

		private DataColumn dataColumn_6;

		private DataColumn dataColumn_7;

		private DataColumn dataColumn_8;

		private DataColumn dataColumn_9;

		private DataColumn dataColumn_10;

		private DataColumn dataColumn_11;

		private DataColumn dataColumn_12;

		private DataColumn dataColumn_13;

		private GDelegate0 gdelegate0_0;

		private GDelegate0 gdelegate0_1;

		private GDelegate0 gdelegate0_2;

		private GDelegate0 gdelegate0_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_0 => dataColumn_0;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_1 => dataColumn_1;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_2 => dataColumn_2;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_3 => dataColumn_3;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_4 => dataColumn_4;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_5 => dataColumn_5;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_6 => dataColumn_6;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_7 => dataColumn_7;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_8 => dataColumn_8;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_9 => dataColumn_9;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_10 => dataColumn_10;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_11 => dataColumn_11;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_12 => dataColumn_12;

		[DebuggerNonUserCode]
		public DataColumn DataColumn_13 => dataColumn_13;

		[Browsable(false)]
		[DebuggerNonUserCode]
		public int Count => Rows.Count;

		[DebuggerNonUserCode]
		public GClass2 this[int index] => (GClass2)Rows[index];

		public event GDelegate0 Event_0
		{
			[MethodImpl(MethodImplOptions.Synchronized)]
			[DebuggerNonUserCode]
			add
			{
				gdelegate0_0 = (GDelegate0)Delegate.Combine(gdelegate0_0, value);
			}
			[MethodImpl(MethodImplOptions.Synchronized)]
			[DebuggerNonUserCode]
			remove
			{
				gdelegate0_0 = (GDelegate0)Delegate.Remove(gdelegate0_0, value);
			}
		}

		public event GDelegate0 Event_1
		{
			[MethodImpl(MethodImplOptions.Synchronized)]
			[DebuggerNonUserCode]
			add
			{
				gdelegate0_1 = (GDelegate0)Delegate.Combine(gdelegate0_1, value);
			}
			[MethodImpl(MethodImplOptions.Synchronized)]
			[DebuggerNonUserCode]
			remove
			{
				gdelegate0_1 = (GDelegate0)Delegate.Remove(gdelegate0_1, value);
			}
		}

		public event GDelegate0 Event_2
		{
			[MethodImpl(MethodImplOptions.Synchronized)]
			[DebuggerNonUserCode]
			add
			{
				gdelegate0_2 = (GDelegate0)Delegate.Combine(gdelegate0_2, value);
			}
			[MethodImpl(MethodImplOptions.Synchronized)]
			[DebuggerNonUserCode]
			remove
			{
				gdelegate0_2 = (GDelegate0)Delegate.Remove(gdelegate0_2, value);
			}
		}

		public event GDelegate0 Event_3
		{
			[MethodImpl(MethodImplOptions.Synchronized)]
			[DebuggerNonUserCode]
			add
			{
				gdelegate0_3 = (GDelegate0)Delegate.Combine(gdelegate0_3, value);
			}
			[MethodImpl(MethodImplOptions.Synchronized)]
			[DebuggerNonUserCode]
			remove
			{
				gdelegate0_3 = (GDelegate0)Delegate.Remove(gdelegate0_3, value);
			}
		}

		[DebuggerNonUserCode]
		static GClass0()
		{
			Class2.LH6iGfYz9j3MJ();
			__ENCList = new List<WeakReference>();
		}

		[DebuggerNonUserCode]
		public GClass0()
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = "Report_รร_4";
			BeginInit();
			InitClass();
			EndInit();
		}

		[DebuggerNonUserCode]
		internal GClass0(DataTable table)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			TableName = table.TableName;
			if (table.CaseSensitive != table.DataSet.CaseSensitive)
			{
				CaseSensitive = table.CaseSensitive;
			}
			if (Operators.CompareString(table.Locale.ToString(), table.DataSet.Locale.ToString(), TextCompare: false) != 0)
			{
				Locale = table.Locale;
			}
			if (Operators.CompareString(table.Namespace, table.DataSet.Namespace, TextCompare: false) != 0)
			{
				Namespace = table.Namespace;
			}
			Prefix = table.Prefix;
			MinimumCapacity = table.MinimumCapacity;
		}

		[DebuggerNonUserCode]
		protected GClass0(SerializationInfo info, StreamingContext context)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(info, context);
			lock (__ENCList)
			{
				__ENCList.Add(new WeakReference(this));
			}
			InitVars();
		}

		[DebuggerNonUserCode]
		public void method_0(GClass2 row)
		{
			Rows.Add(row);
		}

		[DebuggerNonUserCode]
		public GClass2 method_1(string string_0, string string_1, string string_2, string string_3, string string_4, string string_5, string string_6, string string_7, string string_8, string string_9, string string_10, string string_11, string string_12, string string_13)
		{
			GClass2 gClass = (GClass2)NewRow();
			object[] itemArray = new object[14]
			{
				string_0, string_1, string_2, string_3, string_4, string_5, string_6, string_7, string_8, string_9,
				string_10, string_11, string_12, string_13
			};
			gClass.ItemArray = itemArray;
			Rows.Add(gClass);
			return gClass;
		}

		[DebuggerNonUserCode]
		public virtual IEnumerator GetEnumerator()
		{
			return Rows.GetEnumerator();
		}

		[DebuggerNonUserCode]
		public override DataTable Clone()
		{
			GClass0 gClass = (GClass0)base.Clone();
			gClass.InitVars();
			return gClass;
		}

		[DebuggerNonUserCode]
		protected override DataTable CreateInstance()
		{
			return new GClass0();
		}

		[DebuggerNonUserCode]
		internal void InitVars()
		{
			dataColumn_0 = base.Columns["ช\u0e37\u0e48อรร"];
			dataColumn_1 = base.Columns["ว\u0e31นท\u0e35\u0e48"];
			dataColumn_2 = base.Columns["ลำด\u0e31บ"];
			dataColumn_3 = base.Columns["ว\u0e31นท\u0e35\u0e48เข\u0e49าพ\u0e31ก"];
			dataColumn_4 = base.Columns["ห\u0e49องพ\u0e31กเลขท\u0e35\u0e48"];
			dataColumn_5 = base.Columns["ช\u0e37\u0e48อสก\u0e38ล"];
			dataColumn_6 = base.Columns["ส\u0e31ญชาต\u0e34"];
			dataColumn_7 = base.Columns["เลขประจำต\u0e31ว"];
			dataColumn_8 = base.Columns["ท\u0e35\u0e48อย\u0e39\u0e48"];
			dataColumn_9 = base.Columns["อาช\u0e35พ"];
			dataColumn_10 = base.Columns["มาจาก"];
			dataColumn_11 = base.Columns["จะไปท\u0e35\u0e48"];
			dataColumn_12 = base.Columns["ว\u0e31นท\u0e35\u0e48ออก"];
			dataColumn_13 = base.Columns["หมายเหต\u0e38"];
		}

		[DebuggerNonUserCode]
		private void InitClass()
		{
			dataColumn_0 = new DataColumn("ช\u0e37\u0e48อรร", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_0);
			dataColumn_1 = new DataColumn("ว\u0e31นท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_1);
			dataColumn_2 = new DataColumn("ลำด\u0e31บ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_2);
			dataColumn_3 = new DataColumn("ว\u0e31นท\u0e35\u0e48เข\u0e49าพ\u0e31ก", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_3);
			dataColumn_4 = new DataColumn("ห\u0e49องพ\u0e31กเลขท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_4);
			dataColumn_5 = new DataColumn("ช\u0e37\u0e48อสก\u0e38ล", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_5);
			dataColumn_6 = new DataColumn("ส\u0e31ญชาต\u0e34", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_6);
			dataColumn_7 = new DataColumn("เลขประจำต\u0e31ว", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_7);
			dataColumn_8 = new DataColumn("ท\u0e35\u0e48อย\u0e39\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_8);
			dataColumn_9 = new DataColumn("อาช\u0e35พ", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_9);
			dataColumn_10 = new DataColumn("มาจาก", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_10);
			dataColumn_11 = new DataColumn("จะไปท\u0e35\u0e48", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_11);
			dataColumn_12 = new DataColumn("ว\u0e31นท\u0e35\u0e48ออก", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_12);
			dataColumn_13 = new DataColumn("หมายเหต\u0e38", typeof(string), null, MappingType.Element);
			base.Columns.Add(dataColumn_13);
			dataColumn_0.Caption = "DataColumn1";
			dataColumn_1.Caption = "DataColumn1";
			dataColumn_2.Caption = "DataColumn1";
			dataColumn_3.Caption = "DataColumn1";
			dataColumn_4.Caption = "DataColumn1";
			dataColumn_5.Caption = "DataColumn1";
			dataColumn_6.Caption = "DataColumn1";
			dataColumn_7.Caption = "DataColumn1";
			dataColumn_8.Caption = "DataColumn1";
			dataColumn_9.Caption = "DataColumn1";
			dataColumn_10.Caption = "DataColumn1";
			dataColumn_11.Caption = "DataColumn1";
			dataColumn_12.Caption = "DataColumn1";
			dataColumn_13.Caption = "DataColumn1";
		}

		[DebuggerNonUserCode]
		public GClass2 method_2()
		{
			return (GClass2)NewRow();
		}

		[DebuggerNonUserCode]
		protected override DataRow NewRowFromBuilder(DataRowBuilder builder)
		{
			return new GClass2(builder);
		}

		[DebuggerNonUserCode]
		protected override Type GetRowType()
		{
			return typeof(GClass2);
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanged(DataRowChangeEventArgs e)
		{
			base.OnRowChanged(e);
			if (gdelegate0_1 != null)
			{
				gdelegate0_1?.Invoke(this, new GEventArgs0((GClass2)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowChanging(DataRowChangeEventArgs e)
		{
			base.OnRowChanging(e);
			if (gdelegate0_0 != null)
			{
				gdelegate0_0?.Invoke(this, new GEventArgs0((GClass2)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleted(DataRowChangeEventArgs e)
		{
			base.OnRowDeleted(e);
			if (gdelegate0_3 != null)
			{
				gdelegate0_3?.Invoke(this, new GEventArgs0((GClass2)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		protected override void OnRowDeleting(DataRowChangeEventArgs e)
		{
			base.OnRowDeleting(e);
			if (gdelegate0_2 != null)
			{
				gdelegate0_2?.Invoke(this, new GEventArgs0((GClass2)e.Row, e.Action));
			}
		}

		[DebuggerNonUserCode]
		public void method_3(GClass2 row)
		{
			Rows.Remove(row);
		}

		[DebuggerNonUserCode]
		public static XmlSchemaComplexType GetTypedTableSchema(XmlSchemaSet xs)
		{
			XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
			XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
			Datalocal datalocal = new Datalocal();
			XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
			xmlSchemaAny.Namespace = "http://www.w3.org/2001/XMLSchema";
			decimal num = 0m;
			xmlSchemaAny.MinOccurs = num;
			xmlSchemaAny.MaxOccurs = decimal.MaxValue;
			xmlSchemaAny.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny);
			XmlSchemaAny xmlSchemaAny2 = new XmlSchemaAny();
			xmlSchemaAny2.Namespace = "urn:schemas-microsoft-com:xml-diffgram-v1";
			num = 1m;
			xmlSchemaAny2.MinOccurs = num;
			xmlSchemaAny2.ProcessContents = XmlSchemaContentProcessing.Lax;
			xmlSchemaSequence.Items.Add(xmlSchemaAny2);
			XmlSchemaAttribute xmlSchemaAttribute = new XmlSchemaAttribute();
			xmlSchemaAttribute.Name = "namespace";
			xmlSchemaAttribute.FixedValue = datalocal.Namespace;
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute);
			XmlSchemaAttribute xmlSchemaAttribute2 = new XmlSchemaAttribute();
			xmlSchemaAttribute2.Name = "tableTypeName";
			xmlSchemaAttribute2.FixedValue = "Report_รร_4DataTable";
			xmlSchemaComplexType.Attributes.Add(xmlSchemaAttribute2);
			xmlSchemaComplexType.Particle = xmlSchemaSequence;
			XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
			if (xs.Contains(schemaSerializable.TargetNamespace))
			{
				MemoryStream memoryStream = new MemoryStream();
				MemoryStream memoryStream2 = new MemoryStream();
				try
				{
					XmlSchema xmlSchema = null;
					schemaSerializable.Write(memoryStream);
					IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
					while (enumerator.MoveNext())
					{
						xmlSchema = (XmlSchema)enumerator.Current;
						memoryStream2.SetLength(0L);
						xmlSchema.Write(memoryStream2);
						if (memoryStream.Length == memoryStream2.Length)
						{
							memoryStream.Position = 0L;
							memoryStream2.Position = 0L;
							while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
							{
							}
							if (memoryStream.Position == memoryStream.Length)
							{
								return xmlSchemaComplexType;
							}
						}
					}
				}
				finally
				{
					memoryStream?.Close();
					memoryStream2?.Close();
				}
			}
			xs.Add(schemaSerializable);
			return xmlSchemaComplexType;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportBillCashRow : DataRow
	{
		private ReportBillCashDataTable tableReportBillCash;

		[DebuggerNonUserCode]
		public string String_0
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'กร\u0e38\u0e4aป' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_1
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เลขท\u0e35\u0e48' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_3
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ส\u0e48งของท\u0e35\u0e48' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_4
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_4]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ช\u0e37\u0e48อ' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_4] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_5
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_5]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ท\u0e35\u0e48อย\u0e39\u0e48' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_5] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_6
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_6]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ลำด\u0e31บ' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_6] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_7
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_7]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รายการ' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_7] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_8
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_8]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'จำนวน' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_8] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_9
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_9]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ช\u0e37\u0e48อหน\u0e48วย' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_9] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_10
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_10]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'หน\u0e48วยละ' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_10] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_11
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_11]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'จำนวนเง\u0e34น' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_11] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_12
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_12]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคารวม' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_12] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_13
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_13]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ภาษ\u0e35เปอร\u0e4cเซ\u0e49น' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_13] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_14
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_14]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ภาษ\u0e35ราคา' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_14] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_15
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_15]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคารวมภาษ\u0e35' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_15] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_16
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_16]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ผ\u0e39\u0e49ออกบ\u0e34ล' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_16] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_17
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_17]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ผ\u0e39\u0e49ส\u0e48ง' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_17] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_18
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_18]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคาtext' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_18] = value;
			}
		}

		[DebuggerNonUserCode]
		public byte[] barcode
		{
			get
			{
				try
				{
					return (byte[])this[tableReportBillCash.barcodeColumn];
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'barcode' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.barcodeColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_19
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_19]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ส\u0e48วนลดรายการ' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_19] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_20
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_20]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ส\u0e48วนลดรวม' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_20] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_21
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_21]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ห\u0e31วบ\u0e34ล' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_21] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_22
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCash.DataColumn_22]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'หมายเหต\u0e38' in table 'ReportBillCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCash.DataColumn_22] = value;
			}
		}

		[DebuggerNonUserCode]
		internal ReportBillCashRow(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReportBillCash = (ReportBillCashDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableReportBillCash.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableReportBillCash.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableReportBillCash.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableReportBillCash.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableReportBillCash.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void method_5()
		{
			this[tableReportBillCash.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_6()
		{
			return IsNull(tableReportBillCash.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_7()
		{
			this[tableReportBillCash.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_8()
		{
			return IsNull(tableReportBillCash.DataColumn_4);
		}

		[DebuggerNonUserCode]
		public void method_9()
		{
			this[tableReportBillCash.DataColumn_4] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_10()
		{
			return IsNull(tableReportBillCash.DataColumn_5);
		}

		[DebuggerNonUserCode]
		public void method_11()
		{
			this[tableReportBillCash.DataColumn_5] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_12()
		{
			return IsNull(tableReportBillCash.DataColumn_6);
		}

		[DebuggerNonUserCode]
		public void method_13()
		{
			this[tableReportBillCash.DataColumn_6] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_14()
		{
			return IsNull(tableReportBillCash.DataColumn_7);
		}

		[DebuggerNonUserCode]
		public void method_15()
		{
			this[tableReportBillCash.DataColumn_7] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_16()
		{
			return IsNull(tableReportBillCash.DataColumn_8);
		}

		[DebuggerNonUserCode]
		public void method_17()
		{
			this[tableReportBillCash.DataColumn_8] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_18()
		{
			return IsNull(tableReportBillCash.DataColumn_9);
		}

		[DebuggerNonUserCode]
		public void method_19()
		{
			this[tableReportBillCash.DataColumn_9] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_20()
		{
			return IsNull(tableReportBillCash.DataColumn_10);
		}

		[DebuggerNonUserCode]
		public void method_21()
		{
			this[tableReportBillCash.DataColumn_10] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_22()
		{
			return IsNull(tableReportBillCash.DataColumn_11);
		}

		[DebuggerNonUserCode]
		public void method_23()
		{
			this[tableReportBillCash.DataColumn_11] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_24()
		{
			return IsNull(tableReportBillCash.DataColumn_12);
		}

		[DebuggerNonUserCode]
		public void method_25()
		{
			this[tableReportBillCash.DataColumn_12] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_26()
		{
			return IsNull(tableReportBillCash.DataColumn_13);
		}

		[DebuggerNonUserCode]
		public void method_27()
		{
			this[tableReportBillCash.DataColumn_13] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_28()
		{
			return IsNull(tableReportBillCash.DataColumn_14);
		}

		[DebuggerNonUserCode]
		public void method_29()
		{
			this[tableReportBillCash.DataColumn_14] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_30()
		{
			return IsNull(tableReportBillCash.DataColumn_15);
		}

		[DebuggerNonUserCode]
		public void method_31()
		{
			this[tableReportBillCash.DataColumn_15] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_32()
		{
			return IsNull(tableReportBillCash.DataColumn_16);
		}

		[DebuggerNonUserCode]
		public void method_33()
		{
			this[tableReportBillCash.DataColumn_16] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_34()
		{
			return IsNull(tableReportBillCash.DataColumn_17);
		}

		[DebuggerNonUserCode]
		public void method_35()
		{
			this[tableReportBillCash.DataColumn_17] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_36()
		{
			return IsNull(tableReportBillCash.DataColumn_18);
		}

		[DebuggerNonUserCode]
		public void method_37()
		{
			this[tableReportBillCash.DataColumn_18] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsbarcodeNull()
		{
			return IsNull(tableReportBillCash.barcodeColumn);
		}

		[DebuggerNonUserCode]
		public void SetbarcodeNull()
		{
			this[tableReportBillCash.barcodeColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_38()
		{
			return IsNull(tableReportBillCash.DataColumn_19);
		}

		[DebuggerNonUserCode]
		public void method_39()
		{
			this[tableReportBillCash.DataColumn_19] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_40()
		{
			return IsNull(tableReportBillCash.DataColumn_20);
		}

		[DebuggerNonUserCode]
		public void method_41()
		{
			this[tableReportBillCash.DataColumn_20] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_42()
		{
			return IsNull(tableReportBillCash.DataColumn_21);
		}

		[DebuggerNonUserCode]
		public void method_43()
		{
			this[tableReportBillCash.DataColumn_21] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_44()
		{
			return IsNull(tableReportBillCash.DataColumn_22);
		}

		[DebuggerNonUserCode]
		public void method_45()
		{
			this[tableReportBillCash.DataColumn_22] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class Bill_HRow : DataRow
	{
		private Bill_HDataTable tableBill_H;

		[DebuggerNonUserCode]
		public string Company_Name
		{
			get
			{
				return Conversions.ToString(this[tableBill_H.Company_nameColumn]);
			}
			set
			{
				this[tableBill_H.Company_nameColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string Company_address
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableBill_H.Company_addressColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'Company_address' in table 'Bill_H' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableBill_H.Company_addressColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string Company_TaxID
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableBill_H.Company_TaxIDColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'Company_TaxID' in table 'Bill_H' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableBill_H.Company_TaxIDColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string Company_Tel
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableBill_H.Company_TelColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'Company_Tel' in table 'Bill_H' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableBill_H.Company_TelColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string Company_Fax
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableBill_H.Company_FaxColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'Company_Fax' in table 'Bill_H' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableBill_H.Company_FaxColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public byte[] Copany_Logo
		{
			get
			{
				try
				{
					return (byte[])this[tableBill_H.Copany_LogoColumn];
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'Copany_Logo' in table 'Bill_H' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableBill_H.Copany_LogoColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		internal Bill_HRow(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableBill_H = (Bill_HDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool IsCompany_addressNull()
		{
			return IsNull(tableBill_H.Company_addressColumn);
		}

		[DebuggerNonUserCode]
		public void SetCompany_addressNull()
		{
			this[tableBill_H.Company_addressColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsCompany_TaxIDNull()
		{
			return IsNull(tableBill_H.Company_TaxIDColumn);
		}

		[DebuggerNonUserCode]
		public void SetCompany_TaxIDNull()
		{
			this[tableBill_H.Company_TaxIDColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsCompany_TelNull()
		{
			return IsNull(tableBill_H.Company_TelColumn);
		}

		[DebuggerNonUserCode]
		public void SetCompany_TelNull()
		{
			this[tableBill_H.Company_TelColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsCompany_FaxNull()
		{
			return IsNull(tableBill_H.Company_FaxColumn);
		}

		[DebuggerNonUserCode]
		public void SetCompany_FaxNull()
		{
			this[tableBill_H.Company_FaxColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsCopany_LogoNull()
		{
			return IsNull(tableBill_H.Copany_LogoColumn);
		}

		[DebuggerNonUserCode]
		public void SetCopany_LogoNull()
		{
			this[tableBill_H.Copany_LogoColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportRegRow : DataRow
	{
		private ReportRegDataTable tableReportReg;

		[DebuggerNonUserCode]
		public string String_0
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'กร\u0e38\u0e4aป' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_1
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เลขท\u0e35\u0e48' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_3
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ห\u0e49อง' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_4
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_4]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ช\u0e37\u0e48อ' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_4] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_5
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_5]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ท\u0e35\u0e48อย\u0e39\u0e48' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_5] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_6
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_6]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'อาช\u0e35พ' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_6] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_7
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_7]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ส\u0e31ญชาต\u0e34' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_7] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_8
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_8]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'มาจาก' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_8] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_9
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_9]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ไปท\u0e35\u0e48' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_9] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_10
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_10]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'หมายเลข' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_10] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_11
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_11]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'โทร' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_11] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_12
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_12]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48ออก' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_12] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_13
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_13]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคา' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_13] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_14
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_14]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคารวม' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_14] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_15
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_15]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ทะเบ\u0e35ยนรถ' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_15] = value;
			}
		}

		[DebuggerNonUserCode]
		public byte[] pic
		{
			get
			{
				try
				{
					return (byte[])this[tableReportReg.picColumn];
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'pic' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.picColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_16
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_16]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รห\u0e31สล\u0e39กค\u0e49า' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_16] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_17
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_17]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'หมายเหต\u0e38' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_17] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_18
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_18]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคาส\u0e34นค\u0e49า' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_18] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_19
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportReg.DataColumn_19]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคารวมส\u0e34นค\u0e49า' in table 'ReportReg' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportReg.DataColumn_19] = value;
			}
		}

		[DebuggerNonUserCode]
		internal ReportRegRow(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReportReg = (ReportRegDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableReportReg.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableReportReg.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableReportReg.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableReportReg.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableReportReg.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void method_5()
		{
			this[tableReportReg.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_6()
		{
			return IsNull(tableReportReg.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_7()
		{
			this[tableReportReg.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_8()
		{
			return IsNull(tableReportReg.DataColumn_4);
		}

		[DebuggerNonUserCode]
		public void method_9()
		{
			this[tableReportReg.DataColumn_4] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_10()
		{
			return IsNull(tableReportReg.DataColumn_5);
		}

		[DebuggerNonUserCode]
		public void method_11()
		{
			this[tableReportReg.DataColumn_5] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_12()
		{
			return IsNull(tableReportReg.DataColumn_6);
		}

		[DebuggerNonUserCode]
		public void method_13()
		{
			this[tableReportReg.DataColumn_6] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_14()
		{
			return IsNull(tableReportReg.DataColumn_7);
		}

		[DebuggerNonUserCode]
		public void method_15()
		{
			this[tableReportReg.DataColumn_7] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_16()
		{
			return IsNull(tableReportReg.DataColumn_8);
		}

		[DebuggerNonUserCode]
		public void method_17()
		{
			this[tableReportReg.DataColumn_8] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_18()
		{
			return IsNull(tableReportReg.DataColumn_9);
		}

		[DebuggerNonUserCode]
		public void method_19()
		{
			this[tableReportReg.DataColumn_9] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_20()
		{
			return IsNull(tableReportReg.DataColumn_10);
		}

		[DebuggerNonUserCode]
		public void method_21()
		{
			this[tableReportReg.DataColumn_10] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_22()
		{
			return IsNull(tableReportReg.DataColumn_11);
		}

		[DebuggerNonUserCode]
		public void method_23()
		{
			this[tableReportReg.DataColumn_11] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_24()
		{
			return IsNull(tableReportReg.DataColumn_12);
		}

		[DebuggerNonUserCode]
		public void method_25()
		{
			this[tableReportReg.DataColumn_12] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_26()
		{
			return IsNull(tableReportReg.DataColumn_13);
		}

		[DebuggerNonUserCode]
		public void method_27()
		{
			this[tableReportReg.DataColumn_13] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_28()
		{
			return IsNull(tableReportReg.DataColumn_14);
		}

		[DebuggerNonUserCode]
		public void method_29()
		{
			this[tableReportReg.DataColumn_14] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_30()
		{
			return IsNull(tableReportReg.DataColumn_15);
		}

		[DebuggerNonUserCode]
		public void method_31()
		{
			this[tableReportReg.DataColumn_15] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IspicNull()
		{
			return IsNull(tableReportReg.picColumn);
		}

		[DebuggerNonUserCode]
		public void SetpicNull()
		{
			this[tableReportReg.picColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_32()
		{
			return IsNull(tableReportReg.DataColumn_16);
		}

		[DebuggerNonUserCode]
		public void method_33()
		{
			this[tableReportReg.DataColumn_16] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_34()
		{
			return IsNull(tableReportReg.DataColumn_17);
		}

		[DebuggerNonUserCode]
		public void method_35()
		{
			this[tableReportReg.DataColumn_17] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_36()
		{
			return IsNull(tableReportReg.DataColumn_18);
		}

		[DebuggerNonUserCode]
		public void method_37()
		{
			this[tableReportReg.DataColumn_18] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_38()
		{
			return IsNull(tableReportReg.DataColumn_19);
		}

		[DebuggerNonUserCode]
		public void method_39()
		{
			this[tableReportReg.DataColumn_19] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportDepRow : DataRow
	{
		private ReportDepDataTable tableReportDep;

		[DebuggerNonUserCode]
		public string String_0
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDep.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'กร\u0e38\u0e4aป' in table 'ReportDep' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDep.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_1
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDep.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48' in table 'ReportDep' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDep.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDep.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เลขท\u0e35\u0e48' in table 'ReportDep' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDep.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_3
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDep.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ห\u0e49อง' in table 'ReportDep' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDep.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_4
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDep.DataColumn_4]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ช\u0e37\u0e48อ' in table 'ReportDep' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDep.DataColumn_4] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_5
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDep.DataColumn_5]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'จำนวนเง\u0e34น' in table 'ReportDep' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDep.DataColumn_5] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_6
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDep.DataColumn_6]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ต\u0e31วอ\u0e31กษร' in table 'ReportDep' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDep.DataColumn_6] = value;
			}
		}

		[DebuggerNonUserCode]
		internal ReportDepRow(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReportDep = (ReportDepDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableReportDep.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableReportDep.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableReportDep.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableReportDep.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableReportDep.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void method_5()
		{
			this[tableReportDep.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_6()
		{
			return IsNull(tableReportDep.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_7()
		{
			this[tableReportDep.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_8()
		{
			return IsNull(tableReportDep.DataColumn_4);
		}

		[DebuggerNonUserCode]
		public void method_9()
		{
			this[tableReportDep.DataColumn_4] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_10()
		{
			return IsNull(tableReportDep.DataColumn_5);
		}

		[DebuggerNonUserCode]
		public void method_11()
		{
			this[tableReportDep.DataColumn_5] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_12()
		{
			return IsNull(tableReportDep.DataColumn_6);
		}

		[DebuggerNonUserCode]
		public void method_13()
		{
			this[tableReportDep.DataColumn_6] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ConfigRow : DataRow
	{
		private ConfigDataTable tableConfig;

		[DebuggerNonUserCode]
		public string ThemeColor
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableConfig.ThemeColorColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ThemeColor' in table 'Config' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableConfig.ThemeColorColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string ServerIP
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableConfig.ServerIPColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ServerIP' in table 'Config' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableConfig.ServerIPColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string ServerPassword
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableConfig.ServerPasswordColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ServerPassword' in table 'Config' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableConfig.ServerPasswordColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string Store_camera
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableConfig.Store_cameraColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'Store_camera' in table 'Config' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableConfig.Store_cameraColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		internal ConfigRow(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableConfig = (ConfigDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool IsThemeColorNull()
		{
			return IsNull(tableConfig.ThemeColorColumn);
		}

		[DebuggerNonUserCode]
		public void SetThemeColorNull()
		{
			this[tableConfig.ThemeColorColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsServerIPNull()
		{
			return IsNull(tableConfig.ServerIPColumn);
		}

		[DebuggerNonUserCode]
		public void SetServerIPNull()
		{
			this[tableConfig.ServerIPColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsServerPasswordNull()
		{
			return IsNull(tableConfig.ServerPasswordColumn);
		}

		[DebuggerNonUserCode]
		public void SetServerPasswordNull()
		{
			this[tableConfig.ServerPasswordColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsStore_cameraNull()
		{
			return IsNull(tableConfig.Store_cameraColumn);
		}

		[DebuggerNonUserCode]
		public void SetStore_cameraNull()
		{
			this[tableConfig.Store_cameraColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportPicRow : DataRow
	{
		private ReportPicDataTable tableReportPic;

		[DebuggerNonUserCode]
		public byte[] PicData
		{
			get
			{
				try
				{
					return (byte[])this[tableReportPic.PicDataColumn];
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'PicData' in table 'ReportPic' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportPic.PicDataColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		internal ReportPicRow(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReportPic = (ReportPicDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool IsPicDataNull()
		{
			return IsNull(tableReportPic.PicDataColumn);
		}

		[DebuggerNonUserCode]
		public void SetPicDataNull()
		{
			this[tableReportPic.PicDataColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportDaysRow : DataRow
	{
		private ReportDaysDataTable tableReportDays;

		[DebuggerNonUserCode]
		public string String_0
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_1
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ลำด\u0e31บ' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ห\u0e49อง' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_3
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ชน\u0e34ดห\u0e49อง' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_4
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_4]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เข\u0e49า' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_4] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_5
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_5]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ออก' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_5] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_6
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_6]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ค\u0e48าห\u0e49อง' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_6] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_7
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_7]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ค\u0e48าส\u0e34นค\u0e49า' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_7] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_8
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_8]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวม' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_8] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_9
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_9]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมห\u0e49อง' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_9] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_10
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_10]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมราคา' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_10] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_11
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_11]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ช\u0e37\u0e48อ' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_11] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_12
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_12]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'จำนวนว\u0e31น' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_12] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_13
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_13]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31ท\u0e35\u0e48ออก' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_13] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_14
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_14]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'หมายเหต\u0e38' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_14] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_15
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_15]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48พ\u0e31กต\u0e48อ' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_15] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_16
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_16]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ชำระแล\u0e49ว' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_16] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_17
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_17]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'คงค\u0e49าง' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_17] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_18
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_18]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมชำระแล\u0e49ว' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_18] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_19
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportDays.DataColumn_19]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมคงค\u0e49าง' in table 'ReportDays' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportDays.DataColumn_19] = value;
			}
		}

		[DebuggerNonUserCode]
		internal ReportDaysRow(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReportDays = (ReportDaysDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableReportDays.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableReportDays.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableReportDays.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableReportDays.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableReportDays.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void method_5()
		{
			this[tableReportDays.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_6()
		{
			return IsNull(tableReportDays.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_7()
		{
			this[tableReportDays.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_8()
		{
			return IsNull(tableReportDays.DataColumn_4);
		}

		[DebuggerNonUserCode]
		public void method_9()
		{
			this[tableReportDays.DataColumn_4] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_10()
		{
			return IsNull(tableReportDays.DataColumn_5);
		}

		[DebuggerNonUserCode]
		public void method_11()
		{
			this[tableReportDays.DataColumn_5] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_12()
		{
			return IsNull(tableReportDays.DataColumn_6);
		}

		[DebuggerNonUserCode]
		public void method_13()
		{
			this[tableReportDays.DataColumn_6] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_14()
		{
			return IsNull(tableReportDays.DataColumn_7);
		}

		[DebuggerNonUserCode]
		public void method_15()
		{
			this[tableReportDays.DataColumn_7] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_16()
		{
			return IsNull(tableReportDays.DataColumn_8);
		}

		[DebuggerNonUserCode]
		public void method_17()
		{
			this[tableReportDays.DataColumn_8] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_18()
		{
			return IsNull(tableReportDays.DataColumn_9);
		}

		[DebuggerNonUserCode]
		public void method_19()
		{
			this[tableReportDays.DataColumn_9] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_20()
		{
			return IsNull(tableReportDays.DataColumn_10);
		}

		[DebuggerNonUserCode]
		public void method_21()
		{
			this[tableReportDays.DataColumn_10] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_22()
		{
			return IsNull(tableReportDays.DataColumn_11);
		}

		[DebuggerNonUserCode]
		public void method_23()
		{
			this[tableReportDays.DataColumn_11] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_24()
		{
			return IsNull(tableReportDays.DataColumn_12);
		}

		[DebuggerNonUserCode]
		public void method_25()
		{
			this[tableReportDays.DataColumn_12] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_26()
		{
			return IsNull(tableReportDays.DataColumn_13);
		}

		[DebuggerNonUserCode]
		public void method_27()
		{
			this[tableReportDays.DataColumn_13] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_28()
		{
			return IsNull(tableReportDays.DataColumn_14);
		}

		[DebuggerNonUserCode]
		public void method_29()
		{
			this[tableReportDays.DataColumn_14] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_30()
		{
			return IsNull(tableReportDays.DataColumn_15);
		}

		[DebuggerNonUserCode]
		public void method_31()
		{
			this[tableReportDays.DataColumn_15] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_32()
		{
			return IsNull(tableReportDays.DataColumn_16);
		}

		[DebuggerNonUserCode]
		public void method_33()
		{
			this[tableReportDays.DataColumn_16] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_34()
		{
			return IsNull(tableReportDays.DataColumn_17);
		}

		[DebuggerNonUserCode]
		public void method_35()
		{
			this[tableReportDays.DataColumn_17] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_36()
		{
			return IsNull(tableReportDays.DataColumn_18);
		}

		[DebuggerNonUserCode]
		public void method_37()
		{
			this[tableReportDays.DataColumn_18] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_38()
		{
			return IsNull(tableReportDays.DataColumn_19);
		}

		[DebuggerNonUserCode]
		public void method_39()
		{
			this[tableReportDays.DataColumn_19] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportCustINRow : DataRow
	{
		private ReportCustINDataTable tableReportCustIN;

		[DebuggerNonUserCode]
		public string String_0
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_1
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ลำด\u0e31บ' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เลขท\u0e35\u0e48' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_3
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เบอร\u0e4cห\u0e49อง' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_4
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_4]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคาห\u0e49อง' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_4] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_5
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_5]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคาส\u0e34นค\u0e49า' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_5] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_6
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_6]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวม' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_6] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_7
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_7]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เข\u0e49า' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_7] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_8
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_8]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ออก' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_8] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_9
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_9]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ส\u0e34นค\u0e49า' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_9] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_10
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_10]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมห\u0e49อง' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_10] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_11
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_11]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมส\u0e34นค\u0e49า' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_11] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_12
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_12]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมท\u0e31\u0e49งหมด' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_12] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_13
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_13]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ช\u0e37\u0e48อ' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_13] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_14
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_14]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'สถานะการจ\u0e48าย' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_14] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_15
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_15]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'พน\u0e31กงาน' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_15] = value;
			}
		}

		[DebuggerNonUserCode]
		public string Balance
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.BalanceColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'Balance' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.BalanceColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_16
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_16]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'หมายเหต\u0e38' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_16] = value;
			}
		}

		[DebuggerNonUserCode]
		public string note2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_17]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'note2' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_17] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_17
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_18]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคาต\u0e48อค\u0e37น' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_18] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_18
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_19]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมราคาต\u0e48อค\u0e37น' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_19] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_19
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_20]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ชำระแล\u0e49ว' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_20] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_20
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_21]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'คงค\u0e49าง' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_21] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_21
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_22]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมชำระแล\u0e49ว' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_22] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_22
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCustIN.DataColumn_23]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมคงค\u0e49าง' in table 'ReportCustIN' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCustIN.DataColumn_23] = value;
			}
		}

		[DebuggerNonUserCode]
		internal ReportCustINRow(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReportCustIN = (ReportCustINDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableReportCustIN.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableReportCustIN.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableReportCustIN.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableReportCustIN.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableReportCustIN.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void method_5()
		{
			this[tableReportCustIN.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_6()
		{
			return IsNull(tableReportCustIN.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_7()
		{
			this[tableReportCustIN.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_8()
		{
			return IsNull(tableReportCustIN.DataColumn_4);
		}

		[DebuggerNonUserCode]
		public void method_9()
		{
			this[tableReportCustIN.DataColumn_4] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_10()
		{
			return IsNull(tableReportCustIN.DataColumn_5);
		}

		[DebuggerNonUserCode]
		public void method_11()
		{
			this[tableReportCustIN.DataColumn_5] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_12()
		{
			return IsNull(tableReportCustIN.DataColumn_6);
		}

		[DebuggerNonUserCode]
		public void method_13()
		{
			this[tableReportCustIN.DataColumn_6] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_14()
		{
			return IsNull(tableReportCustIN.DataColumn_7);
		}

		[DebuggerNonUserCode]
		public void method_15()
		{
			this[tableReportCustIN.DataColumn_7] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_16()
		{
			return IsNull(tableReportCustIN.DataColumn_8);
		}

		[DebuggerNonUserCode]
		public void method_17()
		{
			this[tableReportCustIN.DataColumn_8] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_18()
		{
			return IsNull(tableReportCustIN.DataColumn_9);
		}

		[DebuggerNonUserCode]
		public void method_19()
		{
			this[tableReportCustIN.DataColumn_9] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_20()
		{
			return IsNull(tableReportCustIN.DataColumn_10);
		}

		[DebuggerNonUserCode]
		public void method_21()
		{
			this[tableReportCustIN.DataColumn_10] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_22()
		{
			return IsNull(tableReportCustIN.DataColumn_11);
		}

		[DebuggerNonUserCode]
		public void method_23()
		{
			this[tableReportCustIN.DataColumn_11] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_24()
		{
			return IsNull(tableReportCustIN.DataColumn_12);
		}

		[DebuggerNonUserCode]
		public void method_25()
		{
			this[tableReportCustIN.DataColumn_12] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_26()
		{
			return IsNull(tableReportCustIN.DataColumn_13);
		}

		[DebuggerNonUserCode]
		public void method_27()
		{
			this[tableReportCustIN.DataColumn_13] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_28()
		{
			return IsNull(tableReportCustIN.DataColumn_14);
		}

		[DebuggerNonUserCode]
		public void method_29()
		{
			this[tableReportCustIN.DataColumn_14] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_30()
		{
			return IsNull(tableReportCustIN.DataColumn_15);
		}

		[DebuggerNonUserCode]
		public void method_31()
		{
			this[tableReportCustIN.DataColumn_15] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsBalanceNull()
		{
			return IsNull(tableReportCustIN.BalanceColumn);
		}

		[DebuggerNonUserCode]
		public void SetBalanceNull()
		{
			this[tableReportCustIN.BalanceColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_32()
		{
			return IsNull(tableReportCustIN.DataColumn_16);
		}

		[DebuggerNonUserCode]
		public void method_33()
		{
			this[tableReportCustIN.DataColumn_16] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_34()
		{
			return IsNull(tableReportCustIN.DataColumn_17);
		}

		[DebuggerNonUserCode]
		public void Setnote2Null()
		{
			this[tableReportCustIN.DataColumn_17] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_35()
		{
			return IsNull(tableReportCustIN.DataColumn_18);
		}

		[DebuggerNonUserCode]
		public void method_36()
		{
			this[tableReportCustIN.DataColumn_18] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_37()
		{
			return IsNull(tableReportCustIN.DataColumn_19);
		}

		[DebuggerNonUserCode]
		public void method_38()
		{
			this[tableReportCustIN.DataColumn_19] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_39()
		{
			return IsNull(tableReportCustIN.DataColumn_20);
		}

		[DebuggerNonUserCode]
		public void method_40()
		{
			this[tableReportCustIN.DataColumn_20] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_41()
		{
			return IsNull(tableReportCustIN.DataColumn_21);
		}

		[DebuggerNonUserCode]
		public void method_42()
		{
			this[tableReportCustIN.DataColumn_21] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_43()
		{
			return IsNull(tableReportCustIN.DataColumn_22);
		}

		[DebuggerNonUserCode]
		public void method_44()
		{
			this[tableReportCustIN.DataColumn_22] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_45()
		{
			return IsNull(tableReportCustIN.DataColumn_23);
		}

		[DebuggerNonUserCode]
		public void method_46()
		{
			this[tableReportCustIN.DataColumn_23] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportVatRow : DataRow
	{
		private ReportVatDataTable tableReportVat;

		[DebuggerNonUserCode]
		public string String_0
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ลำด\u0e31บ' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_1
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เลขท\u0e35\u0e48' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_3
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ช\u0e37\u0e48อ' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_4
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_4]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ม\u0e39ลค\u0e48า1' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_4] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_5
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_5]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ม\u0e39ลค\u0e48า2' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_5] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_6
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_6]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ภาษ\u0e351' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_6] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_7
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_7]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ภาษ\u0e352' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_7] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_8
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_8]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมม\u0e39ลค\u0e48า1' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_8] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_9
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_9]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมม\u0e39ลค\u0e48า2' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_9] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_10
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_10]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมภาษ\u0e351' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_10] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_11
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_11]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมภาษ\u0e352' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_11] = value;
			}
		}

		[DebuggerNonUserCode]
		public string newpage
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.newpageColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'newpage' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.newpageColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_12
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_12]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เด\u0e37อน' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_12] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_13
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_13]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ป\u0e35' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_13] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_14
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_14]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ม\u0e39ลค\u0e48า3' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_14] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_15
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_15]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ภาษ\u0e353' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_15] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_16
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_16]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมม\u0e39ลค\u0e48า3' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_16] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_17
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_17]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมภาษ\u0e3523' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_17] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_18
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_18]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เลขประจำต\u0e31ว' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_18] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_19
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.DataColumn_19]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'สาขา' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.DataColumn_19] = value;
			}
		}

		[DebuggerNonUserCode]
		public string vat_name
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.vat_nameColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'vat_name' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.vat_nameColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string vat_tax
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.vat_taxColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'vat_tax' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.vat_taxColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string vat_address
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportVat.vat_addressColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'vat_address' in table 'ReportVat' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportVat.vat_addressColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		internal ReportVatRow(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReportVat = (ReportVatDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableReportVat.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableReportVat.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableReportVat.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableReportVat.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableReportVat.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void method_5()
		{
			this[tableReportVat.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_6()
		{
			return IsNull(tableReportVat.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_7()
		{
			this[tableReportVat.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_8()
		{
			return IsNull(tableReportVat.DataColumn_4);
		}

		[DebuggerNonUserCode]
		public void method_9()
		{
			this[tableReportVat.DataColumn_4] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_10()
		{
			return IsNull(tableReportVat.DataColumn_5);
		}

		[DebuggerNonUserCode]
		public void method_11()
		{
			this[tableReportVat.DataColumn_5] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_12()
		{
			return IsNull(tableReportVat.DataColumn_6);
		}

		[DebuggerNonUserCode]
		public void method_13()
		{
			this[tableReportVat.DataColumn_6] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_14()
		{
			return IsNull(tableReportVat.DataColumn_7);
		}

		[DebuggerNonUserCode]
		public void method_15()
		{
			this[tableReportVat.DataColumn_7] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_16()
		{
			return IsNull(tableReportVat.DataColumn_8);
		}

		[DebuggerNonUserCode]
		public void method_17()
		{
			this[tableReportVat.DataColumn_8] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_18()
		{
			return IsNull(tableReportVat.DataColumn_9);
		}

		[DebuggerNonUserCode]
		public void method_19()
		{
			this[tableReportVat.DataColumn_9] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_20()
		{
			return IsNull(tableReportVat.DataColumn_10);
		}

		[DebuggerNonUserCode]
		public void method_21()
		{
			this[tableReportVat.DataColumn_10] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_22()
		{
			return IsNull(tableReportVat.DataColumn_11);
		}

		[DebuggerNonUserCode]
		public void method_23()
		{
			this[tableReportVat.DataColumn_11] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsnewpageNull()
		{
			return IsNull(tableReportVat.newpageColumn);
		}

		[DebuggerNonUserCode]
		public void SetnewpageNull()
		{
			this[tableReportVat.newpageColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_24()
		{
			return IsNull(tableReportVat.DataColumn_12);
		}

		[DebuggerNonUserCode]
		public void method_25()
		{
			this[tableReportVat.DataColumn_12] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_26()
		{
			return IsNull(tableReportVat.DataColumn_13);
		}

		[DebuggerNonUserCode]
		public void method_27()
		{
			this[tableReportVat.DataColumn_13] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_28()
		{
			return IsNull(tableReportVat.DataColumn_14);
		}

		[DebuggerNonUserCode]
		public void method_29()
		{
			this[tableReportVat.DataColumn_14] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_30()
		{
			return IsNull(tableReportVat.DataColumn_15);
		}

		[DebuggerNonUserCode]
		public void method_31()
		{
			this[tableReportVat.DataColumn_15] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_32()
		{
			return IsNull(tableReportVat.DataColumn_16);
		}

		[DebuggerNonUserCode]
		public void method_33()
		{
			this[tableReportVat.DataColumn_16] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_34()
		{
			return IsNull(tableReportVat.DataColumn_17);
		}

		[DebuggerNonUserCode]
		public void method_35()
		{
			this[tableReportVat.DataColumn_17] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_36()
		{
			return IsNull(tableReportVat.DataColumn_18);
		}

		[DebuggerNonUserCode]
		public void method_37()
		{
			this[tableReportVat.DataColumn_18] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_38()
		{
			return IsNull(tableReportVat.DataColumn_19);
		}

		[DebuggerNonUserCode]
		public void method_39()
		{
			this[tableReportVat.DataColumn_19] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool Isvat_nameNull()
		{
			return IsNull(tableReportVat.vat_nameColumn);
		}

		[DebuggerNonUserCode]
		public void Setvat_nameNull()
		{
			this[tableReportVat.vat_nameColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool Isvat_taxNull()
		{
			return IsNull(tableReportVat.vat_taxColumn);
		}

		[DebuggerNonUserCode]
		public void Setvat_taxNull()
		{
			this[tableReportVat.vat_taxColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool Isvat_addressNull()
		{
			return IsNull(tableReportVat.vat_addressColumn);
		}

		[DebuggerNonUserCode]
		public void Setvat_addressNull()
		{
			this[tableReportVat.vat_addressColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportShiftRow : DataRow
	{
		private ReportShiftDataTable tableReportShift;

		[DebuggerNonUserCode]
		public string String_0
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportShift.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48' in table 'ReportShift' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportShift.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_1
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportShift.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ลำด\u0e31บ' in table 'ReportShift' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportShift.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportShift.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เลขท\u0e35\u0e48' in table 'ReportShift' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportShift.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_3
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportShift.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เบอร\u0e4cห\u0e49อง' in table 'ReportShift' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportShift.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_4
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportShift.DataColumn_4]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ล\u0e39กค\u0e49า' in table 'ReportShift' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportShift.DataColumn_4] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_5
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportShift.DataColumn_5]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เข\u0e49า' in table 'ReportShift' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportShift.DataColumn_5] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_6
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportShift.DataColumn_6]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ออก' in table 'ReportShift' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportShift.DataColumn_6] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_7
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportShift.DataColumn_7]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ล\u0e39กหน\u0e35\u0e49' in table 'ReportShift' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportShift.DataColumn_7] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_8
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportShift.DataColumn_8]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ม\u0e31ดจำ' in table 'ReportShift' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportShift.DataColumn_8] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_9
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportShift.DataColumn_9]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เง\u0e34นสด' in table 'ReportShift' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportShift.DataColumn_9] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_10
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportShift.DataColumn_10]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'บ\u0e31ตร' in table 'ReportShift' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportShift.DataColumn_10] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_11
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportShift.DataColumn_11]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'จ\u0e48ายล\u0e48างหน\u0e49า' in table 'ReportShift' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportShift.DataColumn_11] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_12
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportShift.DataColumn_12]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ค\u0e37นเง\u0e34น' in table 'ReportShift' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportShift.DataColumn_12] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_13
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportShift.DataColumn_13]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'พน\u0e31กงาน' in table 'ReportShift' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportShift.DataColumn_13] = value;
			}
		}

		[DebuggerNonUserCode]
		internal ReportShiftRow(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReportShift = (ReportShiftDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableReportShift.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableReportShift.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableReportShift.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableReportShift.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableReportShift.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void method_5()
		{
			this[tableReportShift.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_6()
		{
			return IsNull(tableReportShift.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_7()
		{
			this[tableReportShift.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_8()
		{
			return IsNull(tableReportShift.DataColumn_4);
		}

		[DebuggerNonUserCode]
		public void method_9()
		{
			this[tableReportShift.DataColumn_4] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_10()
		{
			return IsNull(tableReportShift.DataColumn_5);
		}

		[DebuggerNonUserCode]
		public void method_11()
		{
			this[tableReportShift.DataColumn_5] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_12()
		{
			return IsNull(tableReportShift.DataColumn_6);
		}

		[DebuggerNonUserCode]
		public void method_13()
		{
			this[tableReportShift.DataColumn_6] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_14()
		{
			return IsNull(tableReportShift.DataColumn_7);
		}

		[DebuggerNonUserCode]
		public void method_15()
		{
			this[tableReportShift.DataColumn_7] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_16()
		{
			return IsNull(tableReportShift.DataColumn_8);
		}

		[DebuggerNonUserCode]
		public void method_17()
		{
			this[tableReportShift.DataColumn_8] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_18()
		{
			return IsNull(tableReportShift.DataColumn_9);
		}

		[DebuggerNonUserCode]
		public void method_19()
		{
			this[tableReportShift.DataColumn_9] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_20()
		{
			return IsNull(tableReportShift.DataColumn_10);
		}

		[DebuggerNonUserCode]
		public void method_21()
		{
			this[tableReportShift.DataColumn_10] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_22()
		{
			return IsNull(tableReportShift.DataColumn_11);
		}

		[DebuggerNonUserCode]
		public void method_23()
		{
			this[tableReportShift.DataColumn_11] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_24()
		{
			return IsNull(tableReportShift.DataColumn_12);
		}

		[DebuggerNonUserCode]
		public void method_25()
		{
			this[tableReportShift.DataColumn_12] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_26()
		{
			return IsNull(tableReportShift.DataColumn_13);
		}

		[DebuggerNonUserCode]
		public void method_27()
		{
			this[tableReportShift.DataColumn_13] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportCuponRow : DataRow
	{
		private ReportCuponDataTable tableReportCupon;

		[DebuggerNonUserCode]
		public string String_0
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCupon.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'กร\u0e38\u0e4aบ' in table 'ReportCupon' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCupon.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_1
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCupon.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เลขท\u0e35\u0e48' in table 'ReportCupon' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCupon.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCupon.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48ทำ' in table 'ReportCupon' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCupon.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_3
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCupon.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48ใช\u0e49งาน' in table 'ReportCupon' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCupon.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_4
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCupon.DataColumn_4]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ห\u0e49อง' in table 'ReportCupon' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCupon.DataColumn_4] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_5
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportCupon.DataColumn_5]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ช\u0e37\u0e48อ' in table 'ReportCupon' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportCupon.DataColumn_5] = value;
			}
		}

		[DebuggerNonUserCode]
		internal ReportCuponRow(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReportCupon = (ReportCuponDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableReportCupon.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableReportCupon.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableReportCupon.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableReportCupon.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableReportCupon.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void method_5()
		{
			this[tableReportCupon.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_6()
		{
			return IsNull(tableReportCupon.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_7()
		{
			this[tableReportCupon.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_8()
		{
			return IsNull(tableReportCupon.DataColumn_4);
		}

		[DebuggerNonUserCode]
		public void method_9()
		{
			this[tableReportCupon.DataColumn_4] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_10()
		{
			return IsNull(tableReportCupon.DataColumn_5);
		}

		[DebuggerNonUserCode]
		public void method_11()
		{
			this[tableReportCupon.DataColumn_5] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class TableShiftCashRow : DataRow
	{
		private TableShiftCashDataTable tableTableShiftCash;

		[DebuggerNonUserCode]
		public string String_0
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ห\u0e31ว' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_1
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ท\u0e35\u0e48' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เลขท\u0e35\u0e48' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_3
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48จ\u0e48าย' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_4
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_4]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ลงทะเบ\u0e35ยน' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_4] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_5
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_5]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เบอร\u0e4cห\u0e49อง' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_5] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_6
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_6]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ช\u0e37\u0e48อล\u0e39กค\u0e49า' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_6] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_7
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_7]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48เข\u0e49า' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_7] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_8
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_8]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เรท' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_8] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_9
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_9]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคาห\u0e49อง' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_9] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_10
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_10]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคาส\u0e34นค\u0e49า' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_10] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_11
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_11]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เง\u0e34นสด' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_11] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_12
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_12]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'บ\u0e31ตร' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_12] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_13
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_13]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'พน\u0e31กงาน' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_13] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_14
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_14]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'สร\u0e38ป' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_14] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_15
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_15]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'หน\u0e35\u0e49' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_15] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_16
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_16]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมจ\u0e48ายหน\u0e35\u0e49' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_16] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_17
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_17]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมสด' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_17] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_18
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_18]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมบ\u0e31ตร' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_18] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_19
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_19]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ฟร\u0e35' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_19] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_20
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_20]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ฟร\u0e35ท\u0e31\u0e49งหมด' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_20] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_21
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_21]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ร\u0e31บม\u0e31ดจำ' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_21] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_22
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_22]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ค\u0e37นม\u0e31ดจำ' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_22] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_23
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_23]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมเง\u0e34นท\u0e35\u0e48ต\u0e49องส\u0e48ง' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_23] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_24
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_24]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ล\u0e34\u0e49นช\u0e31ก' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_24] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_25
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_25]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'โอนเง\u0e34น' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_25] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_26
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_26]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'โอนเง\u0e34นท\u0e31\u0e49งหมด' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_26] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_27
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_27]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมเง\u0e34นท\u0e31\u0e49งหมด' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_27] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_28
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_28]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'สาขา' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_28] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_29
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_29]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เว\u0e47บ' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_29] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_30
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableTableShiftCash.DataColumn_30]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เว\u0e47ปท\u0e31\u0e49งหมด' in table 'TableShiftCash' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableTableShiftCash.DataColumn_30] = value;
			}
		}

		[DebuggerNonUserCode]
		internal TableShiftCashRow(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableTableShiftCash = (TableShiftCashDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableTableShiftCash.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableTableShiftCash.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableTableShiftCash.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableTableShiftCash.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableTableShiftCash.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void method_5()
		{
			this[tableTableShiftCash.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_6()
		{
			return IsNull(tableTableShiftCash.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_7()
		{
			this[tableTableShiftCash.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_8()
		{
			return IsNull(tableTableShiftCash.DataColumn_4);
		}

		[DebuggerNonUserCode]
		public void method_9()
		{
			this[tableTableShiftCash.DataColumn_4] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_10()
		{
			return IsNull(tableTableShiftCash.DataColumn_5);
		}

		[DebuggerNonUserCode]
		public void method_11()
		{
			this[tableTableShiftCash.DataColumn_5] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_12()
		{
			return IsNull(tableTableShiftCash.DataColumn_6);
		}

		[DebuggerNonUserCode]
		public void method_13()
		{
			this[tableTableShiftCash.DataColumn_6] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_14()
		{
			return IsNull(tableTableShiftCash.DataColumn_7);
		}

		[DebuggerNonUserCode]
		public void method_15()
		{
			this[tableTableShiftCash.DataColumn_7] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_16()
		{
			return IsNull(tableTableShiftCash.DataColumn_8);
		}

		[DebuggerNonUserCode]
		public void method_17()
		{
			this[tableTableShiftCash.DataColumn_8] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_18()
		{
			return IsNull(tableTableShiftCash.DataColumn_9);
		}

		[DebuggerNonUserCode]
		public void method_19()
		{
			this[tableTableShiftCash.DataColumn_9] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_20()
		{
			return IsNull(tableTableShiftCash.DataColumn_10);
		}

		[DebuggerNonUserCode]
		public void method_21()
		{
			this[tableTableShiftCash.DataColumn_10] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_22()
		{
			return IsNull(tableTableShiftCash.DataColumn_11);
		}

		[DebuggerNonUserCode]
		public void method_23()
		{
			this[tableTableShiftCash.DataColumn_11] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_24()
		{
			return IsNull(tableTableShiftCash.DataColumn_12);
		}

		[DebuggerNonUserCode]
		public void method_25()
		{
			this[tableTableShiftCash.DataColumn_12] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_26()
		{
			return IsNull(tableTableShiftCash.DataColumn_13);
		}

		[DebuggerNonUserCode]
		public void method_27()
		{
			this[tableTableShiftCash.DataColumn_13] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_28()
		{
			return IsNull(tableTableShiftCash.DataColumn_14);
		}

		[DebuggerNonUserCode]
		public void method_29()
		{
			this[tableTableShiftCash.DataColumn_14] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_30()
		{
			return IsNull(tableTableShiftCash.DataColumn_15);
		}

		[DebuggerNonUserCode]
		public void method_31()
		{
			this[tableTableShiftCash.DataColumn_15] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_32()
		{
			return IsNull(tableTableShiftCash.DataColumn_16);
		}

		[DebuggerNonUserCode]
		public void method_33()
		{
			this[tableTableShiftCash.DataColumn_16] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_34()
		{
			return IsNull(tableTableShiftCash.DataColumn_17);
		}

		[DebuggerNonUserCode]
		public void method_35()
		{
			this[tableTableShiftCash.DataColumn_17] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_36()
		{
			return IsNull(tableTableShiftCash.DataColumn_18);
		}

		[DebuggerNonUserCode]
		public void method_37()
		{
			this[tableTableShiftCash.DataColumn_18] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_38()
		{
			return IsNull(tableTableShiftCash.DataColumn_19);
		}

		[DebuggerNonUserCode]
		public void method_39()
		{
			this[tableTableShiftCash.DataColumn_19] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_40()
		{
			return IsNull(tableTableShiftCash.DataColumn_20);
		}

		[DebuggerNonUserCode]
		public void method_41()
		{
			this[tableTableShiftCash.DataColumn_20] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_42()
		{
			return IsNull(tableTableShiftCash.DataColumn_21);
		}

		[DebuggerNonUserCode]
		public void method_43()
		{
			this[tableTableShiftCash.DataColumn_21] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_44()
		{
			return IsNull(tableTableShiftCash.DataColumn_22);
		}

		[DebuggerNonUserCode]
		public void method_45()
		{
			this[tableTableShiftCash.DataColumn_22] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_46()
		{
			return IsNull(tableTableShiftCash.DataColumn_23);
		}

		[DebuggerNonUserCode]
		public void method_47()
		{
			this[tableTableShiftCash.DataColumn_23] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_48()
		{
			return IsNull(tableTableShiftCash.DataColumn_24);
		}

		[DebuggerNonUserCode]
		public void method_49()
		{
			this[tableTableShiftCash.DataColumn_24] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_50()
		{
			return IsNull(tableTableShiftCash.DataColumn_25);
		}

		[DebuggerNonUserCode]
		public void method_51()
		{
			this[tableTableShiftCash.DataColumn_25] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_52()
		{
			return IsNull(tableTableShiftCash.DataColumn_26);
		}

		[DebuggerNonUserCode]
		public void method_53()
		{
			this[tableTableShiftCash.DataColumn_26] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_54()
		{
			return IsNull(tableTableShiftCash.DataColumn_27);
		}

		[DebuggerNonUserCode]
		public void method_55()
		{
			this[tableTableShiftCash.DataColumn_27] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_56()
		{
			return IsNull(tableTableShiftCash.DataColumn_28);
		}

		[DebuggerNonUserCode]
		public void method_57()
		{
			this[tableTableShiftCash.DataColumn_28] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_58()
		{
			return IsNull(tableTableShiftCash.DataColumn_29);
		}

		[DebuggerNonUserCode]
		public void method_59()
		{
			this[tableTableShiftCash.DataColumn_29] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_60()
		{
			return IsNull(tableTableShiftCash.DataColumn_30);
		}

		[DebuggerNonUserCode]
		public void method_61()
		{
			this[tableTableShiftCash.DataColumn_30] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportBillCreditRow : DataRow
	{
		private ReportBillCreditDataTable tableReportBillCredit;

		[DebuggerNonUserCode]
		public string String_0
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCredit.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เลขท\u0e35\u0e48' in table 'ReportBillCredit' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCredit.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_1
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCredit.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48' in table 'ReportBillCredit' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCredit.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCredit.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รห\u0e31สล\u0e39กค\u0e49า' in table 'ReportBillCredit' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCredit.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_3
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCredit.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ช\u0e37\u0e48อ' in table 'ReportBillCredit' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCredit.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_4
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCredit.DataColumn_4]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ท\u0e35\u0e48อย\u0e39\u0e48' in table 'ReportBillCredit' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCredit.DataColumn_4] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_5
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCredit.DataColumn_5]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'โทร' in table 'ReportBillCredit' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCredit.DataColumn_5] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_6
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCredit.DataColumn_6]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'พน\u0e31กงานขาย' in table 'ReportBillCredit' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCredit.DataColumn_6] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_7
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCredit.DataColumn_7]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ท\u0e35\u0e48' in table 'ReportBillCredit' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCredit.DataColumn_7] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_8
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCredit.DataColumn_8]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รห\u0e31สส\u0e34นค\u0e49า' in table 'ReportBillCredit' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCredit.DataColumn_8] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_9
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCredit.DataColumn_9]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รายการ' in table 'ReportBillCredit' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCredit.DataColumn_9] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_10
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCredit.DataColumn_10]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ขนาดบรรจ\u0e38' in table 'ReportBillCredit' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCredit.DataColumn_10] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_11
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCredit.DataColumn_11]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'จำนวน' in table 'ReportBillCredit' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCredit.DataColumn_11] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_12
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCredit.DataColumn_12]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคาต\u0e48อหน\u0e48วย' in table 'ReportBillCredit' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCredit.DataColumn_12] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_13
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCredit.DataColumn_13]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'จำนวนเง\u0e34น' in table 'ReportBillCredit' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCredit.DataColumn_13] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_14
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCredit.DataColumn_14]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคารวม' in table 'ReportBillCredit' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCredit.DataColumn_14] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_15
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCredit.DataColumn_15]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคาต\u0e31วหน\u0e31งส\u0e37อ' in table 'ReportBillCredit' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCredit.DataColumn_15] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_16
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBillCredit.DataColumn_16]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมจำนวน' in table 'ReportBillCredit' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBillCredit.DataColumn_16] = value;
			}
		}

		[DebuggerNonUserCode]
		internal ReportBillCreditRow(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReportBillCredit = (ReportBillCreditDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableReportBillCredit.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableReportBillCredit.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableReportBillCredit.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableReportBillCredit.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableReportBillCredit.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void method_5()
		{
			this[tableReportBillCredit.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_6()
		{
			return IsNull(tableReportBillCredit.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_7()
		{
			this[tableReportBillCredit.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_8()
		{
			return IsNull(tableReportBillCredit.DataColumn_4);
		}

		[DebuggerNonUserCode]
		public void method_9()
		{
			this[tableReportBillCredit.DataColumn_4] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_10()
		{
			return IsNull(tableReportBillCredit.DataColumn_5);
		}

		[DebuggerNonUserCode]
		public void method_11()
		{
			this[tableReportBillCredit.DataColumn_5] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_12()
		{
			return IsNull(tableReportBillCredit.DataColumn_6);
		}

		[DebuggerNonUserCode]
		public void method_13()
		{
			this[tableReportBillCredit.DataColumn_6] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_14()
		{
			return IsNull(tableReportBillCredit.DataColumn_7);
		}

		[DebuggerNonUserCode]
		public void method_15()
		{
			this[tableReportBillCredit.DataColumn_7] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_16()
		{
			return IsNull(tableReportBillCredit.DataColumn_8);
		}

		[DebuggerNonUserCode]
		public void method_17()
		{
			this[tableReportBillCredit.DataColumn_8] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_18()
		{
			return IsNull(tableReportBillCredit.DataColumn_9);
		}

		[DebuggerNonUserCode]
		public void method_19()
		{
			this[tableReportBillCredit.DataColumn_9] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_20()
		{
			return IsNull(tableReportBillCredit.DataColumn_10);
		}

		[DebuggerNonUserCode]
		public void method_21()
		{
			this[tableReportBillCredit.DataColumn_10] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_22()
		{
			return IsNull(tableReportBillCredit.DataColumn_11);
		}

		[DebuggerNonUserCode]
		public void method_23()
		{
			this[tableReportBillCredit.DataColumn_11] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_24()
		{
			return IsNull(tableReportBillCredit.DataColumn_12);
		}

		[DebuggerNonUserCode]
		public void method_25()
		{
			this[tableReportBillCredit.DataColumn_12] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_26()
		{
			return IsNull(tableReportBillCredit.DataColumn_13);
		}

		[DebuggerNonUserCode]
		public void method_27()
		{
			this[tableReportBillCredit.DataColumn_13] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_28()
		{
			return IsNull(tableReportBillCredit.DataColumn_14);
		}

		[DebuggerNonUserCode]
		public void method_29()
		{
			this[tableReportBillCredit.DataColumn_14] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_30()
		{
			return IsNull(tableReportBillCredit.DataColumn_15);
		}

		[DebuggerNonUserCode]
		public void method_31()
		{
			this[tableReportBillCredit.DataColumn_15] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_32()
		{
			return IsNull(tableReportBillCredit.DataColumn_16);
		}

		[DebuggerNonUserCode]
		public void method_33()
		{
			this[tableReportBillCredit.DataColumn_16] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class Report_Room_allRow : DataRow
	{
		private Report_Room_allDataTable tableReport_Room_all;

		[DebuggerNonUserCode]
		public string head
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Room_all.headColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'head' in table 'Report_Room_all' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Room_all.headColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_0
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Room_all.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'กร\u0e38\u0e4aป' in table 'Report_Room_all' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Room_all.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_1
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Room_all.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รายการ' in table 'Report_Room_all' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Room_all.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Room_all.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31น' in table 'Report_Room_all' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Room_all.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_3
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Room_all.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เด\u0e37อน' in table 'Report_Room_all' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Room_all.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_4
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Room_all.DataColumn_4]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ป\u0e35' in table 'Report_Room_all' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Room_all.DataColumn_4] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_5
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Room_all.DataColumn_5]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ป\u0e35ท\u0e35\u0e48แล\u0e49ว' in table 'Report_Room_all' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Room_all.DataColumn_5] = value;
			}
		}

		[DebuggerNonUserCode]
		internal Report_Room_allRow(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReport_Room_all = (Report_Room_allDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool IsheadNull()
		{
			return IsNull(tableReport_Room_all.headColumn);
		}

		[DebuggerNonUserCode]
		public void SetheadNull()
		{
			this[tableReport_Room_all.headColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableReport_Room_all.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableReport_Room_all.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableReport_Room_all.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableReport_Room_all.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableReport_Room_all.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void method_5()
		{
			this[tableReport_Room_all.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_6()
		{
			return IsNull(tableReport_Room_all.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_7()
		{
			this[tableReport_Room_all.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_8()
		{
			return IsNull(tableReport_Room_all.DataColumn_4);
		}

		[DebuggerNonUserCode]
		public void method_9()
		{
			this[tableReport_Room_all.DataColumn_4] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_10()
		{
			return IsNull(tableReport_Room_all.DataColumn_5);
		}

		[DebuggerNonUserCode]
		public void method_11()
		{
			this[tableReport_Room_all.DataColumn_5] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class Report_Debt_INVRow : DataRow
	{
		private Report_Debt_INVDataTable tableReport_Debt_INV;

		[DebuggerNonUserCode]
		public string String_0
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เลขท\u0e35\u0e48' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_1
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'หมายเหลขห\u0e49อง' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_3
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ช\u0e37\u0e48อ' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_4
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_4]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ท\u0e35\u0e48อย\u0e39\u0e48' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_4] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_5
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_5]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'โทร' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_5] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_6
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_6]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ท\u0e35\u0e48' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_6] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_7
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_7]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รายการ' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_7] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_8
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_8]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'จำนวน' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_8] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_9
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_9]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'หน\u0e48วย' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_9] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_10
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_10]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคา' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_10] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_11
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_11]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคารวม' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_11] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_12
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_12]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ชำระแล\u0e49ว' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_12] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_13
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_13]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ค\u0e49างชำระ' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_13] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_14
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_14]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมค\u0e49างชำระ' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_14] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_15
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_15]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมราคาท\u0e31\u0e49งหมด' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_15] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_16
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_16]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมชำระแล\u0e49ว' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_16] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_17
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_17]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'หมายเหต\u0e38' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_17] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_18
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_18]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เง\u0e34นอ\u0e31กษร' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_18] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_19
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_19]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นออก' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_19] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_20
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_20]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ภาษ\u0e35per' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_20] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_21
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_21]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคาภาษ\u0e35' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_21] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_22
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_22]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคาก\u0e48อนภาษ\u0e35' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_22] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_23
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_23]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'จ\u0e48ายคร\u0e31\u0e49งน\u0e35\u0e49' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_23] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_24
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReport_Debt_INV.DataColumn_24]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมจ\u0e48ายคร\u0e31\u0e49งน\u0e35\u0e49' in table 'Report_Debt_INV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReport_Debt_INV.DataColumn_24] = value;
			}
		}

		[DebuggerNonUserCode]
		internal Report_Debt_INVRow(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReport_Debt_INV = (Report_Debt_INVDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableReport_Debt_INV.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableReport_Debt_INV.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void method_5()
		{
			this[tableReport_Debt_INV.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_6()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_7()
		{
			this[tableReport_Debt_INV.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_8()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_4);
		}

		[DebuggerNonUserCode]
		public void method_9()
		{
			this[tableReport_Debt_INV.DataColumn_4] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_10()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_5);
		}

		[DebuggerNonUserCode]
		public void method_11()
		{
			this[tableReport_Debt_INV.DataColumn_5] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_12()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_6);
		}

		[DebuggerNonUserCode]
		public void method_13()
		{
			this[tableReport_Debt_INV.DataColumn_6] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_14()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_7);
		}

		[DebuggerNonUserCode]
		public void method_15()
		{
			this[tableReport_Debt_INV.DataColumn_7] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_16()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_8);
		}

		[DebuggerNonUserCode]
		public void method_17()
		{
			this[tableReport_Debt_INV.DataColumn_8] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_18()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_9);
		}

		[DebuggerNonUserCode]
		public void method_19()
		{
			this[tableReport_Debt_INV.DataColumn_9] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_20()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_10);
		}

		[DebuggerNonUserCode]
		public void method_21()
		{
			this[tableReport_Debt_INV.DataColumn_10] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_22()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_11);
		}

		[DebuggerNonUserCode]
		public void method_23()
		{
			this[tableReport_Debt_INV.DataColumn_11] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_24()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_12);
		}

		[DebuggerNonUserCode]
		public void method_25()
		{
			this[tableReport_Debt_INV.DataColumn_12] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_26()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_13);
		}

		[DebuggerNonUserCode]
		public void method_27()
		{
			this[tableReport_Debt_INV.DataColumn_13] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_28()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_14);
		}

		[DebuggerNonUserCode]
		public void method_29()
		{
			this[tableReport_Debt_INV.DataColumn_14] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_30()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_15);
		}

		[DebuggerNonUserCode]
		public void method_31()
		{
			this[tableReport_Debt_INV.DataColumn_15] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_32()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_16);
		}

		[DebuggerNonUserCode]
		public void method_33()
		{
			this[tableReport_Debt_INV.DataColumn_16] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_34()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_17);
		}

		[DebuggerNonUserCode]
		public void method_35()
		{
			this[tableReport_Debt_INV.DataColumn_17] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_36()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_18);
		}

		[DebuggerNonUserCode]
		public void method_37()
		{
			this[tableReport_Debt_INV.DataColumn_18] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_38()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_19);
		}

		[DebuggerNonUserCode]
		public void method_39()
		{
			this[tableReport_Debt_INV.DataColumn_19] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_40()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_20);
		}

		[DebuggerNonUserCode]
		public void method_41()
		{
			this[tableReport_Debt_INV.DataColumn_20] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_42()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_21);
		}

		[DebuggerNonUserCode]
		public void method_43()
		{
			this[tableReport_Debt_INV.DataColumn_21] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_44()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_22);
		}

		[DebuggerNonUserCode]
		public void method_45()
		{
			this[tableReport_Debt_INV.DataColumn_22] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_46()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_23);
		}

		[DebuggerNonUserCode]
		public void method_47()
		{
			this[tableReport_Debt_INV.DataColumn_23] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_48()
		{
			return IsNull(tableReport_Debt_INV.DataColumn_24);
		}

		[DebuggerNonUserCode]
		public void method_49()
		{
			this[tableReport_Debt_INV.DataColumn_24] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportFolio1Row : DataRow
	{
		private ReportFolio1DataTable tableReportFolio1;

		[DebuggerNonUserCode]
		public string String_0
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ช\u0e37\u0e48อ' in table 'ReportFolio1' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_1
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'จำนวนคน' in table 'ReportFolio1' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'อ\u0e31ตรค\u0e48าเช\u0e48า' in table 'ReportFolio1' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_3
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'อ\u0e31ตราภาษ\u0e35' in table 'ReportFolio1' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_4
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1.DataColumn_4]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เลขห\u0e49อง' in table 'ReportFolio1' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1.DataColumn_4] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_5
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1.DataColumn_5]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48เข\u0e49า' in table 'ReportFolio1' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1.DataColumn_5] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_6
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1.DataColumn_6]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48ออก' in table 'ReportFolio1' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1.DataColumn_6] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_7
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1.DataColumn_7]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'จำนวนว\u0e31น' in table 'ReportFolio1' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1.DataColumn_7] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_8
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1.DataColumn_8]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'อ\u0e31ตราค\u0e48าเช\u0e48า' in table 'ReportFolio1' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1.DataColumn_8] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_9
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1.DataColumn_9]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'จำนวนเง\u0e34น' in table 'ReportFolio1' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1.DataColumn_9] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_10
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1.DataColumn_10]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมเง\u0e34นท\u0e31\u0e49งหมด' in table 'ReportFolio1' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1.DataColumn_10] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_11
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1.DataColumn_11]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เลขin' in table 'ReportFolio1' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1.DataColumn_11] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_12
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1.DataColumn_12]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ภาษ\u0e35PER' in table 'ReportFolio1' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1.DataColumn_12] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_13
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1.DataColumn_13]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคาภาษ\u0e35' in table 'ReportFolio1' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1.DataColumn_13] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_14
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1.DataColumn_14]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคาก\u0e48อนvat' in table 'ReportFolio1' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1.DataColumn_14] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_15
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1.DataColumn_15]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ต\u0e31วอ\u0e31กษร' in table 'ReportFolio1' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1.DataColumn_15] = value;
			}
		}

		[DebuggerNonUserCode]
		internal ReportFolio1Row(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReportFolio1 = (ReportFolio1DataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableReportFolio1.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableReportFolio1.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableReportFolio1.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableReportFolio1.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableReportFolio1.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void method_5()
		{
			this[tableReportFolio1.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_6()
		{
			return IsNull(tableReportFolio1.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_7()
		{
			this[tableReportFolio1.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_8()
		{
			return IsNull(tableReportFolio1.DataColumn_4);
		}

		[DebuggerNonUserCode]
		public void method_9()
		{
			this[tableReportFolio1.DataColumn_4] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_10()
		{
			return IsNull(tableReportFolio1.DataColumn_5);
		}

		[DebuggerNonUserCode]
		public void method_11()
		{
			this[tableReportFolio1.DataColumn_5] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_12()
		{
			return IsNull(tableReportFolio1.DataColumn_6);
		}

		[DebuggerNonUserCode]
		public void method_13()
		{
			this[tableReportFolio1.DataColumn_6] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_14()
		{
			return IsNull(tableReportFolio1.DataColumn_7);
		}

		[DebuggerNonUserCode]
		public void method_15()
		{
			this[tableReportFolio1.DataColumn_7] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_16()
		{
			return IsNull(tableReportFolio1.DataColumn_8);
		}

		[DebuggerNonUserCode]
		public void method_17()
		{
			this[tableReportFolio1.DataColumn_8] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_18()
		{
			return IsNull(tableReportFolio1.DataColumn_9);
		}

		[DebuggerNonUserCode]
		public void method_19()
		{
			this[tableReportFolio1.DataColumn_9] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_20()
		{
			return IsNull(tableReportFolio1.DataColumn_10);
		}

		[DebuggerNonUserCode]
		public void method_21()
		{
			this[tableReportFolio1.DataColumn_10] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_22()
		{
			return IsNull(tableReportFolio1.DataColumn_11);
		}

		[DebuggerNonUserCode]
		public void method_23()
		{
			this[tableReportFolio1.DataColumn_11] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_24()
		{
			return IsNull(tableReportFolio1.DataColumn_12);
		}

		[DebuggerNonUserCode]
		public void method_25()
		{
			this[tableReportFolio1.DataColumn_12] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_26()
		{
			return IsNull(tableReportFolio1.DataColumn_13);
		}

		[DebuggerNonUserCode]
		public void method_27()
		{
			this[tableReportFolio1.DataColumn_13] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_28()
		{
			return IsNull(tableReportFolio1.DataColumn_14);
		}

		[DebuggerNonUserCode]
		public void method_29()
		{
			this[tableReportFolio1.DataColumn_14] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_30()
		{
			return IsNull(tableReportFolio1.DataColumn_15);
		}

		[DebuggerNonUserCode]
		public void method_31()
		{
			this[tableReportFolio1.DataColumn_15] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportFolio1_2Row : DataRow
	{
		private ReportFolio1_2DataTable tableReportFolio1_2;

		[DebuggerNonUserCode]
		public string String_0
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1_2.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ส\u0e34นค\u0e49า' in table 'ReportFolio1_2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1_2.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_1
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1_2.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'จำนวน' in table 'ReportFolio1_2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1_2.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1_2.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคา' in table 'ReportFolio1_2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1_2.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_3
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio1_2.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคารวม' in table 'ReportFolio1_2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio1_2.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		internal ReportFolio1_2Row(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReportFolio1_2 = (ReportFolio1_2DataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableReportFolio1_2.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableReportFolio1_2.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableReportFolio1_2.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableReportFolio1_2.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableReportFolio1_2.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void method_5()
		{
			this[tableReportFolio1_2.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_6()
		{
			return IsNull(tableReportFolio1_2.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_7()
		{
			this[tableReportFolio1_2.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportFolio2Row : DataRow
	{
		private ReportFolio2DataTable tableReportFolio2;

		[DebuggerNonUserCode]
		public string String_0
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio2.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ห\u0e31ว1' in table 'ReportFolio2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio2.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_1
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio2.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ห\u0e31ว2' in table 'ReportFolio2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio2.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio2.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ห\u0e31ว3' in table 'ReportFolio2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio2.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_3
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio2.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ลำด\u0e31บท\u0e35\u0e48' in table 'ReportFolio2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio2.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_4
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio2.DataColumn_4]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เลขห\u0e49อง' in table 'ReportFolio2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio2.DataColumn_4] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_5
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio2.DataColumn_5]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ช\u0e37\u0e48อ' in table 'ReportFolio2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio2.DataColumn_5] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_6
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio2.DataColumn_6]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เข\u0e49า' in table 'ReportFolio2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio2.DataColumn_6] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_7
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio2.DataColumn_7]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ออก' in table 'ReportFolio2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio2.DataColumn_7] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_8
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio2.DataColumn_8]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ค\u0e37น' in table 'ReportFolio2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio2.DataColumn_8] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_9
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio2.DataColumn_9]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคา' in table 'ReportFolio2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio2.DataColumn_9] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_10
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio2.DataColumn_10]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคารวม' in table 'ReportFolio2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio2.DataColumn_10] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_11
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio2.DataColumn_11]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคารวมท\u0e31\u0e49งหมด' in table 'ReportFolio2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio2.DataColumn_11] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_12
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio2.DataColumn_12]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เลขin' in table 'ReportFolio2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio2.DataColumn_12] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_13
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio2.DataColumn_13]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ภาษ\u0e35PER' in table 'ReportFolio2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio2.DataColumn_13] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_14
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio2.DataColumn_14]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคาภาษ\u0e35' in table 'ReportFolio2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio2.DataColumn_14] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_15
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio2.DataColumn_15]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคาก\u0e48อนvat' in table 'ReportFolio2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio2.DataColumn_15] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_16
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportFolio2.DataColumn_16]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ต\u0e31วอ\u0e31กษร' in table 'ReportFolio2' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportFolio2.DataColumn_16] = value;
			}
		}

		[DebuggerNonUserCode]
		internal ReportFolio2Row(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReportFolio2 = (ReportFolio2DataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableReportFolio2.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableReportFolio2.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableReportFolio2.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableReportFolio2.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableReportFolio2.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void method_5()
		{
			this[tableReportFolio2.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_6()
		{
			return IsNull(tableReportFolio2.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_7()
		{
			this[tableReportFolio2.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_8()
		{
			return IsNull(tableReportFolio2.DataColumn_4);
		}

		[DebuggerNonUserCode]
		public void method_9()
		{
			this[tableReportFolio2.DataColumn_4] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_10()
		{
			return IsNull(tableReportFolio2.DataColumn_5);
		}

		[DebuggerNonUserCode]
		public void method_11()
		{
			this[tableReportFolio2.DataColumn_5] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_12()
		{
			return IsNull(tableReportFolio2.DataColumn_6);
		}

		[DebuggerNonUserCode]
		public void method_13()
		{
			this[tableReportFolio2.DataColumn_6] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_14()
		{
			return IsNull(tableReportFolio2.DataColumn_7);
		}

		[DebuggerNonUserCode]
		public void method_15()
		{
			this[tableReportFolio2.DataColumn_7] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_16()
		{
			return IsNull(tableReportFolio2.DataColumn_8);
		}

		[DebuggerNonUserCode]
		public void method_17()
		{
			this[tableReportFolio2.DataColumn_8] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_18()
		{
			return IsNull(tableReportFolio2.DataColumn_9);
		}

		[DebuggerNonUserCode]
		public void method_19()
		{
			this[tableReportFolio2.DataColumn_9] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_20()
		{
			return IsNull(tableReportFolio2.DataColumn_10);
		}

		[DebuggerNonUserCode]
		public void method_21()
		{
			this[tableReportFolio2.DataColumn_10] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_22()
		{
			return IsNull(tableReportFolio2.DataColumn_11);
		}

		[DebuggerNonUserCode]
		public void method_23()
		{
			this[tableReportFolio2.DataColumn_11] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_24()
		{
			return IsNull(tableReportFolio2.DataColumn_12);
		}

		[DebuggerNonUserCode]
		public void method_25()
		{
			this[tableReportFolio2.DataColumn_12] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_26()
		{
			return IsNull(tableReportFolio2.DataColumn_13);
		}

		[DebuggerNonUserCode]
		public void method_27()
		{
			this[tableReportFolio2.DataColumn_13] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_28()
		{
			return IsNull(tableReportFolio2.DataColumn_14);
		}

		[DebuggerNonUserCode]
		public void method_29()
		{
			this[tableReportFolio2.DataColumn_14] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_30()
		{
			return IsNull(tableReportFolio2.DataColumn_15);
		}

		[DebuggerNonUserCode]
		public void method_31()
		{
			this[tableReportFolio2.DataColumn_15] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_32()
		{
			return IsNull(tableReportFolio2.DataColumn_16);
		}

		[DebuggerNonUserCode]
		public void method_33()
		{
			this[tableReportFolio2.DataColumn_16] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportSaleRow : DataRow
	{
		private ReportSaleDataTable tableReportSale;

		[DebuggerNonUserCode]
		public string String_0
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportSale.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ลำด\u0e31บ' in table 'ReportSale' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportSale.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_1
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportSale.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รห\u0e31ส' in table 'ReportSale' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportSale.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string Name
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportSale.NameColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'Name' in table 'ReportSale' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportSale.NameColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string num
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportSale.numColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'num' in table 'ReportSale' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportSale.numColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportSale.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคา' in table 'ReportSale' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportSale.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_3
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportSale.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคารวม' in table 'ReportSale' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportSale.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_4
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportSale.DataColumn_4]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมจำนวน' in table 'ReportSale' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportSale.DataColumn_4] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_5
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportSale.DataColumn_5]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมราคาท\u0e31\u0e49งหมด' in table 'ReportSale' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportSale.DataColumn_5] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_6
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportSale.DataColumn_6]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ห\u0e31ว' in table 'ReportSale' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportSale.DataColumn_6] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_7
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportSale.DataColumn_7]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ช\u0e37\u0e48อล\u0e39กค\u0e49า' in table 'ReportSale' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportSale.DataColumn_7] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_8
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportSale.DataColumn_8]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคาท\u0e38น' in table 'ReportSale' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportSale.DataColumn_8] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_9
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportSale.DataColumn_9]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ราคาท\u0e38นค\u0e39ณ' in table 'ReportSale' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportSale.DataColumn_9] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_10
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportSale.DataColumn_10]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'กำไร' in table 'ReportSale' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportSale.DataColumn_10] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_11
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportSale.DataColumn_11]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมท\u0e38น' in table 'ReportSale' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportSale.DataColumn_11] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_12
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportSale.DataColumn_12]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'รวมกำไร' in table 'ReportSale' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportSale.DataColumn_12] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_13
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportSale.DataColumn_13]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เลขท\u0e35\u0e48บ\u0e34ล' in table 'ReportSale' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportSale.DataColumn_13] = value;
			}
		}

		[DebuggerNonUserCode]
		internal ReportSaleRow(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReportSale = (ReportSaleDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableReportSale.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableReportSale.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableReportSale.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableReportSale.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsNameNull()
		{
			return IsNull(tableReportSale.NameColumn);
		}

		[DebuggerNonUserCode]
		public void SetNameNull()
		{
			this[tableReportSale.NameColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsnumNull()
		{
			return IsNull(tableReportSale.numColumn);
		}

		[DebuggerNonUserCode]
		public void SetnumNull()
		{
			this[tableReportSale.numColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableReportSale.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void method_5()
		{
			this[tableReportSale.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_6()
		{
			return IsNull(tableReportSale.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_7()
		{
			this[tableReportSale.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_8()
		{
			return IsNull(tableReportSale.DataColumn_4);
		}

		[DebuggerNonUserCode]
		public void method_9()
		{
			this[tableReportSale.DataColumn_4] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_10()
		{
			return IsNull(tableReportSale.DataColumn_5);
		}

		[DebuggerNonUserCode]
		public void method_11()
		{
			this[tableReportSale.DataColumn_5] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_12()
		{
			return IsNull(tableReportSale.DataColumn_6);
		}

		[DebuggerNonUserCode]
		public void method_13()
		{
			this[tableReportSale.DataColumn_6] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_14()
		{
			return IsNull(tableReportSale.DataColumn_7);
		}

		[DebuggerNonUserCode]
		public void method_15()
		{
			this[tableReportSale.DataColumn_7] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_16()
		{
			return IsNull(tableReportSale.DataColumn_8);
		}

		[DebuggerNonUserCode]
		public void method_17()
		{
			this[tableReportSale.DataColumn_8] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_18()
		{
			return IsNull(tableReportSale.DataColumn_9);
		}

		[DebuggerNonUserCode]
		public void method_19()
		{
			this[tableReportSale.DataColumn_9] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_20()
		{
			return IsNull(tableReportSale.DataColumn_10);
		}

		[DebuggerNonUserCode]
		public void method_21()
		{
			this[tableReportSale.DataColumn_10] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_22()
		{
			return IsNull(tableReportSale.DataColumn_11);
		}

		[DebuggerNonUserCode]
		public void method_23()
		{
			this[tableReportSale.DataColumn_11] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_24()
		{
			return IsNull(tableReportSale.DataColumn_12);
		}

		[DebuggerNonUserCode]
		public void method_25()
		{
			this[tableReportSale.DataColumn_12] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_26()
		{
			return IsNull(tableReportSale.DataColumn_13);
		}

		[DebuggerNonUserCode]
		public void method_27()
		{
			this[tableReportSale.DataColumn_13] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportBookingRow : DataRow
	{
		private ReportBookingDataTable tableReportBooking;

		[DebuggerNonUserCode]
		public string BookingNO
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBooking.BookingNOColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'BookingNO' in table 'ReportBooking' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBooking.BookingNOColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string Booking_NAME
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBooking.Booking_NAMEColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'Booking_NAME' in table 'ReportBooking' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBooking.Booking_NAMEColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string CIN
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBooking.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'CIN' in table 'ReportBooking' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBooking.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string COUT
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBooking.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'COUT' in table 'ReportBooking' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBooking.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string NIGHT
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBooking.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'NIGHT' in table 'ReportBooking' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBooking.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string ROOM_TYPE
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBooking.ROOM_TYPEColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ROOM_TYPE' in table 'ReportBooking' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBooking.ROOM_TYPEColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string ROOM_RATE
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBooking.ROOM_RATEColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ROOM_RATE' in table 'ReportBooking' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBooking.ROOM_RATEColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string ROOM_NUM
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBooking.ROOM_NUMColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ROOM_NUM' in table 'ReportBooking' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBooking.ROOM_NUMColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string ROOM_NIGHT
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBooking.ROOM_NIGHTColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ROOM_NIGHT' in table 'ReportBooking' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBooking.ROOM_NIGHTColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string ROOM_TOTAL
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBooking.ROOM_TOTALColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ROOM_TOTAL' in table 'ReportBooking' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBooking.ROOM_TOTALColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string NOTE
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBooking.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'NOTE' in table 'ReportBooking' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBooking.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string CONFIRM_BY
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBooking.CONFIRM_BYColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'CONFIRM_BY' in table 'ReportBooking' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBooking.CONFIRM_BYColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string BOOKING_DATE
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBooking.BOOKING_DATEColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'BOOKING_DATE' in table 'ReportBooking' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBooking.BOOKING_DATEColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string Datain
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBooking.DatainColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'Datain' in table 'ReportBooking' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBooking.DatainColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string Dataout
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBooking.DataoutColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'Dataout' in table 'ReportBooking' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBooking.DataoutColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string total
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBooking.totalColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'total' in table 'ReportBooking' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBooking.totalColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		internal ReportBookingRow(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReportBooking = (ReportBookingDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool IsBookingNONull()
		{
			return IsNull(tableReportBooking.BookingNOColumn);
		}

		[DebuggerNonUserCode]
		public void SetBookingNONull()
		{
			this[tableReportBooking.BookingNOColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsBooking_NAMENull()
		{
			return IsNull(tableReportBooking.Booking_NAMEColumn);
		}

		[DebuggerNonUserCode]
		public void SetBooking_NAMENull()
		{
			this[tableReportBooking.Booking_NAMEColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableReportBooking.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableReportBooking.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableReportBooking.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableReportBooking.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableReportBooking.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void SetNIGHTNull()
		{
			this[tableReportBooking.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsROOM_TYPENull()
		{
			return IsNull(tableReportBooking.ROOM_TYPEColumn);
		}

		[DebuggerNonUserCode]
		public void SetROOM_TYPENull()
		{
			this[tableReportBooking.ROOM_TYPEColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsROOM_RATENull()
		{
			return IsNull(tableReportBooking.ROOM_RATEColumn);
		}

		[DebuggerNonUserCode]
		public void SetROOM_RATENull()
		{
			this[tableReportBooking.ROOM_RATEColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsROOM_NUMNull()
		{
			return IsNull(tableReportBooking.ROOM_NUMColumn);
		}

		[DebuggerNonUserCode]
		public void SetROOM_NUMNull()
		{
			this[tableReportBooking.ROOM_NUMColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsROOM_NIGHTNull()
		{
			return IsNull(tableReportBooking.ROOM_NIGHTColumn);
		}

		[DebuggerNonUserCode]
		public void SetROOM_NIGHTNull()
		{
			this[tableReportBooking.ROOM_NIGHTColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsROOM_TOTALNull()
		{
			return IsNull(tableReportBooking.ROOM_TOTALColumn);
		}

		[DebuggerNonUserCode]
		public void SetROOM_TOTALNull()
		{
			this[tableReportBooking.ROOM_TOTALColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_5()
		{
			return IsNull(tableReportBooking.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_6()
		{
			this[tableReportBooking.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsCONFIRM_BYNull()
		{
			return IsNull(tableReportBooking.CONFIRM_BYColumn);
		}

		[DebuggerNonUserCode]
		public void SetCONFIRM_BYNull()
		{
			this[tableReportBooking.CONFIRM_BYColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsBOOKING_DATENull()
		{
			return IsNull(tableReportBooking.BOOKING_DATEColumn);
		}

		[DebuggerNonUserCode]
		public void SetBOOKING_DATENull()
		{
			this[tableReportBooking.BOOKING_DATEColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsDatainNull()
		{
			return IsNull(tableReportBooking.DatainColumn);
		}

		[DebuggerNonUserCode]
		public void SetDatainNull()
		{
			this[tableReportBooking.DatainColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsDataoutNull()
		{
			return IsNull(tableReportBooking.DataoutColumn);
		}

		[DebuggerNonUserCode]
		public void SetDataoutNull()
		{
			this[tableReportBooking.DataoutColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IstotalNull()
		{
			return IsNull(tableReportBooking.totalColumn);
		}

		[DebuggerNonUserCode]
		public void SettotalNull()
		{
			this[tableReportBooking.totalColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class GClass1 : DataRow
	{
		private ReportBookingINVDataTable tableReportBookingINV;

		[DebuggerNonUserCode]
		public string BookingNO
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.BookingNOColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'BookingNO' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.BookingNOColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string Booking_NAME
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.Booking_NAMEColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'Booking_NAME' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.Booking_NAMEColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string CIN
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'CIN' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string COUT
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'COUT' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string NIGHT
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'NIGHT' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string ROOM_TYPE
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.ROOM_TYPEColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ROOM_TYPE' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.ROOM_TYPEColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string ROOM_RATE
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.ROOM_RATEColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ROOM_RATE' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.ROOM_RATEColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string ROOM_NUM
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.ROOM_NUMColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ROOM_NUM' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.ROOM_NUMColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string ROOM_NIGHT
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.ROOM_NIGHTColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ROOM_NIGHT' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.ROOM_NIGHTColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string ROOM_TOTAL
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.ROOM_TOTALColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ROOM_TOTAL' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.ROOM_TOTALColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string NOTE
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'NOTE' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string CONFIRM_BY
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.CONFIRM_BYColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'CONFIRM_BY' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.CONFIRM_BYColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string BOOKING_DATE
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.BOOKING_DATEColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'BOOKING_DATE' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.BOOKING_DATEColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string Datain
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.DatainColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'Datain' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.DatainColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string Dataout
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.DataoutColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'Dataout' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.DataoutColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string total
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.totalColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'total' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.totalColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string INV_DATE
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.INV_DATEColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'INV_DATE' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.INV_DATEColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string INV_BY
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.INV_BYColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'INV_BY' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.INV_BYColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string INV_TITLE
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.INV_TITLEColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'INV_TITLE' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.INV_TITLEColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string INV_NAME
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.INV_NAMEColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'INV_NAME' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.INV_NAMEColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string INV_COMPANY
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.INV_COMPANYColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'INV_COMPANY' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.INV_COMPANYColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string INV_ADDRESS
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.INV_ADDRESSColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'INV_ADDRESS' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.INV_ADDRESSColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string INV_TEL
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.INV_TELColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'INV_TEL' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.INV_TELColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string INV_NIGHT
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.INV_NIGHTColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'INV_NIGHT' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.INV_NIGHTColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string INV_PAX
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.INV_PAXColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'INV_PAX' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.INV_PAXColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string INV_PAX_CHILD
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.INV_PAX_CHILDColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'INV_PAX_CHILD' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.INV_PAX_CHILDColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string INV_PAYMENT
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.INV_PAYMENTColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'INV_PAYMENT' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.INV_PAYMENTColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string INV_DUEDATE
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.INV_DUEDATEColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'INV_DUEDATE' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.INV_DUEDATEColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string INV_NOTE
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.INV_NOTEColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'INV_NOTE' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.INV_NOTEColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string INV_NO
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.INV_NOColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'INV_NO' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.INV_NOColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string pay
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.payColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'pay' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.payColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string balance
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.balanceColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'balance' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.balanceColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		public string inv_stay
		{
			get
			{
				try
				{
					return Conversions.ToString(this[tableReportBookingINV.inv_stayColumn]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'inv_stay' in table 'ReportBookingINV' is DBNull.", innerException);
				}
			}
			set
			{
				this[tableReportBookingINV.inv_stayColumn] = value;
			}
		}

		[DebuggerNonUserCode]
		internal GClass1(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			tableReportBookingINV = (ReportBookingINVDataTable)Table;
		}

		[DebuggerNonUserCode]
		public bool IsBookingNONull()
		{
			return IsNull(tableReportBookingINV.BookingNOColumn);
		}

		[DebuggerNonUserCode]
		public void SetBookingNONull()
		{
			this[tableReportBookingINV.BookingNOColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsBooking_NAMENull()
		{
			return IsNull(tableReportBookingINV.Booking_NAMEColumn);
		}

		[DebuggerNonUserCode]
		public void SetBooking_NAMENull()
		{
			this[tableReportBookingINV.Booking_NAMEColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(tableReportBookingINV.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[tableReportBookingINV.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(tableReportBookingINV.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[tableReportBookingINV.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(tableReportBookingINV.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void SetNIGHTNull()
		{
			this[tableReportBookingINV.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsROOM_TYPENull()
		{
			return IsNull(tableReportBookingINV.ROOM_TYPEColumn);
		}

		[DebuggerNonUserCode]
		public void SetROOM_TYPENull()
		{
			this[tableReportBookingINV.ROOM_TYPEColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsROOM_RATENull()
		{
			return IsNull(tableReportBookingINV.ROOM_RATEColumn);
		}

		[DebuggerNonUserCode]
		public void SetROOM_RATENull()
		{
			this[tableReportBookingINV.ROOM_RATEColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsROOM_NUMNull()
		{
			return IsNull(tableReportBookingINV.ROOM_NUMColumn);
		}

		[DebuggerNonUserCode]
		public void SetROOM_NUMNull()
		{
			this[tableReportBookingINV.ROOM_NUMColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsROOM_NIGHTNull()
		{
			return IsNull(tableReportBookingINV.ROOM_NIGHTColumn);
		}

		[DebuggerNonUserCode]
		public void SetROOM_NIGHTNull()
		{
			this[tableReportBookingINV.ROOM_NIGHTColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsROOM_TOTALNull()
		{
			return IsNull(tableReportBookingINV.ROOM_TOTALColumn);
		}

		[DebuggerNonUserCode]
		public void SetROOM_TOTALNull()
		{
			this[tableReportBookingINV.ROOM_TOTALColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_5()
		{
			return IsNull(tableReportBookingINV.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_6()
		{
			this[tableReportBookingINV.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsCONFIRM_BYNull()
		{
			return IsNull(tableReportBookingINV.CONFIRM_BYColumn);
		}

		[DebuggerNonUserCode]
		public void SetCONFIRM_BYNull()
		{
			this[tableReportBookingINV.CONFIRM_BYColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsBOOKING_DATENull()
		{
			return IsNull(tableReportBookingINV.BOOKING_DATEColumn);
		}

		[DebuggerNonUserCode]
		public void SetBOOKING_DATENull()
		{
			this[tableReportBookingINV.BOOKING_DATEColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsDatainNull()
		{
			return IsNull(tableReportBookingINV.DatainColumn);
		}

		[DebuggerNonUserCode]
		public void SetDatainNull()
		{
			this[tableReportBookingINV.DatainColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsDataoutNull()
		{
			return IsNull(tableReportBookingINV.DataoutColumn);
		}

		[DebuggerNonUserCode]
		public void SetDataoutNull()
		{
			this[tableReportBookingINV.DataoutColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IstotalNull()
		{
			return IsNull(tableReportBookingINV.totalColumn);
		}

		[DebuggerNonUserCode]
		public void SettotalNull()
		{
			this[tableReportBookingINV.totalColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsINV_DATENull()
		{
			return IsNull(tableReportBookingINV.INV_DATEColumn);
		}

		[DebuggerNonUserCode]
		public void SetINV_DATENull()
		{
			this[tableReportBookingINV.INV_DATEColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsINV_BYNull()
		{
			return IsNull(tableReportBookingINV.INV_BYColumn);
		}

		[DebuggerNonUserCode]
		public void SetINV_BYNull()
		{
			this[tableReportBookingINV.INV_BYColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsINV_TITLENull()
		{
			return IsNull(tableReportBookingINV.INV_TITLEColumn);
		}

		[DebuggerNonUserCode]
		public void SetINV_TITLENull()
		{
			this[tableReportBookingINV.INV_TITLEColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsINV_NAMENull()
		{
			return IsNull(tableReportBookingINV.INV_NAMEColumn);
		}

		[DebuggerNonUserCode]
		public void SetINV_NAMENull()
		{
			this[tableReportBookingINV.INV_NAMEColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsINV_COMPANYNull()
		{
			return IsNull(tableReportBookingINV.INV_COMPANYColumn);
		}

		[DebuggerNonUserCode]
		public void SetINV_COMPANYNull()
		{
			this[tableReportBookingINV.INV_COMPANYColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsINV_ADDRESSNull()
		{
			return IsNull(tableReportBookingINV.INV_ADDRESSColumn);
		}

		[DebuggerNonUserCode]
		public void SetINV_ADDRESSNull()
		{
			this[tableReportBookingINV.INV_ADDRESSColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsINV_TELNull()
		{
			return IsNull(tableReportBookingINV.INV_TELColumn);
		}

		[DebuggerNonUserCode]
		public void SetINV_TELNull()
		{
			this[tableReportBookingINV.INV_TELColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsINV_NIGHTNull()
		{
			return IsNull(tableReportBookingINV.INV_NIGHTColumn);
		}

		[DebuggerNonUserCode]
		public void SetINV_NIGHTNull()
		{
			this[tableReportBookingINV.INV_NIGHTColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsINV_PAXNull()
		{
			return IsNull(tableReportBookingINV.INV_PAXColumn);
		}

		[DebuggerNonUserCode]
		public void SetINV_PAXNull()
		{
			this[tableReportBookingINV.INV_PAXColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsINV_PAX_CHILDNull()
		{
			return IsNull(tableReportBookingINV.INV_PAX_CHILDColumn);
		}

		[DebuggerNonUserCode]
		public void SetINV_PAX_CHILDNull()
		{
			this[tableReportBookingINV.INV_PAX_CHILDColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsINV_PAYMENTNull()
		{
			return IsNull(tableReportBookingINV.INV_PAYMENTColumn);
		}

		[DebuggerNonUserCode]
		public void SetINV_PAYMENTNull()
		{
			this[tableReportBookingINV.INV_PAYMENTColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsINV_DUEDATENull()
		{
			return IsNull(tableReportBookingINV.INV_DUEDATEColumn);
		}

		[DebuggerNonUserCode]
		public void SetINV_DUEDATENull()
		{
			this[tableReportBookingINV.INV_DUEDATEColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsINV_NOTENull()
		{
			return IsNull(tableReportBookingINV.INV_NOTEColumn);
		}

		[DebuggerNonUserCode]
		public void SetINV_NOTENull()
		{
			this[tableReportBookingINV.INV_NOTEColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsINV_NONull()
		{
			return IsNull(tableReportBookingINV.INV_NOColumn);
		}

		[DebuggerNonUserCode]
		public void SetINV_NONull()
		{
			this[tableReportBookingINV.INV_NOColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IspayNull()
		{
			return IsNull(tableReportBookingINV.payColumn);
		}

		[DebuggerNonUserCode]
		public void SetpayNull()
		{
			this[tableReportBookingINV.payColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool IsbalanceNull()
		{
			return IsNull(tableReportBookingINV.balanceColumn);
		}

		[DebuggerNonUserCode]
		public void SetbalanceNull()
		{
			this[tableReportBookingINV.balanceColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool Isinv_stayNull()
		{
			return IsNull(tableReportBookingINV.inv_stayColumn);
		}

		[DebuggerNonUserCode]
		public void Setinv_stayNull()
		{
			this[tableReportBookingINV.inv_stayColumn] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class GClass2 : DataRow
	{
		private GClass0 gclass0_0;

		[DebuggerNonUserCode]
		public string String_0
		{
			get
			{
				try
				{
					return Conversions.ToString(this[gclass0_0.DataColumn_0]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ช\u0e37\u0e48อรร' in table 'Report_รร_4' is DBNull.", innerException);
				}
			}
			set
			{
				this[gclass0_0.DataColumn_0] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_1
		{
			get
			{
				try
				{
					return Conversions.ToString(this[gclass0_0.DataColumn_1]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48' in table 'Report_รร_4' is DBNull.", innerException);
				}
			}
			set
			{
				this[gclass0_0.DataColumn_1] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_2
		{
			get
			{
				try
				{
					return Conversions.ToString(this[gclass0_0.DataColumn_2]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ลำด\u0e31บ' in table 'Report_รร_4' is DBNull.", innerException);
				}
			}
			set
			{
				this[gclass0_0.DataColumn_2] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_3
		{
			get
			{
				try
				{
					return Conversions.ToString(this[gclass0_0.DataColumn_3]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48เข\u0e49าพ\u0e31ก' in table 'Report_รร_4' is DBNull.", innerException);
				}
			}
			set
			{
				this[gclass0_0.DataColumn_3] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_4
		{
			get
			{
				try
				{
					return Conversions.ToString(this[gclass0_0.DataColumn_4]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ห\u0e49องพ\u0e31กเลขท\u0e35\u0e48' in table 'Report_รร_4' is DBNull.", innerException);
				}
			}
			set
			{
				this[gclass0_0.DataColumn_4] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_5
		{
			get
			{
				try
				{
					return Conversions.ToString(this[gclass0_0.DataColumn_5]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ช\u0e37\u0e48อสก\u0e38ล' in table 'Report_รร_4' is DBNull.", innerException);
				}
			}
			set
			{
				this[gclass0_0.DataColumn_5] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_6
		{
			get
			{
				try
				{
					return Conversions.ToString(this[gclass0_0.DataColumn_6]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ส\u0e31ญชาต\u0e34' in table 'Report_รร_4' is DBNull.", innerException);
				}
			}
			set
			{
				this[gclass0_0.DataColumn_6] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_7
		{
			get
			{
				try
				{
					return Conversions.ToString(this[gclass0_0.DataColumn_7]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'เลขประจำต\u0e31ว' in table 'Report_รร_4' is DBNull.", innerException);
				}
			}
			set
			{
				this[gclass0_0.DataColumn_7] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_8
		{
			get
			{
				try
				{
					return Conversions.ToString(this[gclass0_0.DataColumn_8]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ท\u0e35\u0e48อย\u0e39\u0e48' in table 'Report_รร_4' is DBNull.", innerException);
				}
			}
			set
			{
				this[gclass0_0.DataColumn_8] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_9
		{
			get
			{
				try
				{
					return Conversions.ToString(this[gclass0_0.DataColumn_9]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'อาช\u0e35พ' in table 'Report_รร_4' is DBNull.", innerException);
				}
			}
			set
			{
				this[gclass0_0.DataColumn_9] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_10
		{
			get
			{
				try
				{
					return Conversions.ToString(this[gclass0_0.DataColumn_10]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'มาจาก' in table 'Report_รร_4' is DBNull.", innerException);
				}
			}
			set
			{
				this[gclass0_0.DataColumn_10] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_11
		{
			get
			{
				try
				{
					return Conversions.ToString(this[gclass0_0.DataColumn_11]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'จะไปท\u0e35\u0e48' in table 'Report_รร_4' is DBNull.", innerException);
				}
			}
			set
			{
				this[gclass0_0.DataColumn_11] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_12
		{
			get
			{
				try
				{
					return Conversions.ToString(this[gclass0_0.DataColumn_12]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'ว\u0e31นท\u0e35\u0e48ออก' in table 'Report_รร_4' is DBNull.", innerException);
				}
			}
			set
			{
				this[gclass0_0.DataColumn_12] = value;
			}
		}

		[DebuggerNonUserCode]
		public string String_13
		{
			get
			{
				try
				{
					return Conversions.ToString(this[gclass0_0.DataColumn_13]);
				}
				catch (InvalidCastException ex)
				{
					ProjectData.SetProjectError(ex);
					InvalidCastException innerException = ex;
					throw new StrongTypingException("The value for column 'หมายเหต\u0e38' in table 'Report_รร_4' is DBNull.", innerException);
				}
			}
			set
			{
				this[gclass0_0.DataColumn_13] = value;
			}
		}

		[DebuggerNonUserCode]
		internal GClass2(DataRowBuilder rb)
		{
			Class2.LH6iGfYz9j3MJ();
			base._002Ector(rb);
			gclass0_0 = (GClass0)Table;
		}

		[DebuggerNonUserCode]
		public bool method_0()
		{
			return IsNull(gclass0_0.DataColumn_0);
		}

		[DebuggerNonUserCode]
		public void method_1()
		{
			this[gclass0_0.DataColumn_0] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_2()
		{
			return IsNull(gclass0_0.DataColumn_1);
		}

		[DebuggerNonUserCode]
		public void method_3()
		{
			this[gclass0_0.DataColumn_1] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_4()
		{
			return IsNull(gclass0_0.DataColumn_2);
		}

		[DebuggerNonUserCode]
		public void method_5()
		{
			this[gclass0_0.DataColumn_2] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_6()
		{
			return IsNull(gclass0_0.DataColumn_3);
		}

		[DebuggerNonUserCode]
		public void method_7()
		{
			this[gclass0_0.DataColumn_3] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_8()
		{
			return IsNull(gclass0_0.DataColumn_4);
		}

		[DebuggerNonUserCode]
		public void method_9()
		{
			this[gclass0_0.DataColumn_4] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_10()
		{
			return IsNull(gclass0_0.DataColumn_5);
		}

		[DebuggerNonUserCode]
		public void method_11()
		{
			this[gclass0_0.DataColumn_5] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_12()
		{
			return IsNull(gclass0_0.DataColumn_6);
		}

		[DebuggerNonUserCode]
		public void method_13()
		{
			this[gclass0_0.DataColumn_6] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_14()
		{
			return IsNull(gclass0_0.DataColumn_7);
		}

		[DebuggerNonUserCode]
		public void method_15()
		{
			this[gclass0_0.DataColumn_7] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_16()
		{
			return IsNull(gclass0_0.DataColumn_8);
		}

		[DebuggerNonUserCode]
		public void method_17()
		{
			this[gclass0_0.DataColumn_8] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_18()
		{
			return IsNull(gclass0_0.DataColumn_9);
		}

		[DebuggerNonUserCode]
		public void method_19()
		{
			this[gclass0_0.DataColumn_9] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_20()
		{
			return IsNull(gclass0_0.DataColumn_10);
		}

		[DebuggerNonUserCode]
		public void method_21()
		{
			this[gclass0_0.DataColumn_10] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_22()
		{
			return IsNull(gclass0_0.DataColumn_11);
		}

		[DebuggerNonUserCode]
		public void method_23()
		{
			this[gclass0_0.DataColumn_11] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_24()
		{
			return IsNull(gclass0_0.DataColumn_12);
		}

		[DebuggerNonUserCode]
		public void method_25()
		{
			this[gclass0_0.DataColumn_12] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}

		[DebuggerNonUserCode]
		public bool method_26()
		{
			return IsNull(gclass0_0.DataColumn_13);
		}

		[DebuggerNonUserCode]
		public void method_27()
		{
			this[gclass0_0.DataColumn_13] = RuntimeHelpers.GetObjectValue(Convert.DBNull);
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportBillCashRowChangeEvent : EventArgs
	{
		private ReportBillCashRow eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public ReportBillCashRow Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public ReportBillCashRowChangeEvent(ReportBillCashRow row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class Bill_HRowChangeEvent : EventArgs
	{
		private Bill_HRow eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public Bill_HRow Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public Bill_HRowChangeEvent(Bill_HRow row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportRegRowChangeEvent : EventArgs
	{
		private ReportRegRow eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public ReportRegRow Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public ReportRegRowChangeEvent(ReportRegRow row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportDepRowChangeEvent : EventArgs
	{
		private ReportDepRow eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public ReportDepRow Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public ReportDepRowChangeEvent(ReportDepRow row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ConfigRowChangeEvent : EventArgs
	{
		private ConfigRow eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public ConfigRow Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public ConfigRowChangeEvent(ConfigRow row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportPicRowChangeEvent : EventArgs
	{
		private ReportPicRow eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public ReportPicRow Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public ReportPicRowChangeEvent(ReportPicRow row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportDaysRowChangeEvent : EventArgs
	{
		private ReportDaysRow eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public ReportDaysRow Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public ReportDaysRowChangeEvent(ReportDaysRow row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportCustINRowChangeEvent : EventArgs
	{
		private ReportCustINRow eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public ReportCustINRow Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public ReportCustINRowChangeEvent(ReportCustINRow row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportVatRowChangeEvent : EventArgs
	{
		private ReportVatRow eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public ReportVatRow Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public ReportVatRowChangeEvent(ReportVatRow row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportShiftRowChangeEvent : EventArgs
	{
		private ReportShiftRow eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public ReportShiftRow Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public ReportShiftRowChangeEvent(ReportShiftRow row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportCuponRowChangeEvent : EventArgs
	{
		private ReportCuponRow eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public ReportCuponRow Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public ReportCuponRowChangeEvent(ReportCuponRow row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class TableShiftCashRowChangeEvent : EventArgs
	{
		private TableShiftCashRow eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public TableShiftCashRow Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public TableShiftCashRowChangeEvent(TableShiftCashRow row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportBillCreditRowChangeEvent : EventArgs
	{
		private ReportBillCreditRow eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public ReportBillCreditRow Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public ReportBillCreditRowChangeEvent(ReportBillCreditRow row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class Report_Room_allRowChangeEvent : EventArgs
	{
		private Report_Room_allRow eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public Report_Room_allRow Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public Report_Room_allRowChangeEvent(Report_Room_allRow row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class Report_Debt_INVRowChangeEvent : EventArgs
	{
		private Report_Debt_INVRow eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public Report_Debt_INVRow Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public Report_Debt_INVRowChangeEvent(Report_Debt_INVRow row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportFolio1RowChangeEvent : EventArgs
	{
		private ReportFolio1Row eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public ReportFolio1Row Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public ReportFolio1RowChangeEvent(ReportFolio1Row row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportFolio1_2RowChangeEvent : EventArgs
	{
		private ReportFolio1_2Row eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public ReportFolio1_2Row Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public ReportFolio1_2RowChangeEvent(ReportFolio1_2Row row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportFolio2RowChangeEvent : EventArgs
	{
		private ReportFolio2Row eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public ReportFolio2Row Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public ReportFolio2RowChangeEvent(ReportFolio2Row row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportSaleRowChangeEvent : EventArgs
	{
		private ReportSaleRow eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public ReportSaleRow Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public ReportSaleRowChangeEvent(ReportSaleRow row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportBookingRowChangeEvent : EventArgs
	{
		private ReportBookingRow eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public ReportBookingRow Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public ReportBookingRowChangeEvent(ReportBookingRow row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class ReportBookingINVRowChangeEvent : EventArgs
	{
		private GClass1 eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public GClass1 Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public ReportBookingINVRowChangeEvent(GClass1 row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	[GeneratedCode("System.Data.Design.TypedDataSetGenerator", "2.0.0.0")]
	public class GEventArgs0 : EventArgs
	{
		private GClass2 eventRow;

		private DataRowAction eventAction;

		[DebuggerNonUserCode]
		public GClass2 Row => eventRow;

		[DebuggerNonUserCode]
		public DataRowAction Action => eventAction;

		[DebuggerNonUserCode]
		public GEventArgs0(GClass2 row, DataRowAction action)
		{
			Class2.LH6iGfYz9j3MJ();
			// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
			eventRow = row;
			eventAction = action;
		}
	}

	private static List<WeakReference> __ENCList;

	private ReportBillCashDataTable tableReportBillCash;

	private Bill_HDataTable tableBill_H;

	private ReportRegDataTable tableReportReg;

	private ReportDepDataTable tableReportDep;

	private ConfigDataTable tableConfig;

	private ReportPicDataTable tableReportPic;

	private ReportDaysDataTable tableReportDays;

	private ReportCustINDataTable tableReportCustIN;

	private ReportVatDataTable tableReportVat;

	private ReportShiftDataTable tableReportShift;

	private ReportCuponDataTable tableReportCupon;

	private TableShiftCashDataTable tableTableShiftCash;

	private ReportBillCreditDataTable tableReportBillCredit;

	private Report_Room_allDataTable tableReport_Room_all;

	private Report_Debt_INVDataTable tableReport_Debt_INV;

	private ReportFolio1DataTable tableReportFolio1;

	private ReportFolio1_2DataTable tableReportFolio1_2;

	private ReportFolio2DataTable tableReportFolio2;

	private ReportSaleDataTable tableReportSale;

	private ReportBookingDataTable tableReportBooking;

	private ReportBookingINVDataTable tableReportBookingINV;

	private GClass0 gclass0_0;

	private SchemaSerializationMode _schemaSerializationMode;

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	[Browsable(false)]
	[DebuggerNonUserCode]
	public ReportBillCashDataTable ReportBillCash => tableReportBillCash;

	[Browsable(false)]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	[DebuggerNonUserCode]
	public Bill_HDataTable Bill_H => tableBill_H;

	[Browsable(false)]
	[DebuggerNonUserCode]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	public ReportRegDataTable ReportReg => tableReportReg;

	[Browsable(false)]
	[DebuggerNonUserCode]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	public ReportDepDataTable ReportDep => tableReportDep;

	[Browsable(false)]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	[DebuggerNonUserCode]
	public ConfigDataTable Config => tableConfig;

	[DebuggerNonUserCode]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	[Browsable(false)]
	public ReportPicDataTable ReportPic => tableReportPic;

	[DebuggerNonUserCode]
	[Browsable(false)]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	public ReportDaysDataTable ReportDays => tableReportDays;

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	[Browsable(false)]
	[DebuggerNonUserCode]
	public ReportCustINDataTable ReportCustIN => tableReportCustIN;

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	[DebuggerNonUserCode]
	[Browsable(false)]
	public ReportVatDataTable ReportVat => tableReportVat;

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	[DebuggerNonUserCode]
	[Browsable(false)]
	public ReportShiftDataTable ReportShift => tableReportShift;

	[DebuggerNonUserCode]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	[Browsable(false)]
	public ReportCuponDataTable ReportCupon => tableReportCupon;

	[Browsable(false)]
	[DebuggerNonUserCode]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	public TableShiftCashDataTable TableShiftCash => tableTableShiftCash;

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	[DebuggerNonUserCode]
	[Browsable(false)]
	public ReportBillCreditDataTable ReportBillCredit => tableReportBillCredit;

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	[Browsable(false)]
	[DebuggerNonUserCode]
	public Report_Room_allDataTable Report_Room_all => tableReport_Room_all;

	[DebuggerNonUserCode]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	[Browsable(false)]
	public Report_Debt_INVDataTable Report_Debt_INV => tableReport_Debt_INV;

	[Browsable(false)]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	[DebuggerNonUserCode]
	public ReportFolio1DataTable ReportFolio1 => tableReportFolio1;

	[Browsable(false)]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	[DebuggerNonUserCode]
	public ReportFolio1_2DataTable ReportFolio1_2 => tableReportFolio1_2;

	[Browsable(false)]
	[DebuggerNonUserCode]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	public ReportFolio2DataTable ReportFolio2 => tableReportFolio2;

	[Browsable(false)]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	[DebuggerNonUserCode]
	public ReportSaleDataTable ReportSale => tableReportSale;

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	[Browsable(false)]
	[DebuggerNonUserCode]
	public ReportBookingDataTable ReportBooking => tableReportBooking;

	[Browsable(false)]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	[DebuggerNonUserCode]
	public ReportBookingINVDataTable ReportBookingINV => tableReportBookingINV;

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Content)]
	[DebuggerNonUserCode]
	[Browsable(false)]
	public GClass0 GClass0_0 => gclass0_0;

	[DebuggerNonUserCode]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Visible)]
	[Browsable(true)]
	public override SchemaSerializationMode SchemaSerializationMode
	{
		get
		{
			return _schemaSerializationMode;
		}
		set
		{
			_schemaSerializationMode = value;
		}
	}

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
	[DebuggerNonUserCode]
	public new DataTableCollection Tables => base.Tables;

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
	[DebuggerNonUserCode]
	public new DataRelationCollection Relations => base.Relations;

	[DebuggerNonUserCode]
	static Datalocal()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public Datalocal()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		_schemaSerializationMode = SchemaSerializationMode.IncludeSchema;
		BeginInit();
		InitClass();
		CollectionChangeEventHandler value = SchemaChanged;
		base.Tables.CollectionChanged += value;
		base.Relations.CollectionChanged += value;
		EndInit();
	}

	[DebuggerNonUserCode]
	protected Datalocal(SerializationInfo info, StreamingContext context)
	{
		Class2.LH6iGfYz9j3MJ();
		base._002Ector(info, context, ConstructSchema: false);
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		_schemaSerializationMode = SchemaSerializationMode.IncludeSchema;
		if (IsBinarySerialized(info, context))
		{
			InitVars(initTable: false);
			CollectionChangeEventHandler value = SchemaChanged;
			Tables.CollectionChanged += value;
			Relations.CollectionChanged += value;
			return;
		}
		string s = Conversions.ToString(info.GetValue("XmlSchema", typeof(string)));
		if (DetermineSchemaSerializationMode(info, context) == SchemaSerializationMode.IncludeSchema)
		{
			DataSet dataSet = new DataSet();
			dataSet.ReadXmlSchema(new XmlTextReader(new StringReader(s)));
			if (dataSet.Tables["ReportBillCash"] != null)
			{
				base.Tables.Add(new ReportBillCashDataTable(dataSet.Tables["ReportBillCash"]));
			}
			if (dataSet.Tables["Bill_H"] != null)
			{
				base.Tables.Add(new Bill_HDataTable(dataSet.Tables["Bill_H"]));
			}
			if (dataSet.Tables["ReportReg"] != null)
			{
				base.Tables.Add(new ReportRegDataTable(dataSet.Tables["ReportReg"]));
			}
			if (dataSet.Tables["ReportDep"] != null)
			{
				base.Tables.Add(new ReportDepDataTable(dataSet.Tables["ReportDep"]));
			}
			if (dataSet.Tables["Config"] != null)
			{
				base.Tables.Add(new ConfigDataTable(dataSet.Tables["Config"]));
			}
			if (dataSet.Tables["ReportPic"] != null)
			{
				base.Tables.Add(new ReportPicDataTable(dataSet.Tables["ReportPic"]));
			}
			if (dataSet.Tables["ReportDays"] != null)
			{
				base.Tables.Add(new ReportDaysDataTable(dataSet.Tables["ReportDays"]));
			}
			if (dataSet.Tables["ReportCustIN"] != null)
			{
				base.Tables.Add(new ReportCustINDataTable(dataSet.Tables["ReportCustIN"]));
			}
			if (dataSet.Tables["ReportVat"] != null)
			{
				base.Tables.Add(new ReportVatDataTable(dataSet.Tables["ReportVat"]));
			}
			if (dataSet.Tables["ReportShift"] != null)
			{
				base.Tables.Add(new ReportShiftDataTable(dataSet.Tables["ReportShift"]));
			}
			if (dataSet.Tables["ReportCupon"] != null)
			{
				base.Tables.Add(new ReportCuponDataTable(dataSet.Tables["ReportCupon"]));
			}
			if (dataSet.Tables["TableShiftCash"] != null)
			{
				base.Tables.Add(new TableShiftCashDataTable(dataSet.Tables["TableShiftCash"]));
			}
			if (dataSet.Tables["ReportBillCredit"] != null)
			{
				base.Tables.Add(new ReportBillCreditDataTable(dataSet.Tables["ReportBillCredit"]));
			}
			if (dataSet.Tables["Report_Room_all"] != null)
			{
				base.Tables.Add(new Report_Room_allDataTable(dataSet.Tables["Report_Room_all"]));
			}
			if (dataSet.Tables["Report_Debt_INV"] != null)
			{
				base.Tables.Add(new Report_Debt_INVDataTable(dataSet.Tables["Report_Debt_INV"]));
			}
			if (dataSet.Tables["ReportFolio1"] != null)
			{
				base.Tables.Add(new ReportFolio1DataTable(dataSet.Tables["ReportFolio1"]));
			}
			if (dataSet.Tables["ReportFolio1_2"] != null)
			{
				base.Tables.Add(new ReportFolio1_2DataTable(dataSet.Tables["ReportFolio1_2"]));
			}
			if (dataSet.Tables["ReportFolio2"] != null)
			{
				base.Tables.Add(new ReportFolio2DataTable(dataSet.Tables["ReportFolio2"]));
			}
			if (dataSet.Tables["ReportSale"] != null)
			{
				base.Tables.Add(new ReportSaleDataTable(dataSet.Tables["ReportSale"]));
			}
			if (dataSet.Tables["ReportBooking"] != null)
			{
				base.Tables.Add(new ReportBookingDataTable(dataSet.Tables["ReportBooking"]));
			}
			if (dataSet.Tables["ReportBookingINV"] != null)
			{
				base.Tables.Add(new ReportBookingINVDataTable(dataSet.Tables["ReportBookingINV"]));
			}
			if (dataSet.Tables["Report_รร_4"] != null)
			{
				base.Tables.Add(new GClass0(dataSet.Tables["Report_รร_4"]));
			}
			DataSetName = dataSet.DataSetName;
			Prefix = dataSet.Prefix;
			Namespace = dataSet.Namespace;
			Locale = dataSet.Locale;
			CaseSensitive = dataSet.CaseSensitive;
			EnforceConstraints = dataSet.EnforceConstraints;
			Merge(dataSet, preserveChanges: false, MissingSchemaAction.Add);
			InitVars();
		}
		else
		{
			ReadXmlSchema(new XmlTextReader(new StringReader(s)));
		}
		GetSerializationData(info, context);
		CollectionChangeEventHandler value2 = SchemaChanged;
		base.Tables.CollectionChanged += value2;
		Relations.CollectionChanged += value2;
	}

	[DebuggerNonUserCode]
	protected override void InitializeDerivedDataSet()
	{
		BeginInit();
		InitClass();
		EndInit();
	}

	[DebuggerNonUserCode]
	public override DataSet Clone()
	{
		Datalocal datalocal = (Datalocal)base.Clone();
		datalocal.InitVars();
		datalocal.SchemaSerializationMode = SchemaSerializationMode;
		return datalocal;
	}

	[DebuggerNonUserCode]
	protected override bool ShouldSerializeTables()
	{
		return false;
	}

	[DebuggerNonUserCode]
	protected override bool ShouldSerializeRelations()
	{
		return false;
	}

	[DebuggerNonUserCode]
	protected override void ReadXmlSerializable(XmlReader reader)
	{
		if (DetermineSchemaSerializationMode(reader) == SchemaSerializationMode.IncludeSchema)
		{
			Reset();
			DataSet dataSet = new DataSet();
			dataSet.ReadXml(reader);
			if (dataSet.Tables["ReportBillCash"] != null)
			{
				base.Tables.Add(new ReportBillCashDataTable(dataSet.Tables["ReportBillCash"]));
			}
			if (dataSet.Tables["Bill_H"] != null)
			{
				base.Tables.Add(new Bill_HDataTable(dataSet.Tables["Bill_H"]));
			}
			if (dataSet.Tables["ReportReg"] != null)
			{
				base.Tables.Add(new ReportRegDataTable(dataSet.Tables["ReportReg"]));
			}
			if (dataSet.Tables["ReportDep"] != null)
			{
				base.Tables.Add(new ReportDepDataTable(dataSet.Tables["ReportDep"]));
			}
			if (dataSet.Tables["Config"] != null)
			{
				base.Tables.Add(new ConfigDataTable(dataSet.Tables["Config"]));
			}
			if (dataSet.Tables["ReportPic"] != null)
			{
				base.Tables.Add(new ReportPicDataTable(dataSet.Tables["ReportPic"]));
			}
			if (dataSet.Tables["ReportDays"] != null)
			{
				base.Tables.Add(new ReportDaysDataTable(dataSet.Tables["ReportDays"]));
			}
			if (dataSet.Tables["ReportCustIN"] != null)
			{
				base.Tables.Add(new ReportCustINDataTable(dataSet.Tables["ReportCustIN"]));
			}
			if (dataSet.Tables["ReportVat"] != null)
			{
				base.Tables.Add(new ReportVatDataTable(dataSet.Tables["ReportVat"]));
			}
			if (dataSet.Tables["ReportShift"] != null)
			{
				base.Tables.Add(new ReportShiftDataTable(dataSet.Tables["ReportShift"]));
			}
			if (dataSet.Tables["ReportCupon"] != null)
			{
				base.Tables.Add(new ReportCuponDataTable(dataSet.Tables["ReportCupon"]));
			}
			if (dataSet.Tables["TableShiftCash"] != null)
			{
				base.Tables.Add(new TableShiftCashDataTable(dataSet.Tables["TableShiftCash"]));
			}
			if (dataSet.Tables["ReportBillCredit"] != null)
			{
				base.Tables.Add(new ReportBillCreditDataTable(dataSet.Tables["ReportBillCredit"]));
			}
			if (dataSet.Tables["Report_Room_all"] != null)
			{
				base.Tables.Add(new Report_Room_allDataTable(dataSet.Tables["Report_Room_all"]));
			}
			if (dataSet.Tables["Report_Debt_INV"] != null)
			{
				base.Tables.Add(new Report_Debt_INVDataTable(dataSet.Tables["Report_Debt_INV"]));
			}
			if (dataSet.Tables["ReportFolio1"] != null)
			{
				base.Tables.Add(new ReportFolio1DataTable(dataSet.Tables["ReportFolio1"]));
			}
			if (dataSet.Tables["ReportFolio1_2"] != null)
			{
				base.Tables.Add(new ReportFolio1_2DataTable(dataSet.Tables["ReportFolio1_2"]));
			}
			if (dataSet.Tables["ReportFolio2"] != null)
			{
				base.Tables.Add(new ReportFolio2DataTable(dataSet.Tables["ReportFolio2"]));
			}
			if (dataSet.Tables["ReportSale"] != null)
			{
				base.Tables.Add(new ReportSaleDataTable(dataSet.Tables["ReportSale"]));
			}
			if (dataSet.Tables["ReportBooking"] != null)
			{
				base.Tables.Add(new ReportBookingDataTable(dataSet.Tables["ReportBooking"]));
			}
			if (dataSet.Tables["ReportBookingINV"] != null)
			{
				base.Tables.Add(new ReportBookingINVDataTable(dataSet.Tables["ReportBookingINV"]));
			}
			if (dataSet.Tables["Report_รร_4"] != null)
			{
				base.Tables.Add(new GClass0(dataSet.Tables["Report_รร_4"]));
			}
			DataSetName = dataSet.DataSetName;
			Prefix = dataSet.Prefix;
			Namespace = dataSet.Namespace;
			Locale = dataSet.Locale;
			CaseSensitive = dataSet.CaseSensitive;
			EnforceConstraints = dataSet.EnforceConstraints;
			Merge(dataSet, preserveChanges: false, MissingSchemaAction.Add);
			InitVars();
		}
		else
		{
			ReadXml(reader);
			InitVars();
		}
	}

	[DebuggerNonUserCode]
	protected override XmlSchema GetSchemaSerializable()
	{
		MemoryStream memoryStream = new MemoryStream();
		WriteXmlSchema(new XmlTextWriter(memoryStream, null));
		memoryStream.Position = 0L;
		return XmlSchema.Read(new XmlTextReader(memoryStream), null);
	}

	[DebuggerNonUserCode]
	internal void InitVars()
	{
		InitVars(initTable: true);
	}

	[DebuggerNonUserCode]
	internal void InitVars(bool initTable)
	{
		tableReportBillCash = (ReportBillCashDataTable)base.Tables["ReportBillCash"];
		if (initTable && tableReportBillCash != null)
		{
			tableReportBillCash.InitVars();
		}
		tableBill_H = (Bill_HDataTable)base.Tables["Bill_H"];
		if (initTable && tableBill_H != null)
		{
			tableBill_H.InitVars();
		}
		tableReportReg = (ReportRegDataTable)base.Tables["ReportReg"];
		if (initTable && tableReportReg != null)
		{
			tableReportReg.InitVars();
		}
		tableReportDep = (ReportDepDataTable)base.Tables["ReportDep"];
		if (initTable && tableReportDep != null)
		{
			tableReportDep.InitVars();
		}
		tableConfig = (ConfigDataTable)base.Tables["Config"];
		if (initTable && tableConfig != null)
		{
			tableConfig.InitVars();
		}
		tableReportPic = (ReportPicDataTable)base.Tables["ReportPic"];
		if (initTable && tableReportPic != null)
		{
			tableReportPic.InitVars();
		}
		tableReportDays = (ReportDaysDataTable)base.Tables["ReportDays"];
		if (initTable && tableReportDays != null)
		{
			tableReportDays.InitVars();
		}
		tableReportCustIN = (ReportCustINDataTable)base.Tables["ReportCustIN"];
		if (initTable && tableReportCustIN != null)
		{
			tableReportCustIN.InitVars();
		}
		tableReportVat = (ReportVatDataTable)base.Tables["ReportVat"];
		if (initTable && tableReportVat != null)
		{
			tableReportVat.InitVars();
		}
		tableReportShift = (ReportShiftDataTable)base.Tables["ReportShift"];
		if (initTable && tableReportShift != null)
		{
			tableReportShift.InitVars();
		}
		tableReportCupon = (ReportCuponDataTable)base.Tables["ReportCupon"];
		if (initTable && tableReportCupon != null)
		{
			tableReportCupon.InitVars();
		}
		tableTableShiftCash = (TableShiftCashDataTable)base.Tables["TableShiftCash"];
		if (initTable && tableTableShiftCash != null)
		{
			tableTableShiftCash.InitVars();
		}
		tableReportBillCredit = (ReportBillCreditDataTable)base.Tables["ReportBillCredit"];
		if (initTable && tableReportBillCredit != null)
		{
			tableReportBillCredit.InitVars();
		}
		tableReport_Room_all = (Report_Room_allDataTable)base.Tables["Report_Room_all"];
		if (initTable && tableReport_Room_all != null)
		{
			tableReport_Room_all.InitVars();
		}
		tableReport_Debt_INV = (Report_Debt_INVDataTable)base.Tables["Report_Debt_INV"];
		if (initTable && tableReport_Debt_INV != null)
		{
			tableReport_Debt_INV.InitVars();
		}
		tableReportFolio1 = (ReportFolio1DataTable)base.Tables["ReportFolio1"];
		if (initTable && tableReportFolio1 != null)
		{
			tableReportFolio1.InitVars();
		}
		tableReportFolio1_2 = (ReportFolio1_2DataTable)base.Tables["ReportFolio1_2"];
		if (initTable && tableReportFolio1_2 != null)
		{
			tableReportFolio1_2.InitVars();
		}
		tableReportFolio2 = (ReportFolio2DataTable)base.Tables["ReportFolio2"];
		if (initTable && tableReportFolio2 != null)
		{
			tableReportFolio2.InitVars();
		}
		tableReportSale = (ReportSaleDataTable)base.Tables["ReportSale"];
		if (initTable && tableReportSale != null)
		{
			tableReportSale.InitVars();
		}
		tableReportBooking = (ReportBookingDataTable)base.Tables["ReportBooking"];
		if (initTable && tableReportBooking != null)
		{
			tableReportBooking.InitVars();
		}
		tableReportBookingINV = (ReportBookingINVDataTable)base.Tables["ReportBookingINV"];
		if (initTable && tableReportBookingINV != null)
		{
			tableReportBookingINV.InitVars();
		}
		gclass0_0 = (GClass0)base.Tables["Report_รร_4"];
		if (initTable && gclass0_0 != null)
		{
			gclass0_0.InitVars();
		}
	}

	[DebuggerNonUserCode]
	private void InitClass()
	{
		DataSetName = "Datalocal";
		Prefix = "";
		Namespace = "http://tempuri.org/Datalocal.xsd";
		EnforceConstraints = true;
		SchemaSerializationMode = SchemaSerializationMode.IncludeSchema;
		tableReportBillCash = new ReportBillCashDataTable();
		base.Tables.Add(tableReportBillCash);
		tableBill_H = new Bill_HDataTable();
		base.Tables.Add(tableBill_H);
		tableReportReg = new ReportRegDataTable();
		base.Tables.Add(tableReportReg);
		tableReportDep = new ReportDepDataTable();
		base.Tables.Add(tableReportDep);
		tableConfig = new ConfigDataTable();
		base.Tables.Add(tableConfig);
		tableReportPic = new ReportPicDataTable();
		base.Tables.Add(tableReportPic);
		tableReportDays = new ReportDaysDataTable();
		base.Tables.Add(tableReportDays);
		tableReportCustIN = new ReportCustINDataTable();
		base.Tables.Add(tableReportCustIN);
		tableReportVat = new ReportVatDataTable();
		base.Tables.Add(tableReportVat);
		tableReportShift = new ReportShiftDataTable();
		base.Tables.Add(tableReportShift);
		tableReportCupon = new ReportCuponDataTable();
		base.Tables.Add(tableReportCupon);
		tableTableShiftCash = new TableShiftCashDataTable();
		base.Tables.Add(tableTableShiftCash);
		tableReportBillCredit = new ReportBillCreditDataTable();
		base.Tables.Add(tableReportBillCredit);
		tableReport_Room_all = new Report_Room_allDataTable();
		base.Tables.Add(tableReport_Room_all);
		tableReport_Debt_INV = new Report_Debt_INVDataTable();
		base.Tables.Add(tableReport_Debt_INV);
		tableReportFolio1 = new ReportFolio1DataTable();
		base.Tables.Add(tableReportFolio1);
		tableReportFolio1_2 = new ReportFolio1_2DataTable();
		base.Tables.Add(tableReportFolio1_2);
		tableReportFolio2 = new ReportFolio2DataTable();
		base.Tables.Add(tableReportFolio2);
		tableReportSale = new ReportSaleDataTable();
		base.Tables.Add(tableReportSale);
		tableReportBooking = new ReportBookingDataTable();
		base.Tables.Add(tableReportBooking);
		tableReportBookingINV = new ReportBookingINVDataTable();
		base.Tables.Add(tableReportBookingINV);
		gclass0_0 = new GClass0();
		base.Tables.Add(gclass0_0);
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReportBillCash()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeBill_H()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReportReg()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReportDep()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeConfig()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReportPic()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReportDays()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReportCustIN()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReportVat()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReportShift()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReportCupon()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeTableShiftCash()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReportBillCredit()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReport_Room_all()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReport_Debt_INV()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReportFolio1()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReportFolio1_2()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReportFolio2()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReportSale()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReportBooking()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool ShouldSerializeReportBookingINV()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private bool method_0()
	{
		return false;
	}

	[DebuggerNonUserCode]
	private void SchemaChanged(object sender, CollectionChangeEventArgs e)
	{
		if (e.Action == CollectionChangeAction.Remove)
		{
			InitVars();
		}
	}

	[DebuggerNonUserCode]
	public static XmlSchemaComplexType GetTypedDataSetSchema(XmlSchemaSet xs)
	{
		Datalocal datalocal = new Datalocal();
		XmlSchemaComplexType xmlSchemaComplexType = new XmlSchemaComplexType();
		XmlSchemaSequence xmlSchemaSequence = new XmlSchemaSequence();
		XmlSchemaAny xmlSchemaAny = new XmlSchemaAny();
		xmlSchemaAny.Namespace = datalocal.Namespace;
		xmlSchemaSequence.Items.Add(xmlSchemaAny);
		xmlSchemaComplexType.Particle = xmlSchemaSequence;
		XmlSchema schemaSerializable = datalocal.GetSchemaSerializable();
		if (xs.Contains(schemaSerializable.TargetNamespace))
		{
			MemoryStream memoryStream = new MemoryStream();
			MemoryStream memoryStream2 = new MemoryStream();
			try
			{
				XmlSchema xmlSchema = null;
				schemaSerializable.Write(memoryStream);
				IEnumerator enumerator = xs.Schemas(schemaSerializable.TargetNamespace).GetEnumerator();
				while (enumerator.MoveNext())
				{
					xmlSchema = (XmlSchema)enumerator.Current;
					memoryStream2.SetLength(0L);
					xmlSchema.Write(memoryStream2);
					if (memoryStream.Length == memoryStream2.Length)
					{
						memoryStream.Position = 0L;
						memoryStream2.Position = 0L;
						while ((memoryStream.Position != memoryStream.Length && memoryStream.ReadByte() == memoryStream2.ReadByte()) ? true : false)
						{
						}
						if (memoryStream.Position == memoryStream.Length)
						{
							return xmlSchemaComplexType;
						}
					}
				}
			}
			finally
			{
				memoryStream?.Close();
				memoryStream2?.Close();
			}
		}
		xs.Add(schemaSerializable);
		return xmlSchemaComplexType;
	}
}
