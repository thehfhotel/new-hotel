/**
 * @jest-environment jsdom
 *
 * ขอเช็คห้อง on the checkout + settle surface (ADR 0008, CONTEXT.md
 * §Housekeeping "Room-check").
 *
 * The checkout ritual this encodes: the guest is at the counter, the desk
 * fires ขอเช็คห้อง MANUALLY (never auto-fired — many checkouts settle without
 * an inspection), and the maid's answer comes back as either เคลียร์ (settle
 * now) or standing มีของหาย / มีของเสียหาย signals the desk has to see before
 * it takes the money.
 *
 * Every test uses a distinct room id on purpose: the panel remembers observed
 * เคลียร์ answers — and the floor under which an answer is superseded — for
 * the life of the tab (so closing and reopening the modal while the maid walks
 * up does not forget), and both memories are module-level.
 *
 * The read has TWO lists. `signals` is open+acked only, so an answered check is
 * never in it; `answeredRoomChecks` carries today's newest ANSWERED room_check
 * per room, which is what makes a เคลียร์ survive a tab reload. A payload with
 * NO `answeredRoomChecks` key at all is the older-backend skew case, and the
 * panel must then behave exactly as it did before the field existed.
 */

import { act, fireEvent, render, screen, waitFor } from '@testing-library/react'
import type { RoomSignal } from '@/components/v2/signals/signal-lib'

const branchFetchMock = jest.fn()
const useLiveRefreshMock = jest.fn()

jest.mock('@/lib/use-branch-fetch', () => ({
  useBranchFetch: () => branchFetchMock,
}))

jest.mock('@/contexts/BranchContext', () => ({
  useBranch: () => ({ branch: mockBranch, canWrite: true }),
}))

// jsdom has no EventSource; the subscription is exercised through the hook's
// own call signature, and its refresh callback is the live path we fire.
jest.mock('@/lib/v2/use-live-refresh', () => ({
  useLiveRefresh: (...args: unknown[]) => useLiveRefreshMock(...args),
}))

import RoomCheckPanel, {
  CLEAR_TEXT,
  PENDING_TEXT,
  PROBLEMS_TEXT,
  REQUEST_LABEL,
} from '@/components/v2/signals/RoomCheckPanel'

let mockBranch = 'hfhotel'

/** The list `GET /api/housekeeping/signals` currently answers with. */
let currentSignals: RoomSignal[] = []
/** Today's answered room-checks the same read carries. `null` means the key is
 *  OMITTED from the payload entirely — the older-backend skew case, which is a
 *  different thing from an empty array. */
let currentAnswered: RoomSignal[] | null = null
/** Every request the panel made, in order. */
let calls: { url: string; init?: RequestInit }[] = []

function signal(partial: Partial<RoomSignal> & { signalId: number }): RoomSignal {
  return {
    roomId: 1,
    roomNo: '101',
    direction: 'maid_to_desk',
    type: 'item_missing',
    status: 'open',
    createdBy: { badge: 'Q1001', name: 'สมหญิง' },
    createdAt: '2026-09-01T03:00:00Z',
    ...partial,
  }
}

/** An ANSWERED room_check as the read's second list serializes it. */
function answered(
  signalId: number,
  roomId: number,
  outcome: 'clear' | 'problems',
  doneAt = '2026-09-01T04:00:00Z',
): RoomSignal {
  return signal({
    signalId,
    roomId,
    direction: 'desk_to_maid',
    type: 'room_check',
    status: 'done',
    outcome,
    doneBy: { badge: 'Q1001', name: 'สมหญิง' },
    doneAt,
    doneSource: 'room_check_answer',
  })
}

function jsonResponse(body: unknown, status = 200) {
  return { ok: status >= 200 && status < 300, status, json: async () => body }
}

beforeEach(() => {
  jest.clearAllMocks()
  mockBranch = 'hfhotel'
  currentSignals = []
  currentAnswered = null
  calls = []
  useLiveRefreshMock.mockReturnValue(true)
  branchFetchMock.mockImplementation(async (url: string, init?: RequestInit) => {
    calls.push({ url, init })
    if (url.startsWith('/api/housekeeping/signals?') || url === '/api/housekeeping/signals') {
      return jsonResponse({
        success: true,
        signals: currentSignals,
        // The key is present only when the backend has the field at all.
        ...(currentAnswered ? { answeredRoomChecks: currentAnswered } : {}),
      })
    }
    if (/\/api\/housekeeping\/rooms\/\d+\/signals$/.test(url)) {
      return jsonResponse({ success: true, signal: signal({ signalId: 900 }) })
    }
    if (/\/api\/housekeeping\/signals\/\d+\/(ack|done|cancel)$/.test(url)) {
      return jsonResponse({ success: true, signal: signal({ signalId: 900 }) })
    }
    return jsonResponse({ success: false }, 404)
  })
})

/** Render and wait for the first signals read to settle. */
async function renderPanel(roomId: number, roomNo = '101') {
  render(<RoomCheckPanel roomId={roomId} roomNo={roomNo} />)
  await waitFor(() => expect(branchFetchMock).toHaveBeenCalled())
  return roomId
}

/** Push a new server list and fire the live-refresh callback the hook handed
 *  to `useLiveRefresh` — the real SSE path. */
async function pushSignals(next: RoomSignal[], nextAnswered?: RoomSignal[] | null) {
  currentSignals = next
  if (nextAnswered !== undefined) currentAnswered = nextAnswered
  const lastCall = useLiveRefreshMock.mock.calls.at(-1) as [string, string[], () => void]
  await act(async () => {
    lastCall[2]()
  })
}

const postCalls = () => calls.filter((call) => call.init?.method === 'POST')

describe('RoomCheckPanel — asking for the check', () => {
  it('offers the ขอเช็คห้อง button and fires nothing on its own', async () => {
    await renderPanel(10)
    expect(await screen.findByText(REQUEST_LABEL)).toBeInTheDocument()
    expect(postCalls()).toHaveLength(0)
  })

  it('posts the room_check code to this room’s signal endpoint', async () => {
    await renderPanel(10)
    fireEvent.click(await screen.findByText(REQUEST_LABEL))
    await waitFor(() => expect(postCalls()).toHaveLength(1))
    const [sent] = postCalls()
    expect(sent.url).toBe('/api/housekeeping/rooms/10/signals')
    expect(JSON.parse(String(sent.init?.body))).toEqual({ type: 'room_check' })
  })

  it('reads the desk endpoint, never the maid one', async () => {
    await renderPanel(10)
    fireEvent.click(await screen.findByText(REQUEST_LABEL))
    await waitFor(() => expect(postCalls()).toHaveLength(1))
    for (const call of calls) expect(call.url.startsWith('/api/hk/')).toBe(false)
  })

  it('subscribes to the signal live events for the active branch', async () => {
    await renderPanel(10)
    expect(useLiveRefreshMock).toHaveBeenCalledWith(
      'hfhotel',
      expect.arrayContaining([
        'RoomSignalRaised',
        'RoomSignalAcked',
        'RoomSignalCompleted',
        'RoomSignalCancelled',
      ]),
      expect.any(Function),
    )
  })
})

describe('RoomCheckPanel — waiting for the answer', () => {
  it('shows the pending state and withdraws the request button', async () => {
    currentSignals = [
      signal({ signalId: 1, roomId: 11, direction: 'desk_to_maid', type: 'room_check' }),
    ]
    await renderPanel(11)
    expect(await screen.findByText(PENDING_TEXT)).toBeInTheDocument()
    expect(screen.queryByText(REQUEST_LABEL)).not.toBeInTheDocument()
  })

  it('names the maid once she has acked, so the desk knows somebody is on it', async () => {
    currentSignals = [
      signal({
        signalId: 1,
        roomId: 12,
        direction: 'desk_to_maid',
        type: 'room_check',
        status: 'acked',
        ackedBy: { badge: 'Q1002', name: 'มาลี' },
      }),
    ]
    await renderPanel(12)
    expect(await screen.findByText(/รับแล้ว โดย มาลี/)).toBeInTheDocument()
  })

  it('lets the desk cancel its own still-open request', async () => {
    currentSignals = [
      signal({ signalId: 5, roomId: 13, direction: 'desk_to_maid', type: 'room_check' }),
    ]
    await renderPanel(13)
    fireEvent.click(await screen.findByText('ยกเลิก'))
    await waitFor(() => expect(postCalls()).toHaveLength(1))
    expect(postCalls()[0].url).toBe('/api/housekeeping/signals/5/cancel')
  })

  it('ignores another room’s check entirely', async () => {
    currentSignals = [
      signal({ signalId: 1, roomId: 999, direction: 'desk_to_maid', type: 'room_check' }),
    ]
    await renderPanel(14)
    expect(await screen.findByText(REQUEST_LABEL)).toBeInTheDocument()
    expect(screen.queryByText(PENDING_TEXT)).not.toBeInTheDocument()
  })
})

// Every test in this block runs with `currentAnswered = null` — the payload
// carries no `answeredRoomChecks` key, so the panel is on its module-memory
// fallback. That is the older-backend contract, pinned deliberately.
describe('RoomCheckPanel — เคลียร์ inferred from the watched transition (older backend)', () => {
  it('goes green when the tracked check resolves with no problems', async () => {
    currentSignals = [
      signal({ signalId: 6, roomId: 20, direction: 'desk_to_maid', type: 'room_check' }),
    ]
    await renderPanel(20)
    await screen.findByText(PENDING_TEXT)

    await pushSignals([]) // answered เคลียร์ → done, so it leaves the open list

    expect(await screen.findByText(CLEAR_TEXT)).toBeInTheDocument()
    expect(screen.queryByText(PENDING_TEXT)).not.toBeInTheDocument()
  })

  it('still offers ขอเช็คห้อง afterwards, so the desk can ask again', async () => {
    currentSignals = [
      signal({ signalId: 7, roomId: 21, direction: 'desk_to_maid', type: 'room_check' }),
    ]
    await renderPanel(21)
    await screen.findByText(PENDING_TEXT)
    await pushSignals([])
    expect(screen.getByText(REQUEST_LABEL)).toBeInTheDocument()
  })

  it('does NOT go green when the desk cancelled the check itself', async () => {
    currentSignals = [
      signal({ signalId: 8, roomId: 22, direction: 'desk_to_maid', type: 'room_check' }),
    ]
    await renderPanel(22)
    fireEvent.click(await screen.findByText('ยกเลิก'))
    await waitFor(() => expect(postCalls()).toHaveLength(1))
    await pushSignals([])
    expect(screen.queryByText(CLEAR_TEXT)).not.toBeInTheDocument()
    expect(screen.getByText(REQUEST_LABEL)).toBeInTheDocument()
  })

  it('shows no green state on a first open — it never invents an answer', async () => {
    await renderPanel(23)
    expect(await screen.findByText(REQUEST_LABEL)).toBeInTheDocument()
    expect(screen.queryByText(CLEAR_TEXT)).not.toBeInTheDocument()
  })

  it('keeps inferring when the field is genuinely absent, not merely empty', async () => {
    // The skew case end to end: an OLD backend answers this read, so the panel
    // has nothing but the transition it watched — and must still go green.
    currentAnswered = null
    currentSignals = [
      signal({ signalId: 9, roomId: 90, direction: 'desk_to_maid', type: 'room_check' }),
    ]
    await renderPanel(90)
    await screen.findByText(PENDING_TEXT)
    await pushSignals([], null)
    expect(await screen.findByText(CLEAR_TEXT)).toBeInTheDocument()
  })
})

describe('RoomCheckPanel — the answer read back from the server', () => {
  it('shows เคลียร์ on a first open, with no transition ever watched', async () => {
    // The reload case: this tab was never open when the maid answered.
    currentAnswered = [answered(60, 50, 'clear')]
    await renderPanel(50)
    expect(await screen.findByText(CLEAR_TEXT)).toBeInTheDocument()
    expect(screen.queryByText(PENDING_TEXT)).not.toBeInTheDocument()
  })

  it('still offers ขอเช็คห้อง alongside the read-back green', async () => {
    currentAnswered = [answered(60, 51, 'clear')]
    await renderPanel(51)
    await screen.findByText(CLEAR_TEXT)
    expect(screen.getByText(REQUEST_LABEL)).toBeInTheDocument()
  })

  it('shows the problems state for a problems answer, children or not', async () => {
    // The desk may already have completed every spawned child; the settle
    // screen must still not read as "clear".
    currentAnswered = [answered(61, 52, 'problems')]
    await renderPanel(52)
    expect(await screen.findByText(PROBLEMS_TEXT)).toBeInTheDocument()
    expect(screen.queryByText(CLEAR_TEXT)).not.toBeInTheDocument()
  })

  it('renders the standing children under a problems answer', async () => {
    currentAnswered = [answered(62, 53, 'problems')]
    currentSignals = [signal({ signalId: 63, roomId: 53, type: 'item_missing', parentId: 62 })]
    await renderPanel(53)
    expect(await screen.findByText(PROBLEMS_TEXT)).toBeInTheDocument()
    expect(screen.getByText('มีของหาย')).toBeInTheDocument()
  })

  it('lets a live check win over an earlier answer for the same room', async () => {
    currentAnswered = [answered(64, 54, 'clear')]
    currentSignals = [
      signal({ signalId: 65, roomId: 54, direction: 'desk_to_maid', type: 'room_check' }),
    ]
    await renderPanel(54)
    expect(await screen.findByText(PENDING_TEXT)).toBeInTheDocument()
    expect(screen.queryByText(CLEAR_TEXT)).not.toBeInTheDocument()
  })

  it('ignores another room’s answer', async () => {
    currentAnswered = [answered(66, 999, 'clear')]
    await renderPanel(55)
    expect(await screen.findByText(REQUEST_LABEL)).toBeInTheDocument()
    expect(screen.queryByText(CLEAR_TEXT)).not.toBeInTheDocument()
  })

  it('treats an EMPTY list as "nothing answered today", never as the skew case', async () => {
    // Same transition the fallback reads as เคลียร์ — but here the server has
    // spoken and says there is no answer, so the server wins.
    currentAnswered = []
    currentSignals = [
      signal({ signalId: 67, roomId: 56, direction: 'desk_to_maid', type: 'room_check' }),
    ]
    await renderPanel(56)
    await screen.findByText(PENDING_TEXT)
    await pushSignals([], [])
    expect(await screen.findByText(REQUEST_LABEL)).toBeInTheDocument()
    expect(screen.queryByText(CLEAR_TEXT)).not.toBeInTheDocument()
  })
})

describe('RoomCheckPanel — an answer the desk has superseded', () => {
  it('does NOT fall back to this morning’s green after cancelling a new check', async () => {
    currentAnswered = [answered(70, 57, 'clear', '2026-09-01T02:00:00Z')]
    currentSignals = [
      signal({ signalId: 71, roomId: 57, direction: 'desk_to_maid', type: 'room_check' }),
    ]
    await renderPanel(57)
    fireEvent.click(await screen.findByText('ยกเลิก'))
    await waitFor(() => expect(postCalls()).toHaveLength(1))
    // The cancelled check is never in `answeredRoomChecks`; the stale answer is.
    await pushSignals([], [answered(70, 57, 'clear', '2026-09-01T02:00:00Z')])
    expect(screen.queryByText(CLEAR_TEXT)).not.toBeInTheDocument()
    expect(screen.getByText(REQUEST_LABEL)).toBeInTheDocument()
  })

  it('drops the green the moment the desk asks again, before any answer lands', async () => {
    currentAnswered = [answered(72, 58, 'clear', '2026-09-01T02:00:00Z')]
    await renderPanel(58)
    await screen.findByText(CLEAR_TEXT)

    fireEvent.click(screen.getByText(REQUEST_LABEL))
    await waitFor(() => expect(postCalls()).toHaveLength(1))
    await waitFor(() => expect(screen.queryByText(CLEAR_TEXT)).not.toBeInTheDocument())
  })

  it('goes green again once the NEW check is the one that was answered', async () => {
    currentAnswered = [answered(73, 59, 'clear', '2026-09-01T02:00:00Z')]
    await renderPanel(59)
    await screen.findByText(CLEAR_TEXT)

    fireEvent.click(screen.getByText(REQUEST_LABEL))
    await waitFor(() => expect(postCalls()).toHaveLength(1))
    await waitFor(() => expect(screen.queryByText(CLEAR_TEXT)).not.toBeInTheDocument())

    // 900 is the id the send endpoint handed back for the fresh request.
    await pushSignals([], [answered(900, 59, 'clear', '2026-09-01T05:00:00Z')])
    expect(await screen.findByText(CLEAR_TEXT)).toBeInTheDocument()
  })
})

describe('RoomCheckPanel — the answer the desk must act on', () => {
  const problems = (roomId: number) => [
    signal({ signalId: 31, roomId, type: 'item_missing', parentId: 30 }),
    signal({ signalId: 32, roomId, type: 'item_damaged', parentId: 30, status: 'acked' }),
  ]

  it('renders both problems under a "act before settling" heading', async () => {
    currentSignals = problems(24)
    await renderPanel(24)
    expect(await screen.findByText(PROBLEMS_TEXT)).toBeInTheDocument()
    expect(screen.getByText('มีของหาย')).toBeInTheDocument()
    expect(screen.getByText('มีของเสียหาย')).toBeInTheDocument()
  })

  it('never shows เคลียร์ while a spawned problem is still standing', async () => {
    currentSignals = [
      signal({ signalId: 30, roomId: 25, direction: 'desk_to_maid', type: 'room_check' }),
    ]
    await renderPanel(25)
    await screen.findByText(PENDING_TEXT)

    // The maid answers "problems": the check completes and one standing child
    // per problem is inserted in the same transaction.
    await pushSignals(problems(25))

    expect(await screen.findByText(PROBLEMS_TEXT)).toBeInTheDocument()
    expect(screen.queryByText(CLEAR_TEXT)).not.toBeInTheDocument()
  })

  it('acks a problem through the desk endpoint', async () => {
    currentSignals = problems(26)
    await renderPanel(26)
    fireEvent.click(await screen.findByText('รับทราบ'))
    await waitFor(() => expect(postCalls()).toHaveLength(1))
    expect(postCalls()[0].url).toBe('/api/housekeeping/signals/31/ack')
  })

  it('completes a problem through the desk endpoint', async () => {
    currentSignals = [signal({ signalId: 41, roomId: 27, type: 'item_damaged' })]
    await renderPanel(27)
    fireEvent.click(await screen.findByText('เสร็จสิ้น'))
    await waitFor(() => expect(postCalls()).toHaveLength(1))
    expect(postCalls()[0].url).toBe('/api/housekeeping/signals/41/done')
  })

  it('offers รับทราบ only while the problem is still open', async () => {
    currentSignals = [signal({ signalId: 42, roomId: 28, type: 'item_missing', status: 'acked' })]
    await renderPanel(28)
    expect(await screen.findByText('เสร็จสิ้น')).toBeInTheDocument()
    expect(screen.queryByText('รับทราบ')).not.toBeInTheDocument()
  })

  it('surfaces a problem even when this tab never saw the check', async () => {
    currentSignals = [signal({ signalId: 43, roomId: 29, type: 'item_missing' })]
    await renderPanel(29)
    expect(await screen.findByText(PROBLEMS_TEXT)).toBeInTheDocument()
  })
})

describe('RoomCheckPanel — the aggregate branch view', () => {
  it('renders nothing and reads nothing when no single branch is selected', async () => {
    mockBranch = 'all'
    const { container } = render(<RoomCheckPanel roomId={40} roomNo="101" />)
    await waitFor(() => expect(container).toBeEmptyDOMElement())
    expect(branchFetchMock).not.toHaveBeenCalled()
  })
})
