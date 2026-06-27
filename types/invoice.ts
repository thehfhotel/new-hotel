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

/** Task #44: one POS / product / other-charge line on a tax invoice
 *  (ใบกำกับภาษี). Sourced from the folio's `ht_pos_sales` (posted rows). */
export interface InvoiceProductLine {
  name: string;
  unit?: string;
  qty: number;
  unitPrice: number;
  total: number;
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
  /** Task #44: POS / product / other-charge lines from the folio. Absent
   *  or empty for a room-only stay — the template hides the section. */
  products?: InvoiceProductLine[];
  subtotal: number;
  /** Task #44: sum of `products[].total`. Drives the "products subtotal"
   *  line; absent on room-only callers. */
  productsSubtotal?: number;
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

/* ------------------------------------------------------------------ *
 * Task #45 — standalone (walk-up / roomless) sale receipt.
 *
 * `InvoiceData` / `ReceiptData` both REQUIRE a `checkInId` + `rooms[]`,
 * which a walk-up POS sale does not have. These types are deliberately
 * decoupled from any folio so the `StandaloneReceiptTemplate` can render a
 * pure product-line receipt (mirror of legacy `HT_Receipt_H`/`Ds`).
 * ------------------------------------------------------------------ */

/** One product line on a standalone receipt. */
export interface StandaloneReceiptLine {
  /** Product business code (legacy `Pro_no` / `S_Product_no`). Optional
   *  for ad-hoc service lines. */
  productNo?: string;
  name: string;
  unit?: string;
  qty: number;
  unitPrice: number;
  /** Per-line discount in baht (legacy `S_PriceDiscount`). */
  discount?: number;
  total: number;
}

/** A standalone sale receipt — NO check-in / rooms. */
export interface StandaloneReceiptData {
  /** Human-facing receipt number (legacy `Receipt_no`, `B{yyMM}-{4digit}`).
   *  Absent until the legacy writeback allocates it; the template then
   *  shows the canonical id instead. */
  receiptNumber?: string;
  /** Canonical `ht_pos_receipts.receipt_id`. */
  receiptId: number;
  /** Buyer block — all optional for an anonymous walk-up. */
  customerName?: string;
  customerAddress?: string;
  customerTel?: string;
  /** Buyer tax / customer ID (Thai VAT invoice). */
  customerTaxId?: string;
  lines: StandaloneReceiptLine[];
  /** Sum of `lines[].qty × unitPrice` before discounts. */
  subtotal: number;
  /** Total discount (header + per-line) in baht. */
  discount: number;
  /** Subtotal before VAT (VAT-inclusive split). */
  beforeVat?: number;
  vatAmount: number;
  /** VAT percentage applied (e.g. 0 or 7). */
  vatPercent?: number;
  /** VAT-inclusive grand total. */
  grandTotal: number;
  paymentMethod: string;
  paidAmount: number;
  cashierName?: string;
  /** ISO timestamp the sale was rung up. */
  createdAt: string;
}
