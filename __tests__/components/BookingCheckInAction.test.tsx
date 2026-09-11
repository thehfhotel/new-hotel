/**
 * @jest-environment jsdom
 *
 * Task B7a — when the desk may check a reservation in from OUR app, when it
 * must not, and what it sees instead.
 *
 * The rule is pinned here rather than checked by eye on three screens, because
 * the three screens disagreeing is the actual failure mode: the reservations
 * list, the reservation detail and the room board's action sheet all call
 * `resolveBookingCheckIn`, and two of them feed it different fields (`roomCount`
 * vs `rooms`) that must still produce the same answer.
 */

import { render, screen, fireEvent } from '@testing-library/react'

// next/link bottoms out into router APIs that jsdom doesn't bootstrap.
jest.mock('next/navigation', () => ({
  useRouter: () => ({ replace: jest.fn(), push: jest.fn() }),
  usePathname: () => '/v2/reservations',
  useSearchParams: () => new URLSearchParams(),
}))

import BookingCheckInAction, {
  CHECK_IN_LABEL,
  CHECKED_IN_LABEL,
  NO_ROOM_LABEL,
} from '@/components/v2/BookingCheckInAction'
import {
  folioHref,
  openCheckInIdForBooking,
  pickTodaysBookingIdForRoom,
  resolveBookingCheckIn,
} from '@/lib/v2/checkin-from-booking'

const TODAY = new Date('2026-09-11T09:00:00')

describe('resolveBookingCheckIn — who may be checked in here', () => {
  test('a confirmed booking with an assigned room is ready', () => {
    const state = resolveBookingCheckIn(
      { status: 'confirmed', checkIn: '2026-09-11T00:00:00', rooms: [{ roomId: 7, roomNo: '301' }] },
      null,
      TODAY,
    )
    expect(state).toEqual({ kind: 'ready', room: { roomId: 7, roomNo: '301' }, arrivalToday: true })
  })

  test('a PENDING booking is ready only once a deposit has been recorded', () => {
    // The unpaid hold: reception has no money and no confirmation — an arrival
    // it is not. Offering check-in here would let an expiring 2h loyalty hold
    // become an occupied room.
    expect(
      resolveBookingCheckIn({ status: 'pending', depositAmount: 0, roomCount: 1 }, null, TODAY),
    ).toEqual({ kind: 'hidden' })
    expect(
      resolveBookingCheckIn({ status: 'pending', depositAmount: null, roomCount: 1 }, null, TODAY),
    ).toEqual({ kind: 'hidden' })

    // Deposit verified → `confirm_booking_payment` wrote `book_deposit_amount`.
    expect(
      resolveBookingCheckIn({ status: 'pending', depositAmount: 600, roomCount: 1 }, null, TODAY),
    ).toEqual({ kind: 'ready', room: null, arrivalToday: false })
  })

  test('cancelled, completed and no-show say nothing at all', () => {
    for (const status of ['cancelled', 'completed', 'noshow']) {
      expect(
        resolveBookingCheckIn({ status, depositAmount: 900, roomCount: 1 }, null, TODAY),
      ).toEqual({ kind: 'hidden' })
    }
  })

  test('an eligible booking with NO room assigned reports why, rather than vanishing', () => {
    expect(resolveBookingCheckIn({ status: 'confirmed', rooms: [] }, null, TODAY)).toEqual({
      kind: 'no-room',
    })
    // The list row only knows a count (B8a parked bookings are exactly this).
    expect(resolveBookingCheckIn({ status: 'confirmed', roomCount: 0 }, null, TODAY)).toEqual({
      kind: 'no-room',
    })
  })

  test('the two DTO shapes agree — list `roomCount` vs detail `rooms`', () => {
    const list = resolveBookingCheckIn(
      { status: 'confirmed', checkIn: '2026-09-11T00:00:00', roomCount: 1 },
      null,
      TODAY,
    )
    const detail = resolveBookingCheckIn(
      { status: 'confirmed', checkIn: '2026-09-11T00:00:00', rooms: [{ roomId: 7, roomNo: '301' }] },
      null,
      TODAY,
    )
    expect(list.kind).toBe('ready')
    expect(detail.kind).toBe('ready')
  })

  test('arrivalToday follows the STORED date prefix, not a parsed Date', () => {
    // Canonical datetimes are naive Thai-local; parsing "…T00:00:00" in a UTC+7
    // browser would shift to the previous day and mislabel today's arrivals.
    const today = resolveBookingCheckIn(
      { status: 'confirmed', checkIn: '2026-09-11T00:00:00', roomCount: 1 },
      null,
      TODAY,
    )
    const tomorrow = resolveBookingCheckIn(
      { status: 'confirmed', checkIn: '2026-09-12T00:00:00', roomCount: 1 },
      null,
      TODAY,
    )
    expect(today).toMatchObject({ arrivalToday: true })
    expect(tomorrow).toMatchObject({ arrivalToday: false })
  })

  test('an early arrival is still offered — iHOTEL allows it, so we must', () => {
    const state = resolveBookingCheckIn(
      { status: 'confirmed', checkIn: '2026-10-01T00:00:00', roomCount: 1 },
      null,
      TODAY,
    )
    expect(state.kind).toBe('ready')
  })
})

describe('resolveBookingCheckIn — the double-check-in guard', () => {
  test('an open check-in linked to the booking wins over any status', () => {
    expect(
      resolveBookingCheckIn({ status: 'confirmed', roomCount: 1 }, 501, TODAY),
    ).toEqual({ kind: 'checked-in', checkInId: 501 })
  })

  test('booking status `checkedin` blocks a second check-in even with no stay resolved', () => {
    // The stay may have been created in iHOTEL and arrived through the CT sync,
    // so our active-check-ins read can legitimately not know its id yet.
    expect(resolveBookingCheckIn({ status: 'checkedin', roomCount: 1 }, null, TODAY)).toEqual({
      kind: 'checked-in',
      checkInId: null,
    })
  })

  test('openCheckInIdForBooking matches on bookingId, ignoring other stays', () => {
    const checkins = [
      { id: 11, bookingId: null, status: 'active' },
      { id: 12, bookingId: 99, status: 'active' },
      { id: 13, bookingId: 42, status: 'active' },
    ]
    expect(openCheckInIdForBooking(checkins, 42)).toBe(13)
    expect(openCheckInIdForBooking(checkins, 7)).toBeNull()
    expect(openCheckInIdForBooking(undefined, 42)).toBeNull()
  })

  test('openCheckInIdForBooking ignores a closed stay on the same booking', () => {
    // A guest who stayed on this booking and checked out must not block a
    // re-check-in decision with a stale row if an unfiltered list is passed.
    const checkins = [{ id: 20, bookingId: 42, status: 'checkedout' }]
    expect(openCheckInIdForBooking(checkins, 42)).toBeNull()
  })
})

describe('pickTodaysBookingIdForRoom — the room board’s จองแล้ว predicate', () => {
  const entries = [
    {
      id: 'new-booking-42',
      roomNo: '301',
      checkIn: '2026-09-11T00:00:00Z',
      checkOut: '2026-09-13T00:00:00Z',
      status: 'confirmed',
      source: 'new',
    },
    {
      id: 'legacy-booking-R000123',
      roomNo: '302',
      checkIn: '2026-09-11T00:00:00Z',
      checkOut: '2026-09-13T00:00:00Z',
      status: 'confirmed',
      source: 'legacy',
    },
    {
      id: 'new-booking-43',
      roomNo: '303',
      checkIn: '2026-09-09T00:00:00Z',
      checkOut: '2026-09-11T00:00:00Z',
      status: 'confirmed',
      source: 'new',
    },
  ]

  test('finds the canonical booking covering today on that room', () => {
    expect(pickTodaysBookingIdForRoom(entries, '301', '2026-09-11')).toBe(42)
  })

  test('skips a legacy-only booking — there is no ht_bookings row to link to', () => {
    expect(pickTodaysBookingIdForRoom(entries, '302', '2026-09-11')).toBeNull()
  })

  test('excludes a booking whose checkout is TODAY — mirrors book_checkout > CURRENT_DATE', () => {
    expect(pickTodaysBookingIdForRoom(entries, '303', '2026-09-11')).toBeNull()
  })

  test('prefers the arrival-today booking over a stay already running', () => {
    const overlapping = [
      {
        id: 'new-booking-50',
        roomNo: '401',
        checkIn: '2026-09-10T00:00:00Z',
        checkOut: '2026-09-14T00:00:00Z',
        status: 'confirmed',
        source: 'new',
      },
      {
        id: 'new-booking-51',
        roomNo: '401',
        checkIn: '2026-09-11T00:00:00Z',
        checkOut: '2026-09-12T00:00:00Z',
        status: 'pending',
        source: 'new',
      },
    ]
    expect(pickTodaysBookingIdForRoom(overlapping, '401', '2026-09-11')).toBe(51)
  })

  test('returns null for an unknown room rather than guessing', () => {
    expect(pickTodaysBookingIdForRoom(entries, '999', '2026-09-11')).toBeNull()
    expect(pickTodaysBookingIdForRoom([], '301', '2026-09-11')).toBeNull()
  })
})

describe('BookingCheckInAction — what reception sees', () => {
  test('available: a เช็คอิน button that fires the handler', () => {
    const onCheckIn = jest.fn()
    render(
      <BookingCheckInAction
        state={{ kind: 'ready', room: { roomId: 7, roomNo: '301' }, arrivalToday: true }}
        onCheckIn={onCheckIn}
      />,
    )
    const btn = screen.getByTestId('booking-checkin-action')
    expect(btn).toHaveTextContent(CHECK_IN_LABEL)
    expect(btn).toHaveAttribute('data-arrival-today', 'true')
    fireEvent.click(btn)
    expect(onCheckIn).toHaveBeenCalledTimes(1)
  })

  test('available: a future arrival is drawn quietly, not as the primary action', () => {
    render(
      <BookingCheckInAction
        state={{ kind: 'ready', room: null, arrivalToday: false }}
        onCheckIn={jest.fn()}
      />,
    )
    const btn = screen.getByTestId('booking-checkin-action')
    expect(btn).toHaveAttribute('data-arrival-today', 'false')
    expect(btn.className).not.toContain('v2-btn-primary')
  })

  test('available: busy disables the button so a slow resolve cannot double-fire', () => {
    const onCheckIn = jest.fn()
    render(
      <BookingCheckInAction
        state={{ kind: 'ready', room: null, arrivalToday: true }}
        onCheckIn={onCheckIn}
        busy
      />,
    )
    const btn = screen.getByTestId('booking-checkin-action')
    expect(btn).toBeDisabled()
    fireEvent.click(btn)
    expect(onCheckIn).not.toHaveBeenCalled()
  })

  test('already checked in: says so and links to the folio', () => {
    render(
      <BookingCheckInAction state={{ kind: 'checked-in', checkInId: 501 }} onCheckIn={jest.fn()} />,
    )
    const link = screen.getByTestId('booking-checked-in')
    expect(link).toHaveTextContent(CHECKED_IN_LABEL)
    expect(link).toHaveAttribute('href', folioHref(501))
    // No way to start a second check-in from this state.
    expect(screen.queryByTestId('booking-checkin-action')).not.toBeInTheDocument()
  })

  test('already checked in, stay unknown: still blocks, just without a link', () => {
    render(
      <BookingCheckInAction state={{ kind: 'checked-in', checkInId: null }} onCheckIn={jest.fn()} />,
    )
    const badge = screen.getByTestId('booking-checked-in')
    expect(badge).toHaveTextContent(CHECKED_IN_LABEL)
    expect(badge).not.toHaveAttribute('href')
    expect(screen.queryByTestId('booking-checkin-action')).not.toBeInTheDocument()
  })

  test('no room assigned: a disabled control that names the blocker', () => {
    const onCheckIn = jest.fn()
    render(<BookingCheckInAction state={{ kind: 'no-room' }} onCheckIn={onCheckIn} />)
    const btn = screen.getByTestId('booking-checkin-no-room')
    expect(btn).toHaveTextContent(NO_ROOM_LABEL)
    expect(btn).toBeDisabled()
    fireEvent.click(btn)
    expect(onCheckIn).not.toHaveBeenCalled()
  })

  test('hidden renders nothing', () => {
    const { container } = render(
      <BookingCheckInAction state={{ kind: 'hidden' }} onCheckIn={jest.fn()} />,
    )
    expect(container).toBeEmptyDOMElement()
  })

  test('a read-only branch gets no check-in button, but still sees an existing stay', () => {
    const { rerender, container } = render(
      <BookingCheckInAction
        state={{ kind: 'ready', room: null, arrivalToday: true }}
        onCheckIn={jest.fn()}
        disabled
      />,
    )
    expect(container).toBeEmptyDOMElement()

    rerender(
      <BookingCheckInAction
        state={{ kind: 'checked-in', checkInId: 9 }}
        onCheckIn={jest.fn()}
        disabled
      />,
    )
    expect(screen.getByTestId('booking-checked-in')).toBeInTheDocument()
  })
})
