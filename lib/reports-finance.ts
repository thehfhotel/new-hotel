/**
 * Pure helpers for the financial reports surface (task #55).
 *
 * No DOM / network — kept separate from the page component so the money math
 * and formatting are unit-testable (see __tests__/utils/reports-finance.test.ts).
 */

import type { RevenueDataPoint } from '@/types/reports'

/** Format a baht amount with two-decimal precision (statement / receipt style). */
export function formatBaht(amount: number): string {
  return new Intl.NumberFormat('th-TH', {
    style: 'currency',
    currency: 'THB',
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(Number.isFinite(amount) ? amount : 0)
}

/** Format a `Date` as the `YYYY-MM-DD` the report API expects (local, no TZ shift). */
export function formatDateForApi(date: Date): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

/** Grand totals for the printable income table. */
export interface IncomeTotals {
  revenue: number
  bookings: number
}

/** Sum a revenue series into `{ revenue, bookings }` grand totals. */
export function sumIncome(rows: RevenueDataPoint[]): IncomeTotals {
  return rows.reduce<IncomeTotals>(
    (acc, r) => ({
      revenue: acc.revenue + (Number.isFinite(r.revenue) ? r.revenue : 0),
      bookings: acc.bookings + (Number.isFinite(r.bookings) ? r.bookings : 0),
    }),
    { revenue: 0, bookings: 0 },
  )
}

/**
 * Mirror of the backend's inclusive VAT split (writeback::format::
 * vat_inclusive_split) for client-side previews: `before = total / (1 +
 * vat%/100)`, `vat = total - before`, both rounded to 2dp. At 0% it is a no-op.
 * The backend remains the source of truth for the rendered report; this exists
 * only so the UI can sanity-display a derived figure without an extra round trip.
 */
export function vatInclusiveSplit(total: number, vatPercent: number): { beforeVat: number; vat: number } {
  const safeTotal = Number.isFinite(total) ? total : 0
  const divisor = 1 + (Number.isFinite(vatPercent) ? vatPercent : 0) / 100
  const beforeVat = Math.round((safeTotal / divisor) * 100) / 100
  const vat = Math.round((safeTotal - beforeVat) * 100) / 100
  return { beforeVat, vat }
}
