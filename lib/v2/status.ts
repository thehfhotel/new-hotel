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

/** Room status from /api/new/rooms (`status` + isClean/isMaintenance). */
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
      return opts?.isClean === false ? view('รอทำความสะอาด', 'fix') : view('ว่าง', 'ok')
    default:
      return view(status || 'ไม่ทราบ', 'mut')
  }
}

/** Booking status from /api/new/bookings. */
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

/** Check-in / folio status from /api/new/checkins. */
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

/** True if an ISO/stored date string falls on the same calendar day as `ref`
 *  (default: today), comparing in the stored-as-UTC convention used app-wide. */
export function isSameStoredDay(value: string | null | undefined, ref = new Date()): boolean {
  if (!value) return false
  const d = new Date(value)
  if (Number.isNaN(d.getTime())) return false
  return (
    d.getUTCFullYear() === ref.getFullYear() &&
    d.getUTCMonth() === ref.getMonth() &&
    d.getUTCDate() === ref.getDate()
  )
}
