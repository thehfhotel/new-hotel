/**
 * @jest-environment jsdom
 *
 * Task B7a — `CheckInModal` in from-reservation mode.
 *
 * The bug this closes is invisible at the desk and expensive later: every
 * frontend POST /api/checkins was room-only, so a stay created in our app had
 * `cin_book_id` NULL and every B7 app-deposit signpost downstream (folio,
 * payment dialog, checkout) resolved to nothing on exactly the guest it exists
 * for. So the load-bearing assertion here is a REQUEST BODY, not a pixel.
 *
 * The second assertion is a money one: the booking deposit must NOT be
 * pre-filled into the desk deposit field. Those are two different pots —
 * `ht_bookings.book_deposit_amount` is in the bank, `ht_checkin_rooms
 * .cr_dep_amount` (→ legacy `HT_CheckIn_Ds.Cin_Room_Dep`) is in the drawer —
 * and copying one into the other would hand the guest a cash refund at
 * checkout for money they transferred.
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'

const branchFetchMock = jest.fn()

jest.mock('@/lib/use-branch-fetch', () => ({
  useBranchFetch: () => branchFetchMock,
}))

jest.mock('@/contexts/BranchContext', () => ({
  useBranch: () => ({ branch: 'hfhotel', canWrite: true }),
}))

import CheckInModal, { type CheckInBookingContext } from '@/components/CheckInModal'

const ROOM = { id: 7, roomNo: '301', roomTypeName: 'Standard' }

const APP_BOOKING: CheckInBookingContext = {
  id: 42,
  bookNo: 'B-2569-0042',
  customerName: 'สมชาย ใจดี',
  checkIn: '2026-09-11T00:00:00',
  checkOut: '2026-09-14T00:00:00',
  adults: 2,
  children: 1,
  depositAmount: 600,
  bookChannel: 'loyalty',
}

function jsonResponse(body: unknown, status = 200) {
  return { ok: status >= 200 && status < 300, status, json: async () => body }
}

type Call = { url: string; init?: RequestInit }
let calls: Call[] = []

beforeEach(() => {
  jest.clearAllMocks()
  calls = []
  branchFetchMock.mockImplementation(async (url: string, init?: RequestInit) => {
    calls.push({ url, init })
    if (url === '/api/checkins') {
      return jsonResponse({ success: true, id: 501, cinNo: 'CIN-2569-0501' })
    }
    if (url.includes('/registration-slip')) return jsonResponse({ success: true })
    if (url.startsWith('/api/customers')) {
      return jsonResponse({ success: true, id: 999, data: [] })
    }
    return jsonResponse({ success: false }, 404)
  })
})

/** The body of the POST /api/checkins call, parsed. */
function checkinBody(): Record<string, unknown> {
  const call = calls.find((c) => c.url === '/api/checkins' && c.init?.method === 'POST')
  if (!call) throw new Error('no POST /api/checkins was made')
  return JSON.parse(String(call.init!.body))
}

describe('CheckInModal from a reservation', () => {
  test('sends bookingId — the whole point of B7a', async () => {
    render(
      <CheckInModal room={ROOM} booking={APP_BOOKING} onClose={jest.fn()} onSuccess={jest.fn()} />,
    )
    fireEvent.click(screen.getByRole('button', { name: 'เช็คอิน' }))

    await waitFor(() => expect(checkinBody().bookingId).toBe(42))
    expect(checkinBody().roomId).toBe(7)
  })

  test('does NOT create a customer — the backend resolves it from the booking', async () => {
    render(
      <CheckInModal room={ROOM} booking={APP_BOOKING} onClose={jest.fn()} onSuccess={jest.fn()} />,
    )
    fireEvent.click(screen.getByRole('button', { name: 'เช็คอิน' }))

    await waitFor(() => expect(checkinBody().bookingId).toBe(42))
    // No POST /api/customers, and no customerId claimed in the body.
    expect(
      calls.some((c) => c.url === '/api/customers' && c.init?.method === 'POST'),
    ).toBe(false)
    expect(checkinBody().customerId).toBeUndefined()
  })

  test('pre-fills the booking’s dates and party size', async () => {
    render(
      <CheckInModal room={ROOM} booking={APP_BOOKING} onClose={jest.fn()} onSuccess={jest.fn()} />,
    )
    fireEvent.click(screen.getByRole('button', { name: 'เช็คอิน' }))

    await waitFor(() => expect(checkinBody().bookingId).toBe(42))
    const body = checkinBody()
    expect(body.expectedCheckout).toBe('2026-09-14')
    expect(body.adults).toBe(2)
    expect(body.children).toBe(1)
  })

  test('shows the booking and its guest, instead of empty walk-in name fields', () => {
    render(
      <CheckInModal room={ROOM} booking={APP_BOOKING} onClose={jest.fn()} onSuccess={jest.fn()} />,
    )
    const summary = screen.getByTestId('checkin-booking-summary')
    expect(summary).toHaveTextContent('B-2569-0042')
    expect(summary).toHaveTextContent('สมชาย ใจดี')
    // The walk-in customer capture is gone — it would be a field that writes
    // nothing, since the customer comes from the booking.
    expect(screen.queryByLabelText('ชื่อ *')).not.toBeInTheDocument()
    expect(screen.queryByText('ชื่อ *')).not.toBeInTheDocument()
  })

  test('carries the B7 app-deposit notice into the check-in screen', () => {
    render(
      <CheckInModal room={ROOM} booking={APP_BOOKING} onClose={jest.fn()} onSuccess={jest.fn()} />,
    )
    const notice = screen.getByTestId('app-deposit-notice')
    expect(notice).toHaveTextContent('ชำระผ่านแอปแล้ว')
    expect(notice).toHaveTextContent('iHOTEL')
  })

  test('stays silent about an app deposit for a walk-in desk booking', () => {
    render(
      <CheckInModal
        room={ROOM}
        booking={{ ...APP_BOOKING, bookChannel: null, depositAmount: 500 }}
        onClose={jest.fn()}
        onSuccess={jest.fn()}
      />,
    )
    expect(screen.queryByTestId('app-deposit-notice')).not.toBeInTheDocument()
  })

  test('NEVER pre-fills the booking deposit into the desk deposit field', async () => {
    render(
      <CheckInModal room={ROOM} booking={APP_BOOKING} onClose={jest.fn()} onSuccess={jest.fn()} />,
    )
    // Blank on screen, with reception told why…
    const input = screen.getByPlaceholderText('0') as HTMLInputElement
    expect(input.value).toBe('')
    expect(screen.getByTestId('checkin-deposit-hint')).toBeInTheDocument()
    // …and absent from the wire, so `cr_dep_amount` / `Cin_Room_Dep` stay 0.
    fireEvent.click(screen.getByRole('button', { name: 'เช็คอิน' }))
    await waitFor(() => expect(checkinBody().bookingId).toBe(42))
    expect(checkinBody().depositAmount).toBeUndefined()
  })

  test('a deposit typed at the counter IS sent — the field still works', async () => {
    render(
      <CheckInModal room={ROOM} booking={APP_BOOKING} onClose={jest.fn()} onSuccess={jest.fn()} />,
    )
    fireEvent.change(screen.getByPlaceholderText('0'), { target: { value: '300' } })
    fireEvent.click(screen.getByRole('button', { name: 'เช็คอิน' }))

    await waitFor(() => expect(checkinBody().bookingId).toBe(42))
    expect(checkinBody().depositAmount).toBe(300)
  })
})

describe('CheckInModal as a walk-in — unchanged', () => {
  test('still posts a customer first and sends NO bookingId', async () => {
    render(<CheckInModal room={ROOM} onClose={jest.fn()} onSuccess={jest.fn()} />)

    // The walk-in form requires a name; fill the first text input (ชื่อ *).
    const inputs = document.querySelectorAll('input[type="text"]')
    fireEvent.change(inputs[0], { target: { value: 'Walk In' } })
    fireEvent.click(screen.getByRole('button', { name: 'เช็คอิน' }))

    await waitFor(() =>
      expect(calls.some((c) => c.url === '/api/customers' && c.init?.method === 'POST')).toBe(true),
    )
    await waitFor(() => expect(checkinBody().customerId).toBe(999))
    expect(checkinBody().bookingId).toBeUndefined()
  })

  test('shows no booking summary and no app-deposit notice', () => {
    render(<CheckInModal room={ROOM} onClose={jest.fn()} onSuccess={jest.fn()} />)
    expect(screen.queryByTestId('checkin-booking-summary')).not.toBeInTheDocument()
    expect(screen.queryByTestId('app-deposit-notice')).not.toBeInTheDocument()
    expect(screen.queryByTestId('checkin-deposit-hint')).not.toBeInTheDocument()
  })
})
