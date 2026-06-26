/**
 * @jest-environment jsdom
 *
 * Track G4 / T4 HIGH-3 — UI surface for the change-room flow.
 *
 * Modal contract pinned here:
 * - Looks up the active check-in for the source room on mount.
 * - Lists available rooms (excluding the source room itself).
 * - Posts `{ fromRoomId, toRoomId, reason }` to
 *   `/api/checkins/:cinId/change-room` on submit.
 * - Calls `onSuccess` + `onClose` on a 200 response.
 */

import { fireEvent, render, screen, waitFor } from '@testing-library/react'
import ChangeRoomModal from '@/components/ChangeRoomModal'

// Mock lucide-react icons so we don't pull in SVG renderers.
jest.mock('lucide-react', () => ({
  AlertCircle: () => <span data-testid="alert-icon" />,
  ArrowRightLeft: () => <span data-testid="swap-icon" />,
  Loader2: () => <span data-testid="loader-icon" />,
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

describe('ChangeRoomModal — Track G4 / T4 HIGH-3', () => {
  afterEach(() => {
    jest.restoreAllMocks()
  })

  it('renders the source room number in the title and loads the active check-in', async () => {
    setupFetch([
      // Active check-in lookup
      () => ({
        success: true,
        data: [
          {
            id: 42,
            cinNo: 'CIN-20260514-0001',
            customerName: 'Somchai Jaidee',
            checkinTime: '2026-05-13T05:00:00Z',
            expectedCheckout: '2026-05-15',
          },
        ],
      }),
      // Available rooms lookup
      () => ({
        success: true,
        data: [
          { id: 11, roomNo: '402', typeName: 'Standard' },
          { id: 12, roomNo: '403', typeName: 'Deluxe' },
        ],
      }),
    ])

    render(
      <ChangeRoomModal
        room={{ id: 5, roomNo: '303' }}
        onClose={jest.fn()}
        onSuccess={jest.fn()}
      />,
    )

    // Title shows the FROM room number.
    expect(screen.getByText(/เปลี่ยนห้องพัก ห้อง 303/)).toBeInTheDocument()

    // After data loads, the active check-in number renders.
    await waitFor(() => {
      expect(screen.getByText('CIN-20260514-0001')).toBeInTheDocument()
    })
    expect(screen.getByText('Somchai Jaidee')).toBeInTheDocument()

    // Available-room select shows both candidate rooms (and the
    // placeholder is option 0).
    const select = screen.getByLabelText('ห้องที่จะย้ายไป') as HTMLSelectElement
    const options = Array.from(select.options).map((o) => o.text)
    expect(options).toContain('402 (Standard)')
    expect(options).toContain('403 (Deluxe)')
  })

  it('posts {fromRoomId, toRoomId, reason} to /api/checkins/:id/change-room on submit', async () => {
    const onSuccess = jest.fn()
    const onClose = jest.fn()
    const { calls } = setupFetch([
      // Active check-in
      () => ({
        success: true,
        data: [
          {
            id: 99,
            cinNo: 'CIN-20260514-0007',
            customerName: 'Test Guest',
            checkinTime: '2026-05-13T05:00:00Z',
            expectedCheckout: '2026-05-16',
          },
        ],
      }),
      // Available rooms
      () => ({
        success: true,
        data: [{ id: 22, roomNo: '404', typeName: 'Suite' }],
      }),
      // POST change-room
      () => ({
        success: true,
        message: 'Room change recorded',
        checkInId: 99,
        rcId: 1234,
        fromRoomId: 5,
        toRoomId: 22,
      }),
    ])

    render(
      <ChangeRoomModal
        room={{ id: 5, roomNo: '303' }}
        onClose={onClose}
        onSuccess={onSuccess}
      />,
    )

    // Wait for the active-checkin load to complete + the select to populate.
    await waitFor(() => {
      expect(screen.getByText('CIN-20260514-0007')).toBeInTheDocument()
    })

    const select = screen.getByLabelText('ห้องที่จะย้ายไป') as HTMLSelectElement
    fireEvent.change(select, { target: { value: '22' } })
    const reasonInput = screen.getByLabelText('เหตุผล (ไม่จำเป็น)')
    fireEvent.change(reasonInput, { target: { value: 'AC broken' } })

    const submit = screen.getByText('ยืนยันเปลี่ยนห้อง').closest('button')!
    fireEvent.click(submit)

    // The POST must hit the right endpoint with the right payload.
    await waitFor(() => {
      const postCall = calls.find(
        (c) => c.init?.method === 'POST' && c.url.includes('/change-room'),
      )
      expect(postCall).toBeDefined()
      expect(postCall!.url).toContain('/api/checkins/99/change-room')
      const body = JSON.parse(postCall!.init!.body as string)
      expect(body).toEqual({
        fromRoomId: 5,
        toRoomId: 22,
        reason: 'AC broken',
      })
    })
    expect(onSuccess).toHaveBeenCalledTimes(1)
    expect(onClose).toHaveBeenCalledTimes(1)
  })

  it('shows an error message when no active check-in is found for the room', async () => {
    setupFetch([
      // Empty check-in list
      () => ({ success: true, data: [] }),
      // Available rooms (still fetched in parallel)
      () => ({ success: true, data: [] }),
    ])

    render(
      <ChangeRoomModal
        room={{ id: 5, roomNo: '303' }}
        onClose={jest.fn()}
        onSuccess={jest.fn()}
      />,
    )

    await waitFor(() => {
      expect(
        screen.getByText('ห้องนี้ไม่มีผู้เข้าพักอยู่'),
      ).toBeInTheDocument()
    })
  })

  it('does not list the source room itself in the destination options', async () => {
    setupFetch([
      () => ({
        success: true,
        data: [
          {
            id: 7,
            cinNo: 'CIN-test',
            customerName: 'X',
            checkinTime: null,
            expectedCheckout: null,
          },
        ],
      }),
      () => ({
        success: true,
        data: [
          { id: 5, roomNo: '303' }, // same as source room — must be filtered
          { id: 11, roomNo: '404' },
        ],
      }),
    ])

    render(
      <ChangeRoomModal
        room={{ id: 5, roomNo: '303' }}
        onClose={jest.fn()}
        onSuccess={jest.fn()}
      />,
    )

    await waitFor(() => {
      expect(screen.getByText('CIN-test')).toBeInTheDocument()
    })

    const select = screen.getByLabelText('ห้องที่จะย้ายไป') as HTMLSelectElement
    const optionValues = Array.from(select.options).map((o) => o.value)
    // The placeholder + 404 only — source room 5 is filtered out.
    expect(optionValues).toContain('11')
    expect(optionValues).not.toContain('5')
  })
})
