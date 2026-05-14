/**
 * Invoice and Receipt types for hotel billing documents
 */

export interface InvoiceRoom {
  roomNumber: string;
  roomType: string;
  ratePerNight: number;
  nights: number;
  subtotal: number;
}

export interface InvoiceData {
  invoiceNumber: string;
  /** Legacy check-in number (e.g. "CH26-005258"). Used by the
   *  legacy_mirror panels to fetch coupons/minibar/room-changes
   *  attached to this stay. Optional — pages that don't have it
   *  set just won't render the panels. */
  cinNo?: string;
  checkInId: number;
  guestName: string;
  guestIdCard: string;
  guestContact: string;
  /** Track G3 / T4 HIGH-7: corporate buyer's tax-id
   *  (`ht_customers.cust_work_tax`). Required on every Thai VAT invoice
   *  issued to a juristic person; absent for individual walk-ins. */
  guestTaxId?: string;
  checkInDate: string;
  checkOutDate: string;
  rooms: InvoiceRoom[];
  subtotal: number;
  discount: number;
  /** Track G3: VAT-inclusive split — subtotal before VAT (banker's rounding). */
  beforeVat?: number;
  vatAmount: number;
  /** Track G3: VAT percentage applied (e.g. 7). Read from
   *  `ht_settings.vat_percent` server-side; falls back to 7% on lookup
   *  failure. Optional on the type for older callers that haven't been
   *  updated yet. */
  vatPercent?: number;
  grandTotal: number;
  paymentMethod: string;
  paidAmount: number;
  createdAt: string;
}

export interface HotelInfo {
  name: string;
  address: string;
  phone: string;
  taxId: string;
  logo?: string;
}

export interface ReceiptData extends InvoiceData {
  receiptNumber: string;
  cashierName?: string;
}
