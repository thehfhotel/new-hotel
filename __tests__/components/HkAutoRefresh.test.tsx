/**
 * @jest-environment jsdom
 *
 * Wave-5 R2 follow-up — the maid's screens must STAY true, not just start true.
 *
 * THE BUG. `/hk` and `/hk/rooms/[id]` fetched once on mount and never again
 * (the room screen also refetched after the maid's own POST, which only ever
 * showed her back her own write). These pages are opened from a LINE rich-menu
 * tile and the LINE WebView keeps a page alive for hours — so a maid who
 * opened the list at 07:00 was still looking at 07:00 for the rest of her
 * round, no matter what reception did in iHOTEL. CR-1 put iHOTEL's truth in
 * the RESPONSE; nothing ever asked for a second response. That is the whole of
 * "room status doesn't sync to แม่บ้าน".
 *
 * What this suite pins, on BOTH screens:
 *  1. Coming back to the page refetches (the dominant real-world path — she
 *     locks the phone between rooms, or switches to LINE chat and back).
 *  2. A visible page refetches on the poll interval (she is holding it open
 *     while reception flips a room).
 *  3. A HIDDEN page polls NOTHING — the battery/data guarantee for a handset
 *     that lives in an apron pocket.
 *  4. A burst of triggers collapses to ONE request.
 *  5. A failed BACKGROUND refresh is silent and keeps the last good screen —
 *     a maid crossing a lift lobby must not collect a red banner per minute.
 *  6. The room screen does not auto-refresh while her own report is in flight,
 *     so a poll can never paint the pre-write answer over what she just did.
 *
 * `hkFetch`/`hkFetchMe` are mocked at the module boundary, as in the sibling
 * `/hk` suites; this file is about WHEN the page fetches, not what it renders.
 */

import { act, fireEvent, render, screen, waitFor } from '@testing-library/react'

const mockHkFetch = jest.fn()
const mockHkFetchMe = jest.fn()
// The signal poll (ADR 0008) rides this same hook but is a different request
// with its own cadence gate; stubbed here so the counts below stay about the
// ROOM data these tests were written for. Its own scheduling is asserted in
// `HkRoomSignals.test.tsx`.
const mockFetchHkSignals = jest.fn()

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
  }
})

import HkRoomListPage from '@/app/hk/page'
import HkRoomPage from '@/app/hk/rooms/[roomId]/page'
import { HK_POLL_MS } from '@/app/hk/use-hk-auto-refresh'

const ROOMS = [
  { roomId: 7, roomNo: '104', floor: 1, building: null, roomClean: false, cleaning: null },
]

function jsonResponse(body: unknown, status = 200) {
  return { ok: status >= 200 && status < 300, status, json: async () => body }
}

function meResponse() {
  return jsonResponse({
    success: true,
    badge: 'Q1001',
    displayName: null,
    branches: [{ id: 'hfhotel', labelTh: 'ฮาร์เบอร์ฟร้อนท์' }],
    markDirtyEnabled: true,
    branchesUnavailableReason: null,
  })
}

/** Is this recorded `hkFetch` call a GET (a data load) rather than the maid's
 * own POST? `hkFetch(path, branch, init)` — a POST carries an `init.method`. */
function isLoad(call: unknown[]): boolean {
  const init = call[2] as { method?: string } | undefined
  return !init?.method
}

function loadCount(): number {
  return mockHkFetch.mock.calls.filter(isLoad).length
}

/** jsdom has no real visibility; drive `document.hidden` + the event by hand,
 * exactly as a WebView backgrounding would. */
function setHidden(hidden: boolean) {
  Object.defineProperty(document, 'hidden', { value: hidden, configurable: true })
  fireEvent(document, new Event('visibilitychange'))
}

/** Past the burst-collapsing window, without reaching a poll tick. */
async function settleBeyondDebounce() {
  await act(async () => {
    jest.advanceTimersByTime(10_000)
  })
}

beforeEach(() => {
  jest.useFakeTimers()
  mockHkFetch.mockReset()
  mockHkFetchMe.mockReset()
  mockFetchHkSignals.mockReset()
  mockFetchHkSignals.mockResolvedValue([])
  Object.defineProperty(document, 'hidden', { value: false, configurable: true })
  mockHkFetchMe.mockResolvedValue(meResponse())
  mockHkFetch.mockResolvedValue(jsonResponse({ success: true, data: ROOMS }))
})

afterEach(() => {
  jest.useRealTimers()
})

describe('room list (/hk) stays live', () => {
  async function renderList() {
    render(<HkRoomListPage />)
    await screen.findByText('104')
    await waitFor(() => expect(loadCount()).toBe(1))
  }

  it('refetches when the maid comes back to the page', async () => {
    await renderList()
    await settleBeyondDebounce()

    setHidden(true)
    await act(async () => {})
    setHidden(false)
    await act(async () => {})

    await waitFor(() => expect(loadCount()).toBe(2))
  })

  it('refetches on the poll interval while visible', async () => {
    await renderList()

    await act(async () => {
      jest.advanceTimersByTime(HK_POLL_MS)
    })
    await waitFor(() => expect(loadCount()).toBe(2))

    await act(async () => {
      jest.advanceTimersByTime(HK_POLL_MS)
    })
    await waitFor(() => expect(loadCount()).toBe(3))
  })

  it('issues nothing at all while the page is hidden', async () => {
    await renderList()
    await settleBeyondDebounce()
    setHidden(true)

    await act(async () => {
      jest.advanceTimersByTime(HK_POLL_MS * 5)
    })

    // The one initial load, and not a single poll from a pocketed phone.
    expect(loadCount()).toBe(1)
  })

  it('collapses a burst of triggers into one request', async () => {
    await renderList()
    await settleBeyondDebounce()

    // A WebView returning to the foreground fires both, and a tick can land in
    // the same instant.
    setHidden(false)
    fireEvent(window, new Event('focus'))
    fireEvent(window, new Event('focus'))
    await act(async () => {})

    await waitFor(() => expect(loadCount()).toBe(2))
    expect(loadCount()).toBe(2)
  })

  it('keeps the last good list, silently, when a background refresh fails', async () => {
    await renderList()
    mockHkFetch.mockRejectedValue(new Error('offline in the stairwell'))

    await act(async () => {
      jest.advanceTimersByTime(HK_POLL_MS)
    })
    await waitFor(() => expect(loadCount()).toBe(2))

    // Room still on screen, no error banner earned by a blip she cannot act on.
    expect(screen.getByText('104')).toBeInTheDocument()
    expect(screen.queryByText(/ไม่สามารถดึงข้อมูลห้องได้/)).not.toBeInTheDocument()
  })
})

describe('room screen (/hk/rooms/[id]) stays live', () => {
  async function renderRoom() {
    mockHkFetch.mockResolvedValue(
      jsonResponse({ success: true, room: ROOMS[0], events: [] })
    )
    render(<HkRoomPage />)
    await screen.findByText('ห้อง 104')
    await waitFor(() => expect(loadCount()).toBe(1))
  }

  it('refetches when the maid comes back to the room screen', async () => {
    await renderRoom()
    await settleBeyondDebounce()

    setHidden(true)
    await act(async () => {})
    setHidden(false)
    await act(async () => {})

    await waitFor(() => expect(loadCount()).toBe(2))
  })

  it('does not auto-refresh while her own report is in flight', async () => {
    await renderRoom()
    await settleBeyondDebounce()

    // A POST that never settles: the page stays in `posting` for the whole
    // test, which is exactly the window a poll must not intrude on.
    mockHkFetch.mockImplementation((...args: unknown[]) =>
      isLoad(args)
        ? Promise.resolve(jsonResponse({ success: true, room: ROOMS[0], events: [] }))
        : new Promise(() => {})
    )

    const loadsBeforePost = loadCount()
    fireEvent.click(screen.getByRole('button', { name: /เสร็จแล้ว/ }))
    await act(async () => {})

    await act(async () => {
      jest.advanceTimersByTime(HK_POLL_MS * 3)
    })

    expect(loadCount()).toBe(loadsBeforePost)
  })
})
