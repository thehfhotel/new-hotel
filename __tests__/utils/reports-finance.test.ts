/**
 * Unit tests for the financial-report helpers (task #55).
 * Pure functions — no DOM/network.
 */

import {
  formatDateForApi,
  sumIncome,
  vatInclusiveSplit,
} from '@/lib/reports-finance'
import type { RevenueDataPoint } from '@/types/reports'

describe('formatDateForApi', () => {
  it('formats local date components without timezone shift', () => {
    // Construct from local components so the key is TZ-independent.
    const d = new Date(2026, 0, 5) // 2026-01-05 local
    expect(formatDateForApi(d)).toBe('2026-01-05')
  })

  it('zero-pads month and day', () => {
    expect(formatDateForApi(new Date(2026, 8, 9))).toBe('2026-09-09')
  })
})

describe('sumIncome', () => {
  const rows: RevenueDataPoint[] = [
    { period: '2026-05', revenue: 1000, bookings: 4 },
    { period: '2026-06', revenue: 250.5, bookings: 1 },
  ]

  it('accumulates revenue and bookings', () => {
    expect(sumIncome(rows)).toEqual({ revenue: 1250.5, bookings: 5 })
  })

  it('returns zeros for an empty series', () => {
    expect(sumIncome([])).toEqual({ revenue: 0, bookings: 0 })
  })

  it('ignores non-finite values defensively', () => {
    const dirty = [
      { period: 'x', revenue: Number.NaN, bookings: 2 },
      { period: 'y', revenue: 100, bookings: Number.POSITIVE_INFINITY },
    ] as RevenueDataPoint[]
    expect(sumIncome(dirty)).toEqual({ revenue: 100, bookings: 2 })
  })
})

describe('vatInclusiveSplit', () => {
  it('matches the captured legacy receipt math at 7%', () => {
    // Total 801.00 -> BeforeVat 748.60, Vat 52.40 (mirrors the backend).
    expect(vatInclusiveSplit(801, 7)).toEqual({ beforeVat: 748.6, vat: 52.4 })
  })

  it('is a no-op at 0% (net == gross, vat 0)', () => {
    expect(vatInclusiveSplit(1000, 0)).toEqual({ beforeVat: 1000, vat: 0 })
  })

  it('parts sum back to the original total', () => {
    const { beforeVat, vat } = vatInclusiveSplit(107, 7)
    expect(beforeVat).toBe(100)
    expect(vat).toBe(7)
    expect(beforeVat + vat).toBe(107)
  })
})
