/**
 * Unit tests for the availability calendar grid helpers (task #53).
 * Pure functions — no DOM/network. Verifies the [checkIn, checkOut) night model,
 * occupied-over-booked precedence, timezone-safe day keys, and range building.
 */

import {
  buildGrid,
  cellState,
  coversDay,
  dayKey,
  dayRange,
  toDayKey,
  type CalendarEntry,
} from '@/lib/v2/calendar-grid'

describe('dayKey / toDayKey', () => {
  it('slices the YYYY-MM-DD prefix from a stored datetime', () => {
    expect(dayKey('2026-06-27T14:30:00')).toBe('2026-06-27')
    expect(dayKey('2026-06-27')).toBe('2026-06-27')
  })

  it('returns null for empty or malformed input', () => {
    expect(dayKey(null)).toBeNull()
    expect(dayKey('')).toBeNull()
    expect(dayKey('not-a-date')).toBeNull()
  })

  it('formats a local Date without timezone shift', () => {
    // Construct from local components so the key matches regardless of TZ.
    expect(toDayKey(new Date(2026, 5, 27))).toBe('2026-06-27')
  })
})

describe('dayRange', () => {
  it('produces a contiguous run including the start day', () => {
    const range = dayRange(new Date(2026, 5, 27), 3)
    expect(range).toEqual(['2026-06-27', '2026-06-28', '2026-06-29'])
  })

  it('crosses a month boundary correctly', () => {
    const range = dayRange(new Date(2026, 5, 30), 3)
    expect(range).toEqual(['2026-06-30', '2026-07-01', '2026-07-02'])
  })
})

describe('coversDay', () => {
  const entry: CalendarEntry = { roomNo: '101', checkIn: '2026-06-27', checkOut: '2026-06-29' }

  it('occupies the nights in [checkIn, checkOut)', () => {
    expect(coversDay(entry, '2026-06-27')).toBe(true)
    expect(coversDay(entry, '2026-06-28')).toBe(true)
  })

  it('frees the checkout day and days outside the window', () => {
    expect(coversDay(entry, '2026-06-29')).toBe(false) // checkout day
    expect(coversDay(entry, '2026-06-26')).toBe(false) // before
  })

  it('occupies only the check-in day when checkout is missing', () => {
    const open: CalendarEntry = { roomNo: '101', checkIn: '2026-06-27', checkOut: null }
    expect(coversDay(open, '2026-06-27')).toBe(true)
    expect(coversDay(open, '2026-06-28')).toBe(false)
  })

  it('never covers when check-in is unparseable', () => {
    expect(coversDay({ roomNo: '101', checkIn: null, checkOut: '2026-06-29' }, '2026-06-27')).toBe(false)
  })
})

describe('buildGrid + cellState', () => {
  const rooms = ['101', '102', '103']
  const days = dayRange(new Date(2026, 5, 27), 3) // 27,28,29

  const bookings: CalendarEntry[] = [
    { roomNo: '101', checkIn: '2026-06-27', checkOut: '2026-06-29' },
    { roomNo: '999', checkIn: '2026-06-27', checkOut: '2026-06-28' }, // not in roster → ignored
  ]
  const checkins: CalendarEntry[] = [
    { roomNo: '102', checkIn: '2026-06-28', checkOut: '2026-06-29' },
    { roomNo: '101', checkIn: '2026-06-28', checkOut: '2026-06-29' }, // occupied overrides booked
  ]

  const grid = buildGrid({ rooms, days, bookings, checkins })

  it('marks booked nights', () => {
    expect(cellState(grid, '101', '2026-06-27')).toBe('booked')
  })

  it('lets an active check-in override a booking on the same night', () => {
    expect(cellState(grid, '101', '2026-06-28')).toBe('occupied')
  })

  it('marks standalone check-ins as occupied', () => {
    expect(cellState(grid, '102', '2026-06-28')).toBe('occupied')
  })

  it('defaults empty cells to free', () => {
    expect(cellState(grid, '103', '2026-06-27')).toBe('free')
    expect(cellState(grid, '102', '2026-06-27')).toBe('free')
  })

  it('ignores entries whose room is not in the roster', () => {
    expect(cellState(grid, '999', '2026-06-27')).toBe('free')
  })
})
