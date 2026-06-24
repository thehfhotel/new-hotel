const THAI_MONTHS_FULL = [
  'มกราคม', 'กุมภาพันธ์', 'มีนาคม', 'เมษายน', 'พฤษภาคม', 'มิถุนายน',
  'กรกฎาคม', 'สิงหาคม', 'กันยายน', 'ตุลาคม', 'พฤศจิกายน', 'ธันวาคม',
]

const THAI_MONTHS_SHORT = [
  'ม.ค.', 'ก.พ.', 'มี.ค.', 'เม.ย.', 'พ.ค.', 'มิ.ย.',
  'ก.ค.', 'ส.ค.', 'ก.ย.', 'ต.ค.', 'พ.ย.', 'ธ.ค.',
]

export { THAI_MONTHS_FULL, THAI_MONTHS_SHORT }

/** Format currency in Thai Baht. decimals=0 for display, decimals=2 for invoices/billing */
export function formatCurrency(amount: number | null, decimals: 0 | 2 = 0): string {
  if (amount === null || amount === undefined) return '-'
  if (decimals === 0) {
    return new Intl.NumberFormat('th-TH', {
      style: 'currency',
      currency: 'THB',
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(amount)
  }
  return new Intl.NumberFormat('th-TH', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(amount)
}

/** Convert Gregorian year to Buddhist Era year number */
export function toBuddhistYear(date: Date): number {
  return date.getFullYear() + 543
}

/** Format date in Thai with Buddhist Era. 'short'=DD/MM/YYYY, 'full'=D FULLMONTH YEAR, 'abbreviated'=D SHORTMONTH YEAR */
export function formatBuddhistDate(dateInput: string | Date | null, format: 'short' | 'full' | 'abbreviated' = 'short'): string {
  if (!dateInput) return '-'
  const date = typeof dateInput === 'string' ? new Date(dateInput) : dateInput
  const day = date.getDate()
  const dayPadded = String(day).padStart(2, '0')
  const month = date.getMonth()
  const monthPadded = String(month + 1).padStart(2, '0')
  const year = toBuddhistYear(date)

  switch (format) {
    case 'full':
      return `${day} ${THAI_MONTHS_FULL[month]} ${year}`
    case 'abbreviated':
      return `${day} ${THAI_MONTHS_SHORT[month]} ${year}`
    case 'short':
    default:
      return `${dayPadded}/${monthPadded}/${year}`
  }
}

/** Format date for API calls (YYYY-MM-DD) */
export function formatDateForApi(date: Date): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

/** Format Thai date string. Backward-compatible wrapper around formatBuddhistDate. */
export function formatThaiDate(dateStr: string | null, useFullMonth = false): string {
  return formatBuddhistDate(dateStr, useFullMonth ? 'full' : 'abbreviated')
}

/** Format Thai date short (DD/MM/YYYY Buddhist era) */
export function formatThaiDateShort(dateStr: string | null): string {
  return formatBuddhistDate(dateStr, 'short')
}

/**
 * Stored-datetime display helpers (the "Thai time stored as UTC" rule).
 *
 * SQL Server stores datetimes in Thai local time (GMT+7) but the driver returns
 * them with a `Z` suffix, so JS would misinterpret them as UTC. We therefore
 * format with `timeZone: 'UTC'` to render the stored value AS-IS. Using
 * 'Asia/Bangkok' would WRONGLY add 7 hours. See CLAUDE.md timezone-handling note.
 *
 * Each helper reproduces a distinct option-shape that was previously copy-pasted
 * inline across the frontend. Output is byte-identical to the original inline call.
 *
 * NOTE: distinct from `formatThaiDate`/`formatThaiDateShort` above, which render
 * Buddhist-Era dates WITHOUT the UTC rule — do not conflate them.
 */
type StoredDateInput = string | number | Date | null | undefined

function toStoredDate(value: StoredDateInput): Date | null {
  if (value === null || value === undefined) return null
  return new Date(value)
}

/** Stored date as DD/MM/YYYY (Gregorian, 2-digit day/month, numeric year). */
export function formatStoredDate(value: StoredDateInput): string {
  const date = toStoredDate(value)
  if (!date) return '-'
  return date.toLocaleDateString('th-TH', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    timeZone: 'UTC',
  })
}

/** Stored date as "D MON" (day numeric, short month, no year). */
export function formatStoredDayMonth(value: StoredDateInput): string {
  const date = toStoredDate(value)
  if (!date) return '-'
  return date.toLocaleDateString('th-TH', {
    day: 'numeric',
    month: 'short',
    timeZone: 'UTC',
  })
}

/** Stored date as "D MON YYYY" (day numeric, short month, numeric year). */
export function formatStoredDayMonthYear(value: StoredDateInput): string {
  const date = toStoredDate(value)
  if (!date) return '-'
  return date.toLocaleDateString('th-TH', {
    day: 'numeric',
    month: 'short',
    year: 'numeric',
    timeZone: 'UTC',
  })
}

/** Stored datetime as "D MON YYYY, HH:MM" (numeric day/year, short month, 2-digit time). */
export function formatStoredDateTime(value: StoredDateInput): string {
  const date = toStoredDate(value)
  if (!date) return '-'
  return date.toLocaleString('th-TH', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    timeZone: 'UTC',
  })
}

/** Stored datetime as "D MON YYYY HH:MM" (date-string form: numeric day/year, short month, 2-digit time). */
export function formatStoredDayMonthYearTime(value: StoredDateInput): string {
  const date = toStoredDate(value)
  if (!date) return '-'
  return date.toLocaleDateString('th-TH', {
    day: 'numeric',
    month: 'short',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    timeZone: 'UTC',
  })
}

/** Calculate nights between two date strings */
export function calculateNights(checkIn: string, checkOut: string): number {
  if (!checkIn || !checkOut) return 0
  const start = new Date(checkIn)
  const end = new Date(checkOut)
  const diff = end.getTime() - start.getTime()
  return Math.max(0, Math.ceil(diff / (1000 * 60 * 60 * 24)))
}
