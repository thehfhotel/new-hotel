/**
 * @jest-environment jsdom
 *
 * Room signals on the /hk surface — ADR 0008 (`docs/adr/0008-room-signals-not-
 * chat.md`) and CONTEXT.md §Housekeeping.
 *
 * What actually costs somebody something when it breaks, and is therefore
 * what this suite pins:
 *
 * 1. **ขอเช็คห้อง can never be closed by a bare tap.** Its completion is an
 *    ANSWER — เคลียร์, or มีของหาย / มีของเสียหาย — and those problem rows are
 *    what the desk charges (or does not charge) a guest for at settle time. A
 *    เสร็จสิ้น button on that row would let a maid close a checkout inspection
 *    without answering it, with a guest standing at the counter.
 * 2. **The answer body is the record.** `{outcome:'clear'}` and
 *    `{outcome:'problems', problems:[…]}` are money-adjacent: the problems
 *    array is what spawns the standing guest-accountability signals.
 * 3. **Roles do not bleed.** A maid sends only maid→desk types and acts only
 *    on desk→maid ones; a reception viewer is the exact mirror. Nobody acts on
 *    their own direction except cancel-own-while-open.
 * 4. **Live delivery never breaks the page.** The SSE stream is best-effort:
 *    when it is connected the poll stands down, when it drops the poll takes
 *    over, and a page with no EventSource at all still works.
 *
 * `hkFetch`/`hkFetchMe` and the four signal helpers are mocked at the module
 * boundary, as in the sibling /hk suites — `hk-lib.test.ts` owns the URL and
 * body construction those helpers do.
 */

import { act, fireEvent, render, screen, waitFor, within } from '@testing-library/react'

const mockHkFetch = jest.fn()
const mockHkFetchMe = jest.fn()
const mockFetchHkSignals = jest.fn()
const mockSendHkSignal = jest.fn()
const mockActOnHkSignal = jest.fn()
const mockAnswerHkRoomCheck = jest.fn()

jest.mock('next/navigation', () => ({
  useParams: () => ({ roomId: '7' }),
}))

jest.mock('@/app/hk/hk-lib', () => {
  const actual = jest.requireActual('@/app/hk/hk-lib')
  return {
    ...actual,
    hkFetch: (...args: unknown[]) => mockHkFetch(...args),
    hkFetchMe: (...args: unknown[]) => mockHkFetchMe(...args),
    fetchHkSignals: (...args: unknown[]) => mockFetchHkSignals(...args),
    sendHkSignal: (...args: unknown[]) => mockSendHkSignal(...args),
    actOnHkSignal: (...args: unknown[]) => mockActOnHkSignal(...args),
    answerHkRoomCheck: (...args: unknown[]) => mockAnswerHkRoomCheck(...args),
  }
})

import HkRoomPage from '@/app/hk/rooms/[roomId]/page'
import HkRoomListPage from '@/app/hk/page'
import { DESK_SIGNALS, MAID_SIGNALS, type RoomSignal } from '@/app/hk/hk-lib'
import { HK_POLL_MS } from '@/app/hk/use-hk-auto-refresh'

// ---------------------------------------------------------------------------
// Test doubles for the two browser APIs the live path needs. jsdom has
// NEITHER (verified: `typeof EventSource === 'undefined'`), which is itself
// part of the contract — the page must work without them.
// ---------------------------------------------------------------------------

type Listener = (event: { data: string }) => void

class FakeEventSource {
  static instances: FakeEventSource[] = []
  url: string
  closed = false
  onopen: (() => void) | null = null
  onerror: (() => void) | null = null
  private listeners: Record<string, Listener[]> = {}

  constructor(url: string) {
    this.url = url
    FakeEventSource.instances.push(this)
  }

  addEventListener(name: string, fn: Listener) {
    ;(this.listeners[name] ||= []).push(fn)
  }

  removeEventListener(name: string, fn: Listener) {
    this.listeners[name] = (this.listeners[name] ?? []).filter((f) => f !== fn)
  }

  close() {
    this.closed = true
  }

  /** Deliver one `hk_signal` frame exactly as the backend would. */
  emit(name: string, data: unknown) {
    ;(this.listeners[name] ?? []).forEach((fn) =>
      fn({ data: typeof data === 'string' ? data : JSON.stringify(data) })
    )
  }
}

/** Every oscillator the cue has started since the last reset — the observable
 * proof a sound was (or was not) played. */
const startedTones: number[] = []

class FakeAudioContext {
  currentTime = 0
  destination = {}
  resume = () => Promise.resolve()
  createGain() {
    return {
      gain: { setValueAtTime: () => {}, exponentialRampToValueAtTime: () => {} },
      connect: () => {},
    }
  }
  createOscillator() {
    return {
      type: 'sine',
      frequency: { value: 0 },
      connect: () => {},
      start: () => startedTones.push(1),
      stop: () => {},
    }
  }
}

// ---------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------

const ROOM = {
  roomId: 7,
  roomNo: '104',
  floor: 1,
  building: null,
  roomClean: false,
  cleaning: null,
  occupancy: 'occupied',
}

function jsonResponse(body: unknown, status = 200) {
  return { ok: status >= 200 && status < 300, status, json: async () => body }
}

function meResponse(overrides: Record<string, unknown> = {}) {
  return jsonResponse({
    success: true,
    badge: 'Q1001',
    displayName: null,
    branches: [{ id: 'hfhotel', labelTh: 'ฮาร์เบอร์ฟร้อนท์' }],
    markDirtyEnabled: true,
    branchesUnavailableReason: null,
    ...overrides,
  })
}

function signal(overrides: Partial<RoomSignal> = {}): RoomSignal {
  return {
    signalId: 1,
    roomId: 7,
    roomNo: '104',
    direction: 'desk_to_maid',
    type: 'priority_clean',
    status: 'open',
    createdBy: { badge: 'R900', name: 'ต้อนรับ' },
    createdAt: '2026-09-01T03:00:00.000Z',
    ...overrides,
  }
}

/** Render the room screen for a role with a scripted signal list. */
async function renderRoom(signals: RoomSignal[], meExtras: Record<string, unknown> = {}) {
  mockHkFetchMe.mockResolvedValue(meResponse(meExtras))
  mockHkFetch.mockResolvedValue(jsonResponse({ success: true, room: ROOM, events: [] }))
  mockFetchHkSignals.mockResolvedValue(signals)
  render(<HkRoomPage />)
  await screen.findByText('ห้อง 104')
  // The signals arrive on their own request; wait for the section to settle.
  await waitFor(() => expect(mockFetchHkSignals).toHaveBeenCalled())
  return screen.getByTestId('hk-signals')
}

/** The signals section's own row for one signal. */
function row(signalId: number) {
  return screen.getByTestId(`hk-signal-${signalId}`)
}

beforeEach(() => {
  jest.clearAllMocks()
  localStorage.clear()
  startedTones.length = 0
  FakeEventSource.instances = []
  ;(globalThis as unknown as { EventSource: unknown }).EventSource = FakeEventSource
  ;(window as unknown as { AudioContext: unknown }).AudioContext = FakeAudioContext
  mockFetchHkSignals.mockResolvedValue([])
})

// ---------------------------------------------------------------------------
// The maid's side of the conversation
// ---------------------------------------------------------------------------

describe('maid — desk→maid signals', () => {
  it('lists an incoming signal with its Thai label, sender and status', async () => {
    await renderRoom([signal({ type: 'deliver_linen' })])

    const item = row(1)
    expect(within(item).getByText('แขกขอผ้าเพิ่ม')).toBeInTheDocument()
    expect(within(item).getByText(/จากแผนกต้อนรับ/)).toBeInTheDocument()
    expect(within(item).getByText(/ต้อนรับ/)).toBeInTheDocument()
    expect(within(item).getByText('รอรับเรื่อง')).toBeInTheDocument()
  })

  it('รับทราบ acks it and shows who has it', async () => {
    await renderRoom([signal()])
    mockActOnHkSignal.mockResolvedValue(
      signal({ status: 'acked', ackedBy: { badge: 'Q1001', name: 'สมศรี' } })
    )

    fireEvent.click(within(row(1)).getByRole('button', { name: 'รับทราบ' }))

    await waitFor(() => expect(mockActOnHkSignal).toHaveBeenCalledTimes(1))
    expect(mockActOnHkSignal).toHaveBeenCalledWith('hfhotel', 1, 'ack')
    // The merged response is what the row now reads from — no refetch needed.
    expect(await within(row(1)).findByText('รับเรื่องแล้ว โดย สมศรี')).toBeInTheDocument()
    // A second ack would only overwrite the name already on the row.
    expect(within(row(1)).queryByRole('button', { name: 'รับทราบ' })).not.toBeInTheDocument()
  })

  it('เสร็จสิ้น completes it and it leaves the screen', async () => {
    await renderRoom([signal()])
    mockActOnHkSignal.mockResolvedValue(signal({ status: 'done', doneSource: 'tap' }))

    fireEvent.click(within(row(1)).getByRole('button', { name: 'เสร็จสิ้น' }))

    await waitFor(() => expect(mockActOnHkSignal).toHaveBeenCalledWith('hfhotel', 1, 'done'))
    // `done` is terminal: the list is what is still somebody's work.
    await waitFor(() => expect(screen.queryByTestId('hk-signal-1')).not.toBeInTheDocument())
    expect(screen.getByText('บันทึกแล้ว: เสร็จสิ้น')).toBeInTheDocument()
  })

  // A failed action must leave the signal exactly where it was: a row that
  // vanishes on a failed tap is a request nobody is working and nobody knows.
  it('keeps the signal and shows the error when the action fails', async () => {
    await renderRoom([signal()])
    mockActOnHkSignal.mockRejectedValue(new Error('ส่งไม่สำเร็จ กรุณาลองใหม่'))

    fireEvent.click(within(row(1)).getByRole('button', { name: 'เสร็จสิ้น' }))

    expect(await screen.findByText('ส่งไม่สำเร็จ กรุณาลองใหม่')).toBeInTheDocument()
    expect(row(1)).toBeInTheDocument()
    expect(within(row(1)).getByRole('button', { name: 'เสร็จสิ้น' })).toBeInTheDocument()
  })

  it('lists only THIS room’s signals', async () => {
    await renderRoom([signal(), signal({ signalId: 2, roomId: 8, roomNo: '203' })])

    expect(row(1)).toBeInTheDocument()
    expect(screen.queryByTestId('hk-signal-2')).not.toBeInTheDocument()
  })
})

// ---------------------------------------------------------------------------
// ขอเช็คห้อง — the answer flow. The most load-bearing behaviour in the feature.
// ---------------------------------------------------------------------------

describe('maid — ขอเช็คห้อง answer flow', () => {
  const CHECK = signal({ type: 'room_check' })

  // THE regression this suite exists for: a checkout inspection may not be
  // closed without an answer, whatever its status.
  it.each(['open', 'acked'] as const)(
    'offers NO bare เสร็จสิ้น on a %s room_check',
    async (status) => {
      await renderRoom([signal({ ...CHECK, status })])

      expect(within(row(1)).queryByRole('button', { name: 'เสร็จสิ้น' })).not.toBeInTheDocument()
      expect(within(row(1)).getByRole('button', { name: 'เคลียร์' })).toBeInTheDocument()
    }
  )

  it('เคลียร์ answers with outcome clear and no problems key', async () => {
    await renderRoom([CHECK])
    mockAnswerHkRoomCheck.mockResolvedValue({
      signal: signal({ ...CHECK, status: 'done', outcome: 'clear' }),
      spawned: [],
    })

    fireEvent.click(within(row(1)).getByRole('button', { name: 'เคลียร์' }))

    await waitFor(() => expect(mockAnswerHkRoomCheck).toHaveBeenCalledTimes(1))
    expect(mockAnswerHkRoomCheck).toHaveBeenCalledWith('hfhotel', 1, { outcome: 'clear' })
    await waitFor(() => expect(screen.queryByTestId('hk-signal-1')).not.toBeInTheDocument())
  })

  it('ส่งคำตอบ is dead until a problem is toggled on', async () => {
    await renderRoom([CHECK])

    expect(within(row(1)).getByRole('button', { name: 'ส่งคำตอบ' })).toBeDisabled()
    fireEvent.click(within(row(1)).getByRole('button', { name: 'มีของหาย' }))
    expect(within(row(1)).getByRole('button', { name: 'ส่งคำตอบ' })).toBeEnabled()

    // ...and toggling it back off disarms it again — an empty `problems` list
    // must never reach the wire.
    fireEvent.click(within(row(1)).getByRole('button', { name: 'มีของหาย' }))
    expect(within(row(1)).getByRole('button', { name: 'ส่งคำตอบ' })).toBeDisabled()
    expect(mockAnswerHkRoomCheck).not.toHaveBeenCalled()
  })

  // The body IS the record behind a guest charge. Both problems, in vocabulary
  // order, whatever order the thumbs hit them in.
  it('POSTs both problems in ROOM_CHECK_PROBLEMS order', async () => {
    await renderRoom([CHECK])
    fireEvent.click(within(row(1)).getByRole('button', { name: 'มีของเสียหาย' }))
    fireEvent.click(within(row(1)).getByRole('button', { name: 'มีของหาย' }))

    mockAnswerHkRoomCheck.mockResolvedValue({
      signal: signal({ ...CHECK, status: 'done', outcome: 'problems' }),
      spawned: [
        signal({ signalId: 21, direction: 'maid_to_desk', type: 'item_missing', parentId: 1 }),
        signal({ signalId: 22, direction: 'maid_to_desk', type: 'item_damaged', parentId: 1 }),
      ],
    })
    fireEvent.click(within(row(1)).getByRole('button', { name: 'ส่งคำตอบ' }))

    await waitFor(() => expect(mockAnswerHkRoomCheck).toHaveBeenCalledTimes(1))
    expect(mockAnswerHkRoomCheck).toHaveBeenCalledWith('hfhotel', 1, {
      outcome: 'problems',
      problems: ['item_missing', 'item_damaged'],
    })
  })

  // The children are the guest-accountability signals the desk settles
  // against; they arrive in the same transaction and must land on screen with
  // the answered check's removal, not a refetch later.
  it('shows the spawned child signals and drops the answered check', async () => {
    await renderRoom([CHECK])
    fireEvent.click(within(row(1)).getByRole('button', { name: 'มีของหาย' }))
    mockAnswerHkRoomCheck.mockResolvedValue({
      signal: signal({ ...CHECK, status: 'done', outcome: 'problems' }),
      spawned: [
        signal({ signalId: 21, direction: 'maid_to_desk', type: 'item_missing', parentId: 1 }),
      ],
    })

    fireEvent.click(within(row(1)).getByRole('button', { name: 'ส่งคำตอบ' }))

    expect(await screen.findByTestId('hk-signal-21')).toBeInTheDocument()
    expect(screen.queryByTestId('hk-signal-1')).not.toBeInTheDocument()
    // It is HER report now: her own direction, so no ack/done — only ยกเลิก.
    const child = row(21)
    expect(within(child).getByText('มีของหาย')).toBeInTheDocument()
    expect(within(child).getByText(/ส่งถึงแผนกต้อนรับ/)).toBeInTheDocument()
    expect(within(child).queryByRole('button', { name: 'รับทราบ' })).not.toBeInTheDocument()
  })
})

// ---------------------------------------------------------------------------
// Sending, and the direction rules
// ---------------------------------------------------------------------------

describe('maid — sending maid→desk signals', () => {
  it('the first tap opens the list and sends NOTHING', async () => {
    await renderRoom([])

    fireEvent.click(screen.getByRole('button', { name: /แจ้งแผนกต้อนรับ/ }))

    await screen.findByRole('button', { name: 'ลูกค้ายังอยู่ในห้อง' })
    expect(mockSendHkSignal).not.toHaveBeenCalled()
  })

  it('offers exactly the four maid→desk types, and no desk→maid one', async () => {
    await renderRoom([])
    fireEvent.click(screen.getByRole('button', { name: /แจ้งแผนกต้อนรับ/ }))
    await screen.findByRole('button', { name: 'ลูกค้ายังอยู่ในห้อง' })

    MAID_SIGNALS.forEach(({ label }) =>
      expect(screen.getByRole('button', { name: label })).toBeInTheDocument()
    )
    // ขอเช็คห้อง / ทำห้องนี้ก่อน / … are the desk's to send, never hers.
    DESK_SIGNALS.forEach(({ label }) =>
      expect(screen.queryByRole('button', { name: label })).not.toBeInTheDocument()
    )
  })

  it('sends the chosen type on a single tap and closes the panel', async () => {
    await renderRoom([])
    fireEvent.click(screen.getByRole('button', { name: /แจ้งแผนกต้อนรับ/ }))
    await screen.findByRole('button', { name: 'พบของลืมในห้อง' })
    mockSendHkSignal.mockResolvedValue(
      signal({ signalId: 31, direction: 'maid_to_desk', type: 'found_belongings' })
    )

    fireEvent.click(screen.getByRole('button', { name: 'พบของลืมในห้อง' }))

    await waitFor(() => expect(mockSendHkSignal).toHaveBeenCalledTimes(1))
    // Direction is never sent — the server derives it from the role.
    expect(mockSendHkSignal).toHaveBeenCalledWith('hfhotel', 7, 'found_belongings')
    expect(await screen.findByTestId('hk-signal-31')).toBeInTheDocument()
    expect(screen.getByText('ส่งแล้ว: พบของลืมในห้อง')).toBeInTheDocument()
    await waitFor(() =>
      expect(screen.queryByRole('button', { name: 'ลูกค้ายังอยู่ในห้อง' })).not.toBeInTheDocument()
    )
  })

  // The mis-tap escape hatch — and its limit. Once the desk has acked it,
  // somebody is already acting on it.
  it('offers ยกเลิก on her own OPEN signal and not on an acked one', async () => {
    await renderRoom([
      signal({ signalId: 41, direction: 'maid_to_desk', type: 'item_missing' }),
      signal({
        signalId: 42,
        direction: 'maid_to_desk',
        type: 'guest_in_room',
        status: 'acked',
        ackedBy: { badge: 'R900', name: 'ต้อนรับ' },
      }),
    ])

    expect(within(row(41)).getByRole('button', { name: 'ยกเลิก' })).toBeInTheDocument()
    expect(within(row(42)).queryByRole('button', { name: 'ยกเลิก' })).not.toBeInTheDocument()

    mockActOnHkSignal.mockResolvedValue(
      signal({ signalId: 41, direction: 'maid_to_desk', status: 'cancelled' })
    )
    fireEvent.click(within(row(41)).getByRole('button', { name: 'ยกเลิก' }))

    await waitFor(() => expect(mockActOnHkSignal).toHaveBeenCalledWith('hfhotel', 41, 'cancel'))
    await waitFor(() => expect(screen.queryByTestId('hk-signal-41')).not.toBeInTheDocument())
  })

  it('never offers ack or done on her own direction', async () => {
    await renderRoom([signal({ signalId: 41, direction: 'maid_to_desk', type: 'item_missing' })])

    expect(within(row(41)).queryByRole('button', { name: 'รับทราบ' })).not.toBeInTheDocument()
    expect(within(row(41)).queryByRole('button', { name: 'เสร็จสิ้น' })).not.toBeInTheDocument()
  })

  // The maid's เสร็จแล้ว report auto-completes this room's cleaning-urgency
  // signals server-side, in the same transaction. Without the re-read she is
  // left tapping เสร็จสิ้น on requests she has already satisfied.
  it('re-reads signals after a เสร็จแล้ว cleaning report', async () => {
    await renderRoom([signal({ type: 'priority_clean' })])
    const readsBefore = mockFetchHkSignals.mock.calls.length
    mockFetchHkSignals.mockResolvedValue([])

    fireEvent.click(screen.getByRole('button', { name: 'เสร็จแล้ว' }))

    await waitFor(() =>
      expect(mockFetchHkSignals.mock.calls.length).toBe(readsBefore + 1)
    )
    await waitFor(() => expect(screen.queryByTestId('hk-signal-1')).not.toBeInTheDocument())
  })
})

// ---------------------------------------------------------------------------
// The reception viewer — the exact mirror
// ---------------------------------------------------------------------------

describe('reception viewer (canReport: false)', () => {
  const VIEWER = { canReport: false }

  it('offers exactly the five desk→maid types, and no maid→desk one', async () => {
    await renderRoom([], VIEWER)
    fireEvent.click(screen.getByRole('button', { name: /แจ้งแม่บ้าน/ }))
    await screen.findByRole('button', { name: 'ขอเช็คห้อง' })

    DESK_SIGNALS.forEach(({ label }) =>
      expect(screen.getByRole('button', { name: label })).toBeInTheDocument()
    )
    MAID_SIGNALS.filter(({ type }) => type !== 'item_missing' && type !== 'item_damaged').forEach(
      ({ label }) => expect(screen.queryByRole('button', { name: label })).not.toBeInTheDocument()
    )
  })

  it('sends a desk type on a single tap', async () => {
    await renderRoom([], VIEWER)
    fireEvent.click(screen.getByRole('button', { name: /แจ้งแม่บ้าน/ }))
    await screen.findByRole('button', { name: 'ขอเช็คห้อง' })
    mockSendHkSignal.mockResolvedValue(signal({ signalId: 51, type: 'room_check' }))

    fireEvent.click(screen.getByRole('button', { name: 'ขอเช็คห้อง' }))

    await waitFor(() => expect(mockSendHkSignal).toHaveBeenCalledWith('hfhotel', 7, 'room_check'))
  })

  it('acks and completes a maid→desk signal', async () => {
    await renderRoom(
      [signal({ signalId: 61, direction: 'maid_to_desk', type: 'item_missing' })],
      VIEWER
    )

    const item = row(61)
    expect(within(item).getByText(/จากแม่บ้าน/)).toBeInTheDocument()
    expect(within(item).getByRole('button', { name: 'รับทราบ' })).toBeInTheDocument()

    mockActOnHkSignal.mockResolvedValue(
      signal({ signalId: 61, direction: 'maid_to_desk', status: 'done' })
    )
    fireEvent.click(within(item).getByRole('button', { name: 'เสร็จสิ้น' }))

    await waitFor(() => expect(mockActOnHkSignal).toHaveBeenCalledWith('hfhotel', 61, 'done'))
  })

  // Her own ขอเช็คห้อง is hers to cancel while it is open — and NOT hers to
  // answer: the answer is the maid's judgment about the room.
  it('gets ยกเลิก but no answer flow on the desk’s own room_check', async () => {
    await renderRoom([signal({ type: 'room_check' })], VIEWER)

    expect(within(row(1)).getByRole('button', { name: 'ยกเลิก' })).toBeInTheDocument()
    expect(within(row(1)).queryByRole('button', { name: 'เคลียร์' })).not.toBeInTheDocument()
    expect(within(row(1)).queryByRole('button', { name: 'ส่งคำตอบ' })).not.toBeInTheDocument()
  })
})

// ---------------------------------------------------------------------------
// Live delivery
// ---------------------------------------------------------------------------

describe('live delivery (SSE + cue)', () => {
  it('subscribes to the branch stream under the /hk/api prefix', async () => {
    await renderRoom([])
    expect(FakeEventSource.instances).toHaveLength(1)
    expect(FakeEventSource.instances[0].url).toBe('/hk/api/events?branch=hfhotel')
  })

  it('renders an arriving signal and plays the cue', async () => {
    await renderRoom([])
    const stream = FakeEventSource.instances[0]

    await act(async () => {
      stream.emit('hk_signal', signal({ signalId: 71, type: 'room_check' }))
    })

    expect(screen.getByTestId('hk-signal-71')).toBeInTheDocument()
    expect(startedTones.length).toBeGreaterThan(0)
  })

  // A cue for one's own taps is a cue people mute — so the maid's own
  // maid→desk signal arrives silently, as does an echo of one already listed.
  it('stays silent for her own direction and for an echo', async () => {
    await renderRoom([signal({ signalId: 72 })])
    const stream = FakeEventSource.instances[0]

    await act(async () => {
      stream.emit('hk_signal', signal({ signalId: 73, direction: 'maid_to_desk' }))
      stream.emit('hk_signal', signal({ signalId: 72, status: 'acked' }))
    })

    expect(screen.getByTestId('hk-signal-73')).toBeInTheDocument()
    expect(startedTones).toHaveLength(0)
  })

  it('removes a signal completed elsewhere', async () => {
    await renderRoom([signal({ signalId: 74 })])
    const stream = FakeEventSource.instances[0]

    await act(async () => {
      stream.emit('hk_signal', signal({ signalId: 74, status: 'done', doneSource: 'clean_report' }))
    })

    expect(screen.queryByTestId('hk-signal-74')).not.toBeInTheDocument()
  })

  // One unreadable frame from a mid-deploy backend must not take the listener
  // — or the screen — down with it.
  it('survives a malformed frame', async () => {
    await renderRoom([])
    const stream = FakeEventSource.instances[0]

    await act(async () => {
      stream.emit('hk_signal', 'not json at all')
      stream.emit('hk_signal', signal({ signalId: 75 }))
    })

    expect(screen.getByTestId('hk-signal-75')).toBeInTheDocument()
    expect(screen.getByText('ห้อง 104')).toBeInTheDocument()
  })

  it('honours the mute stored by the room list', async () => {
    localStorage.setItem('hk.signalSoundMuted', '1')
    await renderRoom([])

    await act(async () => {
      FakeEventSource.instances[0].emit('hk_signal', signal({ signalId: 76 }))
    })

    expect(screen.getByTestId('hk-signal-76')).toBeInTheDocument()
    expect(startedTones).toHaveLength(0)
  })

  // The page must work with no EventSource at all (an old WebView) — the poll
  // below is the whole fallback story.
  it('renders normally when the browser has no EventSource', async () => {
    ;(globalThis as unknown as { EventSource?: unknown }).EventSource = undefined
    await renderRoom([signal({ signalId: 77 })])

    expect(screen.getByTestId('hk-signal-77')).toBeInTheDocument()
  })
})

describe('poll fallback', () => {
  beforeEach(() => {
    jest.useFakeTimers()
  })
  afterEach(() => {
    jest.useRealTimers()
  })

  it('polls signals on the existing cadence while the stream is down', async () => {
    mockHkFetchMe.mockResolvedValue(meResponse())
    mockHkFetch.mockResolvedValue(jsonResponse({ success: true, room: ROOM, events: [] }))
    render(<HkRoomPage />)
    await waitFor(() => expect(mockFetchHkSignals).toHaveBeenCalledTimes(1))

    // Never opened (or errored): the poll is the only mechanism left.
    await act(async () => {
      FakeEventSource.instances[0].onerror?.()
      jest.advanceTimersByTime(HK_POLL_MS)
    })

    await waitFor(() => expect(mockFetchHkSignals).toHaveBeenCalledTimes(2))
  })

  it('stands the poll down while the stream is connected', async () => {
    mockHkFetchMe.mockResolvedValue(meResponse())
    mockHkFetch.mockResolvedValue(jsonResponse({ success: true, room: ROOM, events: [] }))
    render(<HkRoomPage />)
    await waitFor(() => expect(mockFetchHkSignals).toHaveBeenCalledTimes(1))

    await act(async () => {
      FakeEventSource.instances[0].onopen?.()
    })
    await act(async () => {
      jest.advanceTimersByTime(HK_POLL_MS * 3)
    })

    // Live push already delivers every change; a poll on top is pure cost on a
    // phone.
    expect(mockFetchHkSignals).toHaveBeenCalledTimes(1)
  })

  // A signals read that fails must cost the chips, never the screen: the room
  // is what a maid works from and it is already rendered.
  it('keeps the screen when the signals read fails', async () => {
    mockHkFetchMe.mockResolvedValue(meResponse())
    mockHkFetch.mockResolvedValue(jsonResponse({ success: true, room: ROOM, events: [] }))
    mockFetchHkSignals.mockRejectedValue(new Error('offline in the stairwell'))
    render(<HkRoomPage />)

    expect(await screen.findByText('ห้อง 104')).toBeInTheDocument()
    expect(screen.getByTestId('hk-signals')).toBeInTheDocument()
    expect(screen.queryByText('offline in the stairwell')).not.toBeInTheDocument()
  })
})

// ---------------------------------------------------------------------------
// The room LIST chip
// ---------------------------------------------------------------------------

describe('room list — signal chip', () => {
  const ROOMS = [
    { roomId: 7, roomNo: '104', floor: 1, building: null, roomClean: false, cleaning: null },
    { roomId: 8, roomNo: '203', floor: 2, building: null, roomClean: true, cleaning: null },
  ]

  async function renderList(signals: RoomSignal[]) {
    mockHkFetchMe.mockResolvedValue(meResponse())
    mockHkFetch.mockResolvedValue(jsonResponse({ success: true, data: ROOMS }))
    mockFetchHkSignals.mockResolvedValue(signals)
    render(<HkRoomListPage />)
    await screen.findByText('104')
    await waitFor(() => expect(mockFetchHkSignals).toHaveBeenCalled())
  }

  function card(roomNo: string) {
    return screen.getByText(roomNo).closest('li') as HTMLElement
  }

  it('counts a room’s live signals on its card, and leaves other cards plain', async () => {
    await renderList([signal(), signal({ signalId: 2, type: 'room_check' })])

    expect(await within(card('104')).findByText('แจ้ง 2')).toBeInTheDocument()
    expect(within(card('203')).queryByText(/^แจ้ง /)).not.toBeInTheDocument()
  })

  it('counts both directions — the chip means "work outstanding"', async () => {
    await renderList([
      signal(),
      signal({ signalId: 2, direction: 'maid_to_desk', type: 'item_missing' }),
    ])

    expect(await within(card('104')).findByText('แจ้ง 2')).toBeInTheDocument()
  })

  it('drops the chip when the last signal completes on the stream', async () => {
    await renderList([signal()])
    expect(await within(card('104')).findByText('แจ้ง 1')).toBeInTheDocument()

    await act(async () => {
      FakeEventSource.instances[0].emit('hk_signal', signal({ status: 'done' }))
    })

    expect(within(card('104')).queryByText(/^แจ้ง /)).not.toBeInTheDocument()
  })

  it('mutes and unmutes the cue from the header', async () => {
    await renderList([])

    fireEvent.click(screen.getByRole('button', { name: 'ปิดเสียงแจ้งเตือน' }))

    await act(async () => {
      FakeEventSource.instances[0].emit('hk_signal', signal({ signalId: 81 }))
    })
    expect(startedTones).toHaveLength(0)

    fireEvent.click(screen.getByRole('button', { name: 'เปิดเสียงแจ้งเตือน' }))
    await act(async () => {
      FakeEventSource.instances[0].emit('hk_signal', signal({ signalId: 82 }))
    })
    expect(startedTones.length).toBeGreaterThan(0)
  })
})
