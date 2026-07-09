/**
 * @jest-environment jsdom
 *
 * Guest-move drag confirm dialog (#225, ADR 0003) — the additive alternate to
 * ChangeRoomModal. Contract pinned here:
 * - Looks up the active check-in for the source room on mount; shows guest,
 *   from/to room, and the price implication when the room type differs
 *   (improvement invariant: every value visible before commit).
 * - Warns when the target is vacant-dirty.
 * - Posts `{ fromRoomId, toRoomId, reason }` to the SAME
 *   `/api/checkins/:cinId/change-room` endpoint the RoomActionSheet path uses.
 * - On success offers the room-change slip print, exactly like ChangeRoomModal.
 */

import { fireEvent, render, screen, waitFor } from '@testing-library/react'
import GuestMoveConfirmModal from '@/components/v2/GuestMoveConfirmModal'
import type { SpatialRoom } from '@/lib/v2/spatial-grid'

// Mock lucide-react icons so we don't pull in SVG renderers.
jest.mock('lucide-react', () => ({
  AlertCircle: () => <span data-testid="alert-icon" />,
  AlertTriangle: () => <span data-testid="warn-icon" />,
  ArrowRightLeft: () => <span data-testid="swap-icon" />,
  CheckCircle2: () => <span data-testid="check-icon" />,
  Loader2: () => <span data-testid="loader-icon" />,
  Printer: () => <span data-testid="printer-icon" />,
  X: () => <span data-testid="x-icon" />,
}))

// Stub the branch-fetch hook so the modal hits a controllable fake.
jest.mock('@/lib/use-branch-fetch', () => ({
  useBranchFetch: () => global.fetch,
}))

interface FetchCall {
  url: string
  init?: RequestInit
}

function setupFetch(handlers: Array<(url: string) => unknown>) {
  const calls: FetchCall[] = []
  const fetchMock = jest.fn(
    async (input: string | URL | Request, init?: RequestInit) => {
      const url = typeof input === 'string' ? input : input.toString()
      calls.push({ url, init })
      const handler = handlers.shift()
      const body = handler ? handler(url) : { success: false }
      return {
        ok: true,
        status: 200,
        json: async () => body,
      } as unknown as Response
    },
  ) as jest.Mock
  global.fetch = fetchMock as unknown as typeof fetch
  return { fetchMock, calls }
}

function room(partial: Partial<SpatialRoom> & { id: number; roomNo: string }): SpatialRoom {
  return {
    roomTypeName: 'Standard',
    floor: 1,
    status: 'available',
    isClean: true,
    isMaintenance: false,
    ...partial,
  } as SpatialRoom
}

const FROM = room({
  id: 5,
  roomNo: '303',
  status: 'occupied',
  roomTypeId: 1,
  roomTypeName: 'Standard',
  priceWeekday: 890,
})

const activeCheckinResponse = () => ({
  success: true,
  data: [
    {
      id: 99,
      cinNo: 'CIN-20260709-0001',
      customerName: 'Somchai Jaidee',
      checkInTime: '2026-07-08T05:00:00Z',
      expectedCheckout: '2026-07-10',
    },
  ],
})

describe('GuestMoveConfirmModal — #225 confirm dialog', () => {
  afterEach(() => {
    jest.restoreAllMocks()
  })

  it('shows from/to room, guest name, and the price implication when the type differs', async () => {
    setupFetch([activeCheckinResponse])

    const to = room({
      id: 22,
      roomNo: '404',
      roomTypeId: 2,
      roomTypeName: 'Deluxe',
      priceWeekday: 1200,
    })
    render(
      <GuestMoveConfirmModal fromRoom={FROM} toRoom={to} onClose={jest.fn()} onSuccess={jest.fn()} />,
    )

    expect(screen.getByText(/ยืนยันการย้ายห้อง 303 → 404/)).toBeInTheDocument()

    await waitFor(() => {
      expect(screen.getByText('Somchai Jaidee')).toBeInTheDocument()
    })
    expect(screen.getByText('CIN-20260709-0001')).toBeInTheDocument()
    expect(screen.getByText('303 (Standard)')).toBeInTheDocument()
    expect(screen.getByText('404 (Deluxe)')).toBeInTheDocument()

    // Room type differs → price implication block with both prices.
    expect(screen.getByText('ประเภทห้องไม่เหมือนเดิม — ราคาจะเปลี่ยน')).toBeInTheDocument()
    expect(screen.getByText('890 บาท')).toBeInTheDocument()
    expect(screen.getByText('1,200 บาท')).toBeInTheDocument()

    // Clean target → no dirty warning.
    expect(screen.queryByText(/ยังรอทำความสะอาด/)).not.toBeInTheDocument()
  })

  it('hides the price implication when the room type is the same', async () => {
    setupFetch([activeCheckinResponse])
    const to = room({ id: 23, roomNo: '304', roomTypeId: 1, priceWeekday: 890 })
    render(
      <GuestMoveConfirmModal fromRoom={FROM} toRoom={to} onClose={jest.fn()} onSuccess={jest.fn()} />,
    )
    await waitFor(() => {
      expect(screen.getByText('Somchai Jaidee')).toBeInTheDocument()
    })
    expect(
      screen.queryByText('ประเภทห้องไม่เหมือนเดิม — ราคาจะเปลี่ยน'),
    ).not.toBeInTheDocument()
  })

  it('warns when the target room is vacant-dirty', async () => {
    setupFetch([activeCheckinResponse])
    const to = room({ id: 24, roomNo: '305', roomTypeId: 1, isClean: false })
    render(
      <GuestMoveConfirmModal fromRoom={FROM} toRoom={to} onClose={jest.fn()} onSuccess={jest.fn()} />,
    )
    await waitFor(() => {
      expect(screen.getByText(/ยังรอทำความสะอาด/)).toBeInTheDocument()
    })
  })

  it('posts {fromRoomId, toRoomId, reason} to the existing change-room endpoint, then offers the slip', async () => {
    const onSuccess = jest.fn()
    const onClose = jest.fn()
    const { calls } = setupFetch([
      activeCheckinResponse,
      // POST change-room
      () => ({ success: true, rcId: 777, fromRoomId: 5, toRoomId: 22 }),
    ])

    const to = room({ id: 22, roomNo: '404', roomTypeId: 2, priceWeekday: 1200 })
    render(
      <GuestMoveConfirmModal fromRoom={FROM} toRoom={to} onClose={onClose} onSuccess={onSuccess} />,
    )

    await waitFor(() => {
      expect(screen.getByText('Somchai Jaidee')).toBeInTheDocument()
    })

    fireEvent.change(screen.getByLabelText('เหตุผล (ไม่จำเป็น)'), {
      target: { value: 'แอร์เสีย' },
    })
    fireEvent.click(screen.getByText('ยืนยันเปลี่ยนห้อง').closest('button')!)

    await waitFor(() => {
      const postCall = calls.find(
        (c) => c.init?.method === 'POST' && c.url.includes('/change-room'),
      )
      expect(postCall).toBeDefined()
      expect(postCall!.url).toContain('/api/checkins/99/change-room')
      expect(JSON.parse(postCall!.init!.body as string)).toEqual({
        fromRoomId: 5,
        toRoomId: 22,
        reason: 'แอร์เสีย',
      })
    })

    // Success view: grid refreshed, modal still open, slip print offered.
    await waitFor(() => {
      expect(screen.getByText('เปลี่ยนห้องสำเร็จ')).toBeInTheDocument()
    })
    expect(onSuccess).toHaveBeenCalledTimes(1)
    expect(onClose).not.toHaveBeenCalled()
    expect(screen.getByText('พิมพ์ใบเปลี่ยนห้อง')).toBeInTheDocument()

    fireEvent.click(screen.getByText('เสร็จสิ้น').closest('button')!)
    expect(onClose).toHaveBeenCalledTimes(1)
  })

  it('shows an error when the source room has no active check-in', async () => {
    setupFetch([() => ({ success: true, data: [] })])
    const to = room({ id: 22, roomNo: '404' })
    render(
      <GuestMoveConfirmModal fromRoom={FROM} toRoom={to} onClose={jest.fn()} onSuccess={jest.fn()} />,
    )
    await waitFor(() => {
      expect(screen.getByText('ห้องนี้ไม่มีผู้เข้าพักอยู่')).toBeInTheDocument()
    })
    // Confirm stays disabled without a folio.
    expect(screen.getByText('ยืนยันเปลี่ยนห้อง').closest('button')).toBeDisabled()
  })
})
