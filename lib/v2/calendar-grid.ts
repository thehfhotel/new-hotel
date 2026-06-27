/**
 * Pure helpers for the availability calendar grid (rooms × dates) — task #53.
 *
 * Fed by GET /api/calendar (bookings + check-ins, both sites). All date math is
 * done on the literal "YYYY-MM-DD" prefix of the stored date strings, NEVER via
 * `Date` parsing — the canonical API serializes naive Thai-local datetimes
 * without a timezone, so parsing them in a UTC+7 browser shifts a midnight value
 * to the previous day (same reasoning as `lib/v2/status.ts::isSameStoredDay`).
 */

/** Cell occupancy for one (room, day) pair. */
export type CellState = 'free' | 'booked' | 'occupied'

/** Minimal shape of a calendar booking / check-in entry from /api/calendar. */
export interface CalendarEntry {
  roomNo: string | null
  customerName?: string | null
  checkIn: string | null
  checkOut: string | null
}

/** "YYYY-MM-DD" slice of a stored date string, or null when unparseable. */
export function dayKey(value: string | null | undefined): string | null {
  if (!value) return null
  const s = String(value).slice(0, 10)
  return /^\d{4}-\d{2}-\d{2}$/.test(s) ? s : null
}

/** Format a Date as a local "YYYY-MM-DD" key (no timezone shift). */
export function toDayKey(date: Date): string {
  const y = date.getFullYear()
  const m = String(date.getMonth() + 1).padStart(2, '0')
  const d = String(date.getDate()).padStart(2, '0')
  return `${y}-${m}-${d}`
}

/** A contiguous run of `count` day keys starting at `start` (inclusive). */
export function dayRange(start: Date, count: number): string[] {
  const out: string[] = []
  for (let i = 0; i < count; i++) {
    const d = new Date(start)
    d.setDate(start.getDate() + i)
    out.push(toDayKey(d))
  }
  return out
}

/**
 * True when `day` falls inside the stay's occupied nights — `[checkIn, checkOut)`
 * (the checkout day frees the room). A missing checkout occupies only the
 * check-in day. Lexicographic compare is valid for zero-padded ISO dates.
 */
export function coversDay(entry: CalendarEntry, day: string): boolean {
  const ci = dayKey(entry.checkIn)
  if (!ci) return false
  const co = dayKey(entry.checkOut)
  if (!co) return ci === day
  return ci <= day && day < co
}

export interface GridInputs {
  /** Room numbers forming the grid rows. */
  rooms: string[]
  /** Day keys ("YYYY-MM-DD") forming the grid columns. */
  days: string[]
  bookings: CalendarEntry[]
  checkins: CalendarEntry[]
}

/**
 * Build a `${roomNo}|${day}` → CellState map. Precedence: occupied (an active
 * check-in) always wins over booked (a reservation), which wins over free.
 * Entries whose room is not in `rooms` are ignored.
 */
export function buildGrid({ rooms, days, bookings, checkins }: GridInputs): Map<string, CellState> {
  const grid = new Map<string, CellState>()
  const roomSet = new Set(rooms)

  const mark = (entries: CalendarEntry[], state: CellState) => {
    for (const e of entries) {
      const room = e.roomNo
      if (!room || !roomSet.has(room)) continue
      for (const day of days) {
        if (!coversDay(e, day)) continue
        const key = `${room}|${day}`
        // occupied overrides anything; booked only fills a free/empty cell.
        if (state === 'occupied' || !grid.has(key)) grid.set(key, state)
      }
    }
  }

  mark(bookings, 'booked')
  mark(checkins, 'occupied')
  return grid
}

/** Look up a cell's state, defaulting to 'free'. */
export function cellState(
  grid: Map<string, CellState>,
  roomNo: string,
  day: string,
): CellState {
  return grid.get(`${roomNo}|${day}`) ?? 'free'
}
