import type { InvoiceData } from '@/types/invoice'

/**
 * Shape of one invoice as returned by `GET /api/checkins/:id/invoice`
 * (`hotel-backend/src/routes/new_invoice.rs::Invoice`). Shared by the
 * classic folio page (`app/billing/[id]`) and the v2 tax-invoice page
 * (`app/v2/invoice/[id]`) so the API→`InvoiceData` mapping lives in one
 * place and the two surfaces can never drift.
 */
export interface InvoiceApiInvoice {
  checkinId: number
  cinNo: string
  bookingId: number | null
  bookingNo: string | null
  /** Track G3 / T4 HIGH-7: PG-only invoice number (legacy
   *  HT_INVOICE.INV_NO alignment deferred — see new_invoice.rs module doc). */
  invNo: string
  guest: {
    id: number
    firstName: string
    lastName: string | null
    fullName: string
    email: string | null
    phone: string | null
    address: string | null
    idCard: string | null
    passport: string | null
    /** Track G3: corporate buyer tax-id from cust_work_tax. */
    taxId: string | null
  }
  room: {
    roomId: number
    roomNo: string
    roomType: string | null
    floor: number | null
  }
  checkInTime: string | null
  checkOutTime: string | null
  expectedCheckout: string | null
  adults: number
  children: number
  rates: {
    ratePerNight: number
    nights: number
    subtotal: number
  }
  /** Task #44: itemised POS / other charges from the folio. */
  products?: Array<{
    name: string
    unit: string | null
    qty: number
    unitPrice: number
    total: number
  }>
  /** Task #44: room subtotal (== rates.subtotal). */
  roomTotal?: number
  /** Task #44: sum of posted POS line totals. */
  productsTotal?: number
  /** Stored cin_total_amount (room before checkout; room+product after). */
  totalAmount: number
  /** Task #44: reconciling document total = roomTotal + productsTotal. */
  grandTotal?: number
  /** Track G3: VAT-inclusive split (banker's rounding server-side). */
  beforeVat: number
  vatAmount: number
  vatPer: number
  paymentStatus: string | null
  notes: string | null
  createdAt: string | null
}

export interface InvoiceApiResponse {
  success: boolean
  invoice?: InvoiceApiInvoice
  error?: string
}

/**
 * Map the backend invoice payload to the `InvoiceData` shape consumed by
 * `components/documents/InvoiceTemplate`. Single source of truth for the
 * transform so the classic folio and the v2 tax-invoice render identically.
 *
 * The document total prefers the backend's reconciling `grandTotal`
 * (room + products) and falls back to `totalAmount`/room subtotal for
 * older API responses that pre-date task #44.
 */
export function mapInvoiceResponse(invoice: InvoiceApiInvoice): InvoiceData {
  const products = (invoice.products ?? []).map((p) => ({
    name: p.name,
    unit: p.unit || undefined,
    qty: p.qty,
    unitPrice: p.unitPrice,
    total: p.total,
  }))
  const productsSubtotal =
    invoice.productsTotal ?? products.reduce((sum, p) => sum + p.total, 0)
  const grandTotal =
    invoice.grandTotal ?? invoice.totalAmount ?? invoice.rates.subtotal

  return {
    // Track G3: prefer the dedicated invoice number (INVyyMM-NNNNNN) when
    // the backend supplied one; fall back to the legacy cin_no for older
    // API responses that pre-date the G3 field.
    invoiceNumber: invoice.invNo || invoice.cinNo,
    cinNo: invoice.cinNo,
    checkInId: invoice.checkinId,
    guestName: invoice.guest.fullName,
    guestIdCard: invoice.guest.idCard || invoice.guest.passport || '',
    guestContact: invoice.guest.phone || invoice.guest.email || '',
    // Track G3: corporate buyer tax-id. `undefined` for individual walk-ins
    // so the InvoiceTemplate header hides the row.
    guestTaxId: invoice.guest.taxId || undefined,
    checkInDate: invoice.checkInTime || '',
    checkOutDate: invoice.checkOutTime || invoice.expectedCheckout || '',
    rooms: [
      {
        roomNumber: invoice.room.roomNo,
        roomType: invoice.room.roomType || 'Standard',
        ratePerNight: invoice.rates.ratePerNight,
        nights: invoice.rates.nights,
        subtotal: invoice.rates.subtotal,
      },
    ],
    products,
    subtotal: invoice.roomTotal ?? invoice.rates.subtotal,
    productsSubtotal,
    discount: 0,
    // Track G3: VAT-inclusive split — derived server-side via banker's
    // rounding so the printed receipt matches the legacy .NET app's
    // Math.Round behaviour.
    beforeVat: invoice.beforeVat,
    vatAmount: invoice.vatAmount ?? 0,
    vatPercent: invoice.vatPer,
    grandTotal,
    paymentMethod: invoice.paymentStatus || 'pending',
    paidAmount: invoice.totalAmount || 0,
    createdAt: invoice.createdAt || new Date().toISOString(),
  }
}
