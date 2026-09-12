/**
 * @jest-environment jsdom
 *
 * Task B7a — the B7 app-deposit signpost on the CHECKOUT modal.
 *
 * B7 wired the notice into the folio, the registration card, the reservation
 * detail and the payment dialog, but not here — and checkout is the last and
 * most expensive moment for the divergence to bite: iHOTEL has shown this
 * booking's deposit as 0 for the whole stay, and this is the screen where the
 * remaining balance is taken. So the notice must be ABOVE the folio and the
 * tender select, not below them.
 *
 * The read rides `GET /api/checkins/:id/deposits`, which already carries
 * `bookChannel` + `bookingDepositAmount` (B7, PR #302) — the checkout-quote DTO
 * stays money-only, so no backend change was needed for this.
 */

import { render, screen, waitFor } from '@testing-library/react'

const branchFetchMock = jest.fn()

jest.mock('@/lib/use-branch-fetch', () => ({
  useBranchFetch: () => branchFetchMock,
}))

jest.mock('@/contexts/BranchContext', () => ({
  useBranch: () => ({ branch: 'hfhotel', canWrite: true }),
}))

jest.mock('@/lib/v2/use-live-refresh', () => ({
  useLiveRefresh: () => true,
}))

import CheckOutModal from '@/components/CheckOutModal'

const CHECKIN = {
  id: 501,
  cinNo: 'CI-0501',
  customerName: 'สมชาย',
  checkInTime: '2026-09-08T14:00:00',
  expectedCheckout: '2026-09-11T12:00:00',
}

const QUOTE = {
  success: true,
  nights: 3,
  ratePerNight: 900,
  roomTotal: 2700,
  productTotal: 0,
  vatPercent: 0,
  vat: 0,
  deposit: 0,
  netTotal: 2700,
  payTotal: 0,
  balance: 2700,
}

function jsonResponse(body: unknown, status = 200) {
  return { ok: status >= 200 && status < 300, status, json: async () => body }
}

const room = { id: 7, roomNo: '301' }

/** Wire the modal's four reads; `deposits` is the one under test. */
function mockFetch(deposits: Record<string, unknown>) {
  branchFetchMock.mockImplementation(async (url: string) => {
    if (url.startsWith('/api/checkins?')) return jsonResponse({ success: true, data: [CHECKIN] })
    if (url.includes('/checkout-quote')) return jsonResponse(QUOTE)
    if (url.endsWith('/deposits')) return jsonResponse(deposits)
    if (url.endsWith('/rooms')) return jsonResponse({ success: true, data: [] })
    if (url.startsWith('/api/housekeeping/signals')) {
      return jsonResponse({ success: true, signals: [] })
    }
    return jsonResponse({ success: false }, 404)
  })
}

beforeEach(() => {
  jest.clearAllMocks()
})

describe('CheckOutModal — app-deposit signpost', () => {
  test('warns before the desk takes the balance on an app booking', async () => {
    mockFetch({ success: true, deposits: [], bookChannel: 'loyalty', bookingDepositAmount: 600 })
    render(<CheckOutModal room={room} onClose={jest.fn()} onSuccess={jest.fn()} />)

    const notice = await screen.findByTestId('app-deposit-notice')
    expect(notice).toHaveTextContent('ชำระผ่านแอปแล้ว')
    expect(notice).toHaveTextContent('iHOTEL')
  })

  test('sits ABOVE the folio and the tender select, where it can still stop a second charge', async () => {
    mockFetch({ success: true, deposits: [], bookChannel: 'loyalty', bookingDepositAmount: 600 })
    render(<CheckOutModal room={room} onClose={jest.fn()} onSuccess={jest.fn()} />)

    const notice = await screen.findByTestId('app-deposit-notice')
    const folioTotal = await screen.findByText('รวมทั้งหมด')
    // Node.DOCUMENT_POSITION_FOLLOWING === 4
    expect(notice.compareDocumentPosition(folioTotal) & 4).toBeTruthy()
  })

  test('an app booking whose rooms took NO desk deposit still gets the notice', async () => {
    // The normal shape for an app booking: zero `ht_checkin_rooms` deposit rows,
    // so keying the notice off the per-room list would have shown nothing.
    mockFetch({ success: true, deposits: [], bookChannel: 'loyalty', bookingDepositAmount: 1200 })
    render(<CheckOutModal room={room} onClose={jest.fn()} onSuccess={jest.fn()} />)
    expect(await screen.findByTestId('app-deposit-notice')).toBeInTheDocument()
  })

  test('stays silent for a walk-in stay', async () => {
    mockFetch({ success: true, deposits: [], bookChannel: null, bookingDepositAmount: null })
    render(<CheckOutModal room={room} onClose={jest.fn()} onSuccess={jest.fn()} />)

    expect(await screen.findByText('CI-0501')).toBeInTheDocument()
    await waitFor(() => expect(screen.getByText('รวมทั้งหมด')).toBeInTheDocument())
    expect(screen.queryByTestId('app-deposit-notice')).not.toBeInTheDocument()
  })

  test('stays silent for an OTA stay — that money sits with the agency', async () => {
    mockFetch({ success: true, deposits: [], bookChannel: 'agoda', bookingDepositAmount: 900 })
    render(<CheckOutModal room={room} onClose={jest.fn()} onSuccess={jest.fn()} />)

    expect(await screen.findByText('CI-0501')).toBeInTheDocument()
    await waitFor(() => expect(screen.getByText('รวมทั้งหมด')).toBeInTheDocument())
    expect(screen.queryByTestId('app-deposit-notice')).not.toBeInTheDocument()
  })

  test('a failed deposits read never blocks checkout', async () => {
    mockFetch({ success: false })
    render(<CheckOutModal room={room} onClose={jest.fn()} onSuccess={jest.fn()} />)

    expect(await screen.findByText('CI-0501')).toBeInTheDocument()
    expect(await screen.findByText('รวมทั้งหมด')).toBeInTheDocument()
    expect(screen.queryByTestId('app-deposit-notice')).not.toBeInTheDocument()
    expect(screen.getByText('ชำระเงินและเช็คเอ้าท์').closest('button')).not.toBeDisabled()
  })

  test('every pre-existing checkout element survives, additively', async () => {
    mockFetch({ success: true, deposits: [], bookChannel: 'loyalty', bookingDepositAmount: 600 })
    render(<CheckOutModal room={room} onClose={jest.fn()} onSuccess={jest.fn()} />)

    expect(await screen.findByText('รวมทั้งหมด')).toBeInTheDocument()
    expect(screen.getByText('CI-0501')).toBeInTheDocument()
    expect(screen.getByText('สมชาย')).toBeInTheDocument()
    expect(screen.getByText('เลขที่เช็คอิน:')).toBeInTheDocument()
    expect(screen.getByText('คงเหลือ')).toBeInTheDocument()
    expect(screen.getByText('ชำระเงินและเช็คเอ้าท์')).toBeInTheDocument()
  })
})
