/** Maps backend status strings to the v2 visual language (label + tint class +
 *  dot class). Tints/dots are defined in app/v2/v2.css (.s-* and .d-*). */

export type V2Tone = 'ok' | 'occ' | 'arr' | 'dep' | 'fix' | 'mut'

export interface V2StatusView {
  label: string
  tone: V2Tone
  /** background+text tint class, e.g. "s-ok" */
  cls: string
  /** dot color class, e.g. "d-ok" */
  dot: string
}

function view(label: string, tone: V2Tone): V2StatusView {
  return { label, tone, cls: `s-${tone}`, dot: `d-${tone}` }
}

/** Room status from /api/rooms (`status` + isClean/isMaintenance). */
export function roomStatusView(
  status: string,
  opts?: { isClean?: boolean; isMaintenance?: boolean },
): V2StatusView {
  if (opts?.isMaintenance || status === 'maintenance') return view('ซ่อมบำรุง', 'fix')
  switch (status) {
    case 'occupied':
      return view('มีผู้เข้าพัก', 'occ')
    case 'booked':
      return view('จองแล้ว', 'arr')
    case 'checkout_pending':
      return view('รอเช็คเอาท์', 'dep')
    case 'available':
      // A vacant room that still needs housekeeping shows as "รอทำความสะอาด"
      // (matches iHOTEL marking the room dirty right after checkout). This was
      // previously suppressed because the room_clean flag was GLOBALLY INVERTED
      // in the sync (legacy Room_Clean='yes'=dirty mapped to canonical
      // true=clean), so every free room looked dirty. That inversion is fixed +
      // backfilled (2026-06-30), so isClean is now accurate and safe to surface.
      // isClean === false → needs cleaning; true/undefined → clean (ว่าง).
      return opts?.isClean === false ? view('รอทำความสะอาด', 'dep') : view('ว่าง', 'ok')
    default:
      return view(status || 'ไม่ทราบ', 'mut')
  }
}

/** Booking status from /api/bookings. */
export function bookingStatusView(status: string): V2StatusView {
  switch (status) {
    case 'pending':
      return view('รอยืนยัน', 'arr')
    case 'confirmed':
      return view('ยืนยันแล้ว', 'ok')
    case 'checkedin':
      return view('เช็คอินแล้ว', 'occ')
    case 'completed':
      return view('เสร็จสิ้น', 'mut')
    case 'cancelled':
      return view('ยกเลิก', 'fix')
    case 'noshow':
      return view('ไม่มาตามนัด', 'fix')
    default:
      return view(status || '-', 'mut')
  }
}

/** Check-in / folio status from /api/checkins. */
export function checkinStatusView(status: string): V2StatusView {
  switch (status) {
    case 'active':
    case 'checkedin':
      return view('กำลังเข้าพัก', 'occ')
    case 'checkedout':
    case 'completed':
      return view('เช็คเอาท์แล้ว', 'mut')
    case 'cancelled':
      return view('ยกเลิก', 'fix')
    default:
      return view(status || '-', 'mut')
  }
}

/** True if a stored date string falls on the same calendar day as `ref`
 *  (default: today).
 *
 *  The canonical `/api/*` endpoints serialize naive datetimes WITHOUT a
 *  timezone suffix (e.g. "2026-06-26T00:00:00"), while others append "Z". Both
 *  carry Thai-local clock values (see CLAUDE.md timezone rule), so the only safe
 *  comparison is on the literal Y-M-D prefix of the string vs. today's Thai
 *  date — NOT Date parsing, which would shift a no-Z midnight to the previous
 *  day in a UTC+7 browser and under-count arrivals/departures by one day. */
export function isSameStoredDay(value: string | null | undefined, ref = new Date()): boolean {
  if (!value) return false
  const datePart = String(value).slice(0, 10) // "YYYY-MM-DD"
  if (!/^\d{4}-\d{2}-\d{2}$/.test(datePart)) {
    // Fallback for non-ISO inputs: compare via local date components.
    const d = new Date(value)
    if (Number.isNaN(d.getTime())) return false
    return (
      d.getFullYear() === ref.getFullYear() &&
      d.getMonth() === ref.getMonth() &&
      d.getDate() === ref.getDate()
    )
  }
  const refPart = `${ref.getFullYear()}-${String(ref.getMonth() + 1).padStart(2, '0')}-${String(
    ref.getDate(),
  ).padStart(2, '0')}`
  return datePart === refPart
}
