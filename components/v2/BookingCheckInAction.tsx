/**
 * The check-in control on a reservation — Task B7a.
 *
 * One component for all three answers, so the reservations list row, the
 * reservation detail header and the room board's action sheet cannot drift:
 *
 *   ready      → "เช็คอิน" (loud when the guest arrives today)
 *   checked-in → "เช็คอินแล้ว", linking to the folio when we know which stay
 *   no-room    → a disabled control that says WHY, instead of vanishing
 *
 * `hidden` renders nothing (cancelled / completed / no-show / unpaid hold).
 * The rule itself lives in `lib/v2/checkin-from-booking`.
 */

'use client'

import Link from 'next/link'
import { LogIn, Loader2, CheckCircle2, BedDouble } from 'lucide-react'
import { folioHref, type BookingCheckInState } from '@/lib/v2/checkin-from-booking'

export const CHECK_IN_LABEL = 'เช็คอิน'
export const CHECKED_IN_LABEL = 'เช็คอินแล้ว'
export const NO_ROOM_LABEL = 'ยังไม่ได้กำหนดห้อง'

export default function BookingCheckInAction({
  state,
  onCheckIn,
  busy = false,
  disabled = false,
  size = 'sm',
  className = '',
}: {
  state: BookingCheckInState
  onCheckIn: () => void
  /** A resolve/open is in flight — the button spins and refuses a second click. */
  busy?: boolean
  /** Read-only branch (HF Ville until writes are enabled) — hide the action. */
  disabled?: boolean
  size?: 'sm' | 'md'
  className?: string
}) {
  const sizeCls = size === 'sm' ? 'v2-btn-sm' : ''

  if (state.kind === 'hidden') return null

  if (state.kind === 'checked-in') {
    // Reception's next question after "it says checked in" is always "then
    // where is the bill" — answer it in the same control rather than making
    // them go find the stay by room number.
    const label = (
      <>
        <CheckCircle2 size={15} /> {CHECKED_IN_LABEL}
      </>
    )
    if (state.checkInId == null) {
      return (
        <span
          data-testid="booking-checked-in"
          className={`v2-btn v2-btn-ghost ${sizeCls} pointer-events-none opacity-70 ${className}`}
        >
          {label}
        </span>
      )
    }
    return (
      <Link
        data-testid="booking-checked-in"
        href={folioHref(state.checkInId)}
        onClick={(e) => e.stopPropagation()}
        className={`v2-btn v2-btn-ghost ${sizeCls} ${className}`}
      >
        {label}
      </Link>
    )
  }

  if (state.kind === 'no-room') {
    // Disabled-but-present, not absent: "there is no button" reads as a bug,
    // "assign a room first" reads as an instruction.
    return (
      <button
        type="button"
        data-testid="booking-checkin-no-room"
        disabled
        title={NO_ROOM_LABEL}
        onClick={(e) => e.stopPropagation()}
        className={`v2-btn v2-btn-ghost ${sizeCls} opacity-60 ${className}`}
      >
        <BedDouble size={15} /> {NO_ROOM_LABEL}
      </button>
    )
  }

  if (disabled) return null

  return (
    <button
      type="button"
      data-testid="booking-checkin-action"
      data-arrival-today={state.arrivalToday ? 'true' : 'false'}
      disabled={busy}
      onClick={(e) => {
        e.stopPropagation()
        onCheckIn()
      }}
      className={`v2-btn ${state.arrivalToday ? 'v2-btn-primary' : 'v2-btn-ghost'} ${sizeCls} ${className}`}
    >
      {busy ? <Loader2 size={15} className="animate-spin" /> : <LogIn size={15} />}
      {CHECK_IN_LABEL}
    </button>
  )
}
