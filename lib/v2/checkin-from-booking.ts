/**
 * "Can this reservation be checked in here, and has it been already?" — Task B7a.
 *
 * The backend has accepted `CreateCheckInRequest.booking_id` since the spike
 * (`routes/new_checkins.rs` → `CheckInService::check_in_to_booking`, which
 * writes `ht_checkins.cin_book_id` and flips `ht_bookings.book_status` to
 * `'checkedin'`), but every frontend POST /api/checkins call site was room-only
 * and never sent it. A stay created in our app therefore had no booking link,
 * so the B7 app-deposit signposts — which resolve through the originating
 * booking — had nothing to resolve and stayed silent on exactly the guest they
 * exist for.
 *
 * This module is the ONE rule shared by every surface that offers the action
 * (the reservations list row, the reservation detail header, and the room
 * board's action sheet for a `booked` room), so reception can never see a
 * booking that offers check-in on one screen and not on another.
 *
 * DELIBERATELY NOT A DATE GATE. iHOTEL lets the desk check a guest in early,
 * and ADR 0003 forbids removing a capability reception has today, so an
 * out-of-window booking still offers the action — `arrivalToday` only changes
 * how loudly the button is drawn. The commit itself is still behind the
 * check-in modal's confirm screen.
 */

import { isSameStoredDay } from '@/lib/v2/status'

/** `ht_bookings.book_status` once `check_in_to_booking` has run. */
export const CHECKED_IN_STATUS = 'checkedin'

/** One assigned room of a booking (`NewBookingDetail.rooms[]`). */
export interface BookingRoomRef {
  roomId: number
  roomNo: string | null
}

/**
 * The booking fields the rule reads. Shaped so BOTH booking DTOs fit:
 * the LIST row knows only `roomCount`, the DETAIL response carries `rooms`.
 */
export interface BookingCheckInInput {
  /** `ht_bookings.book_status`. */
  status: string
  /** `book_checkin` — arrival date, used only for the `arrivalToday` hint. */
  checkIn?: string | null
  /** `book_deposit_amount` in baht — what makes a `pending` booking eligible. */
  depositAmount?: number | null
  /** Assigned rooms (detail response). `undefined` ⇒ fall back to `roomCount`. */
  rooms?: BookingRoomRef[] | null
  /** Assigned-room count (list response). */
  roomCount?: number | null
}

export type BookingCheckInState =
  /** Offer "เช็คอิน". `room` is null when only a count was known (list row) — the
   *  caller then fetches the booking detail to learn which room. */
  | { kind: 'ready'; room: BookingRoomRef | null; arrivalToday: boolean }
  /** Already checked in — show "เช็คอินแล้ว" and link to the folio when known. */
  | { kind: 'checked-in'; checkInId: number | null }
  /** Eligible on status, but no room is assigned yet — nothing to check into. */
  | { kind: 'no-room' }
  /** Cancelled / completed / no-show / unpaid-pending — say nothing at all. */
  | { kind: 'hidden' }

/** Minimal shape of `GET /api/checkins` rows (`NewCheckIn`) used by the lookup. */
export interface CheckInLite {
  id: number
  bookingId?: number | null
  status?: string | null
}

/**
 * The id of the OPEN check-in that already claims `bookingId`, or `null`.
 *
 * This is the double-check-in guard's PROACTIVE half: it answers before the
 * POST, so the button reads "เช็คอินแล้ว" instead of offering an action that
 * will fail.
 *
 * It is not the authority. Since B7b the backend refuses a second stay on one
 * booking inside the check-in transaction (`check_in_to_booking` →
 * `reject_double_checkin`), which is what closes the window this read cannot:
 * the state here is computed BEFORE the receptionist fills the form, so another
 * desk — or iHOTEL — can check the same booking in while she types. That
 * refusal arrives as `409 { reason: "booking_already_checked_in",
 * conflictingId }` and is decoded by {@link parseAlreadyCheckedIn}.
 *
 * `checkins` is expected to be an already-active-filtered list
 * (`?status=active`); the `status` re-check makes the function safe to call on
 * an unfiltered list too.
 */
export function openCheckInIdForBooking(
  checkins: readonly CheckInLite[] | null | undefined,
  bookingId: number,
): number | null {
  if (!checkins) return null
  const hit = checkins.find(
    (c) =>
      c.bookingId === bookingId &&
      (c.status == null || c.status === 'active' || c.status === 'checkedin'),
  )
  return hit ? hit.id : null
}

/** Booking statuses from which a check-in may be started at all. */
function statusAllowsCheckIn(status: string, depositAmount: number | null | undefined): boolean {
  switch (status.trim().toLowerCase()) {
    case 'confirmed':
      return true
    case 'pending':
      // A hold nobody has paid for is not an arrival. `book_deposit_amount > 0`
      // is the only "money has been taken" marker on `ht_bookings` — for a
      // loyalty booking it is written exactly once, by
      // `repository::channel::confirm_booking_payment` after the app verified
      // the guest's slip (the same signal `appDepositNoticeView` trusts).
      return typeof depositAmount === 'number' && Number.isFinite(depositAmount) && depositAmount > 0
    default:
      return false
  }
}

/**
 * Resolve what the check-in control should do for one booking.
 *
 * `openCheckInId` comes from {@link openCheckInIdForBooking}; pass `undefined`
 * when the caller has not loaded the active check-ins (the `'checkedin'` status
 * still catches the common case, just without a folio link).
 */
export function resolveBookingCheckIn(
  booking: BookingCheckInInput,
  openCheckInId?: number | null,
  now: Date = new Date(),
): BookingCheckInState {
  if (typeof openCheckInId === 'number') {
    return { kind: 'checked-in', checkInId: openCheckInId }
  }

  const status = (booking.status ?? '').trim().toLowerCase()
  if (status === CHECKED_IN_STATUS) {
    // The booking says it has been checked in but we could not resolve which
    // stay (no active list loaded, or the stay was created in iHOTEL and the
    // link arrived through the CT sync). Still refuse to offer a second one.
    return { kind: 'checked-in', checkInId: null }
  }

  if (!statusAllowsCheckIn(status, booking.depositAmount)) return { kind: 'hidden' }

  const rooms = booking.rooms
  if (rooms !== undefined && rooms !== null) {
    if (rooms.length === 0) return { kind: 'no-room' }
    return {
      kind: 'ready',
      room: rooms[0],
      arrivalToday: isSameStoredDay(booking.checkIn, now),
    }
  }

  if ((booking.roomCount ?? 0) <= 0) return { kind: 'no-room' }
  return { kind: 'ready', room: null, arrivalToday: isSameStoredDay(booking.checkIn, now) }
}

/** `CalendarBooking.id` prefix for a canonical (PG) booking — `routes/calendar.rs`. */
const NEW_BOOKING_ID_PREFIX = 'new-booking-'

/** One `data.bookings[]` entry of `GET /api/calendar` (`CalendarBooking`). */
export interface CalendarBookingEntry {
  id: string
  roomNo?: string | null
  checkIn?: string | null
  checkOut?: string | null
  status?: string | null
  source?: string | null
}

/** `YYYY-MM-DD` of a stored (naive Thai-local) datetime string, or `null`. */
function ymd(value: string | null | undefined): string | null {
  if (!value) return null
  const part = String(value).slice(0, 10)
  return /^\d{4}-\d{2}-\d{2}$/.test(part) ? part : null
}

/** Today as `YYYY-MM-DD` in the browser's (Thai) local clock. */
export function todayYmd(now: Date = new Date()): string {
  return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(
    now.getDate(),
  ).padStart(2, '0')}`
}

/**
 * Which booking makes this room read จองแล้ว today — the canonical booking id,
 * or `null`.
 *
 * This re-states, client-side, the EXACT predicate the backend paints the room
 * with (`LIVE_ROOM_FLAGS_SQL` in `routes/new_rooms.rs`):
 * `book_status IN ('confirmed','pending') AND book_checkin <= CURRENT_DATE AND
 * book_checkout > CURRENT_DATE`. Keeping the two in step matters: a room the
 * board calls `booked` whose booking this function cannot find is a dead
 * button, which is exactly the failure the sheet's notice reports.
 *
 * Legacy-sourced calendar rows are skipped — an iHOTEL-only booking has no
 * canonical `ht_bookings` row to link a check-in to, and the desk checks those
 * in inside iHOTEL (see docs/loyalty-channel.md §"Checking an app booking in").
 */
export function pickTodaysBookingIdForRoom(
  entries: readonly CalendarBookingEntry[] | null | undefined,
  roomNo: string,
  today: string = todayYmd(),
): number | null {
  if (!entries) return null
  const wanted = roomNo.trim()
  const candidates = entries.filter((e) => {
    if ((e.source ?? '') !== 'new') return false
    if ((e.roomNo ?? '').trim() !== wanted) return false
    const status = (e.status ?? '').trim().toLowerCase()
    if (status !== 'confirmed' && status !== 'pending') return false
    const inYmd = ymd(e.checkIn)
    const outYmd = ymd(e.checkOut)
    if (!inYmd || !outYmd) return false
    return inYmd <= today && outYmd > today
  })
  if (candidates.length === 0) return null
  // An arrival today beats a stay already in progress on paper but never
  // checked in — that is the guest standing at the desk.
  const best = candidates.find((e) => ymd(e.checkIn) === today) ?? candidates[0]
  const raw = best.id.startsWith(NEW_BOOKING_ID_PREFIX)
    ? best.id.slice(NEW_BOOKING_ID_PREFIX.length)
    : best.id
  const id = Number(raw)
  return Number.isInteger(id) && id > 0 ? id : null
}

/**
 * The backend's B7b refusal, decoded — `409` with
 * `reason: "booking_already_checked_in"`.
 *
 * Emitted by `CheckInService::check_in_to_booking` when the booking already has
 * as many OPEN check-ins as it has assigned rooms. It is the RACE half of the
 * rule above: {@link openCheckInIdForBooking} answers before the form is
 * filled, this answers at the moment of the write.
 *
 * Two things make a decoder worth having rather than a `res.ok` check:
 *
 * * the body's human text is **English and internal** ("booking 812 is already
 *   checked in (check-in 4242) …"), so a surface that renders `data.error` raw
 *   shows Thai-speaking reception a sentence with row ids in it;
 * * `conflictingId` is the folio that already exists, which is the only useful
 *   next action — without decoding it, the refusal is a dead end.
 *
 * `checkInId` is `null` when the backend could not name one (it never should;
 * the guard only fires with at least one open stay), so callers must render the
 * message without a link in that case rather than linking to `/billing/null`.
 */
export const BOOKING_ALREADY_CHECKED_IN_REASON = 'booking_already_checked_in'

export interface AlreadyCheckedInRefusal {
  checkInId: number | null
}

export function parseAlreadyCheckedIn(
  status: number,
  body: unknown,
): AlreadyCheckedInRefusal | null {
  if (status !== 409 || typeof body !== 'object' || body === null) return null
  const data = body as { reason?: unknown; conflictingId?: unknown }
  if (data.reason !== BOOKING_ALREADY_CHECKED_IN_REASON) return null
  const id = data.conflictingId
  return { checkInId: typeof id === 'number' && Number.isFinite(id) ? id : null }
}

/**
 * What reception reads. Thai, no row ids, no English — the backend's own
 * message is a developer artefact and must never reach the desk.
 */
export const ALREADY_CHECKED_IN_MESSAGE =
  'การจองนี้เช็คอินไปแล้ว — เปิดใบแจ้งหนี้เดิมแทนการเช็คอินซ้ำ'

/**
 * Where "เช็คอินแล้ว" goes: the folio for that stay.
 *
 * `/billing/[id]` (not `/v2/invoice/[id]`) is deliberate — the folio is the
 * page that carries the B7 `AppDepositNotice`, the per-room deposit panel and
 * the payment dialog, which is the whole reason reception follows this link.
 */
export function folioHref(checkInId: number): string {
  return `/billing/${checkInId}`
}
