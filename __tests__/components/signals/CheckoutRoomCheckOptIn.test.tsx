/**
 * @jest-environment jsdom
 *
 * Where the room-signal UI is allowed to appear, and where it must not.
 *
 * `components/CheckOutModal` is mounted by THREE surfaces — the v2 desk rooms
 * board, the v1 dashboard, and the v1 rooms page — and only the v2 desk is in
 * the ADR 0008 build. The panel is therefore opt-in via the `roomCheck` prop.
 * This file is the leak test: without the prop the checkout modal must render
 * no ขอเช็คห้อง control AND issue no signal read, so the v1 screens are
 * byte-for-byte what they were.
 *
 * It also pins the improvement invariant on the surface that DOES opt in:
 * every pre-existing checkout element is still present, still in order, and
 * the room-check never fires on open.
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
import { REQUEST_LABEL } from '@/components/v2/signals/RoomCheckPanel'

const CHECKIN = {
  id: 501,
  cinNo: 'CI-0001',
  customerName: 'สมชาย',
  checkInTime: '2026-08-30T14:00:00',
  expectedCheckout: '2026-09-01T12:00:00',
}

const QUOTE = {
  success: true,
  nights: 2,
  ratePerNight: 900,
  roomTotal: 1800,
  productTotal: 0,
  vatPercent: 0,
  vat: 0,
  deposit: 0,
  netTotal: 1800,
  payTotal: 1800,
  balance: 0,
}

function jsonResponse(body: unknown, status = 200) {
  return { ok: status >= 200 && status < 300, status, json: async () => body }
}

let calls: string[] = []

beforeEach(() => {
  jest.clearAllMocks()
  calls = []
  branchFetchMock.mockImplementation(async (url: string) => {
    calls.push(url)
    if (url.startsWith('/api/checkins?')) return jsonResponse({ success: true, data: [CHECKIN] })
    if (url.includes('/checkout-quote')) return jsonResponse(QUOTE)
    if (url.endsWith('/rooms')) return jsonResponse({ success: true, data: [] })
    if (url.startsWith('/api/housekeeping/signals')) {
      return jsonResponse({ success: true, signals: [] })
    }
    return jsonResponse({ success: false }, 404)
  })
})

const room = { id: 7, roomNo: '101' }

describe('CheckOutModal without the opt-in — the v1 mounts', () => {
  it('renders no ขอเช็คห้อง control', async () => {
    render(<CheckOutModal room={room} onClose={jest.fn()} onSuccess={jest.fn()} />)
    expect(await screen.findByText('CI-0001')).toBeInTheDocument()
    expect(screen.queryByText(REQUEST_LABEL)).not.toBeInTheDocument()
  })

  it('reads no signals endpoint at all', async () => {
    render(<CheckOutModal room={room} onClose={jest.fn()} onSuccess={jest.fn()} />)
    expect(await screen.findByText('CI-0001')).toBeInTheDocument()
    expect(calls.some((url) => url.includes('/housekeeping/signals'))).toBe(false)
  })
})

describe('CheckOutModal with roomCheck — the v2 desk mount', () => {
  it('renders the ขอเช็คห้อง control', async () => {
    render(<CheckOutModal room={room} onClose={jest.fn()} onSuccess={jest.fn()} roomCheck />)
    expect(await screen.findByText(REQUEST_LABEL)).toBeInTheDocument()
  })

  it('does NOT auto-fire the check on open — the desk asks manually', async () => {
    render(<CheckOutModal room={room} onClose={jest.fn()} onSuccess={jest.fn()} roomCheck />)
    expect(await screen.findByText(REQUEST_LABEL)).toBeInTheDocument()
    await waitFor(() => expect(calls.some((url) => url.includes('/housekeeping/signals'))).toBe(true))
    expect(calls.some((url) => /\/rooms\/\d+\/signals$/.test(url))).toBe(false)
  })

  it('keeps every existing checkout element, additively', async () => {
    render(<CheckOutModal room={room} onClose={jest.fn()} onSuccess={jest.fn()} roomCheck />)
    // The folio arrives in a second read; wait for it before auditing the body.
    expect(await screen.findByText('รวมทั้งหมด')).toBeInTheDocument()
    expect(screen.getByText('CI-0001')).toBeInTheDocument()
    expect(screen.getByText('สมชาย')).toBeInTheDocument()
    expect(screen.getByText('เลขที่เช็คอิน:')).toBeInTheDocument()
    expect(screen.getByText('คงเหลือ')).toBeInTheDocument()
    expect(screen.getByText('ไม่มียอดค้างชำระ เช็คเอ้าท์ได้ทันที')).toBeInTheDocument()
    expect(screen.getByText('ยืนยันเช็คเอ้าท์')).toBeInTheDocument()
  })

  it('leaves the confirm button enabled — the panel informs, it does not gate', async () => {
    render(<CheckOutModal room={room} onClose={jest.fn()} onSuccess={jest.fn()} roomCheck />)
    await screen.findByText(REQUEST_LABEL)
    await screen.findByText('รวมทั้งหมด')
    expect(screen.getByText('ยืนยันเช็คเอ้าท์').closest('button')).not.toBeDisabled()
  })

  it('appends the panel BELOW the settle controls, so no existing step moves', async () => {
    render(<CheckOutModal room={room} onClose={jest.fn()} onSuccess={jest.fn()} roomCheck />)
    const request = await screen.findByText(REQUEST_LABEL)
    const tenderNote = await screen.findByText('ไม่มียอดค้างชำระ เช็คเอ้าท์ได้ทันที')
    // Node.DOCUMENT_POSITION_FOLLOWING === 4
    expect(tenderNote.compareDocumentPosition(request) & 4).toBeTruthy()
  })
})
