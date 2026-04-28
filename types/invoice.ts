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
  checkInDate: string;
  checkOutDate: string;
  rooms: InvoiceRoom[];
  subtotal: number;
  discount: number;
  vatAmount: number;
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
