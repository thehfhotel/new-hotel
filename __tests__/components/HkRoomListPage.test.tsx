/**
 * @jest-environment jsdom
 *
 * Wave-5 R2a / CR-1 — the maid's ROOM LIST (`app/hk/page.tsx`).
 *
 * The room list is the screen the owner's decision is really about: it is what
 * a maid works from, and it is where the fallback note was specified to
 * appear. Two properties:
 *
 * 1. The `roomClean` dot renders whatever the server sent — the server has
 *    already merged iHOTEL over the PMS mirror, so the client must not
 *    second-guess it (and must not carry a second opinion at all).
 * 2. When the server says it fell back, the Thai note is VISIBLE and the list
 *    beneath it still renders. Stale-but-shown beats a dead screen.
 *
 * `hkFetch` / `hkFetchMe` are mocked at the module boundary; `hk-lib.test.ts`
 * owns the pure helpers and the URL construction.
 */

import { render, screen, within } from '@testing-library/react'

const mockHkFetch = jest.fn()
const mockHkFetchMe = jest.fn()
// Room signals (ADR 0008) have their own suite (`HkRoomSignals.test.tsx`);
// stubbed empty here so the chip-row assertions below stay about cleanliness.
const mockFetchHkSignals = jest.fn()

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
import { LEGACY_STATUS_STALE_NOTE } from '@/app/hk/hk-lib'
import { HK_STATUS_LABELS } from '@/lib/v2/status'

function jsonResponse(body: unknown, status = 200) {
  return { ok: status >= 200 && status < 300, status, json: async () => body }
}

const ROOMS = [
  {
    roomId: 1,
    roomNo: '104',
    floor: 1,
    building: null,
    roomClean: false,
    cleaning: null,
    occupancy: 'occupied',
  },
  {
    roomId: 2,
    roomNo: '203',
    floor: 2,
    building: null,
    roomClean: true,
    cleaning: null,
    occupancy: 'vacant',
  },
]

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

/** Render with an arbitrary `data` array and wait for the first room's card. */
async function renderRooms(rooms: Array<Record<string, unknown>>, waitForText: string) {
  mockHkFetchMe.mockResolvedValue(meResponse())
  mockHkFetch.mockResolvedValue(jsonResponse({ success: true, data: rooms }))
  render(<HkRoomListPage />)
  await screen.findByText(waitForText)
}

/** Render with a scripted `/rooms` payload and wait for the list. */
async function renderList(extra: Record<string, unknown>) {
  mockHkFetchMe.mockResolvedValue(
    jsonResponse({
      success: true,
      badge: 'Q1001',
      displayName: null,
      branches: [{ id: 'hfhotel', labelTh: 'ฮาร์เบอร์ฟร้อนท์' }],
      markDirtyEnabled: true,
      branchesUnavailableReason: null,
    })
  )
  mockHkFetch.mockResolvedValue(jsonResponse({ success: true, data: ROOMS, ...extra }))
  render(<HkRoomListPage />)
  await screen.findByText('104')
}

beforeEach(() => {
  jest.clearAllMocks()
  localStorage.clear()
  mockFetchHkSignals.mockResolvedValue([])
})

it('shows the Thai note when the backend fell back to the PMS mirror', async () => {
  await renderList({ legacyStatusStale: true })
  expect(screen.getByText(LEGACY_STATUS_STALE_NOTE)).toBeInTheDocument()
  // The whole point of the fallback: the list is still there under the note.
  expect(screen.getByText('203')).toBeInTheDocument()
})

it('shows no note when iHOTEL answered', async () => {
  await renderList({ legacyStatusStale: false })
  expect(screen.queryByText(LEGACY_STATUS_STALE_NOTE)).not.toBeInTheDocument()
  expect(screen.getByText('203')).toBeInTheDocument()
})

// A rollback (or an older backend) omits the field. Reading silence as "stale"
// would paint a permanent banner over a healthy list.
it('shows no note when the backend omits the flag entirely', async () => {
  await renderList({})
  expect(screen.queryByText(LEGACY_STATUS_STALE_NOTE)).not.toBeInTheDocument()
})

// The dirty chip is driven purely by the server-merged `roomClean`. A client
// that re-derived it from anything else would put the maid back on a second
// opinion — exactly what CR-1 removes.
it('marks only the rooms the server reported unclean', async () => {
  await renderList({ legacyStatusStale: false })
  const dirtyCard = screen.getByText('104').closest('li') as HTMLElement
  const cleanCard = screen.getByText('203').closest('li') as HTMLElement
  expect(within(dirtyCard).getByText(HK_STATUS_LABELS.dirty)).toBeInTheDocument()
  expect(within(cleanCard).queryByText(HK_STATUS_LABELS.dirty)).not.toBeInTheDocument()
})

// ---------------------------------------------------------------------------
// Explicit clean/dirty chip (owner feedback, wave-5): "I don't see status
// from iHOTEL at แม่บ้าน" — a clean room used to show NOTHING and a dirty one
// only the small dot above. Every room now carries a labelled chip, in the
// SAME words reception reads at /v2/housekeeping (lib/v2/status.ts).
// ---------------------------------------------------------------------------

it('gives a CLEAN room an explicit สะอาด chip, not just the absence of a marker', async () => {
  await renderList({ legacyStatusStale: false })
  const cleanCard = screen.getByText('203').closest('li') as HTMLElement
  expect(within(cleanCard).getByText(HK_STATUS_LABELS.clean)).toBeInTheDocument()
  // The clean room must NOT also carry the dirty chip.
  expect(within(cleanCard).queryByText(HK_STATUS_LABELS.dirty)).not.toBeInTheDocument()
})

it('gives a DIRTY room the explicit รอทำความสะอาด chip', async () => {
  await renderList({ legacyStatusStale: false })
  const dirtyCard = screen.getByText('104').closest('li') as HTMLElement
  expect(within(dirtyCard).getByText(HK_STATUS_LABELS.dirty)).toBeInTheDocument()
})

it('keeps the progress chip visible as a secondary label next to the clean/dirty chip', async () => {
  await renderList({ legacyStatusStale: false })
  // ROOMS fixture: room 104 has cleaning: null → progressLabel default,
  // "ยังไม่เริ่ม" — the classic morning state alongside รอทำความสะอาด.
  const dirtyCard = screen.getByText('104').closest('li') as HTMLElement
  expect(within(dirtyCard).getByText(HK_STATUS_LABELS.dirty)).toBeInTheDocument()
  expect(within(dirtyCard).getByText('ยังไม่เริ่ม')).toBeInTheDocument()
})

it('renders clean/dirty chips using the exact same words as reception (lib/v2/status.ts HK labels)', async () => {
  await renderList({ legacyStatusStale: false })
  expect(screen.getAllByText(HK_STATUS_LABELS.clean).length).toBeGreaterThan(0)
  expect(screen.getAllByText(HK_STATUS_LABELS.dirty).length).toBeGreaterThan(0)
})

// When the backend fell back to the PMS mirror, the chips still render — the
// canonical fallback value is a real value, not an unknown one.
it('still renders the clean/dirty chip while legacyStatusStale is true', async () => {
  await renderList({ legacyStatusStale: true })
  const dirtyCard = screen.getByText('104').closest('li') as HTMLElement
  expect(within(dirtyCard).getByText(HK_STATUS_LABELS.dirty)).toBeInTheDocument()
})

// ---------------------------------------------------------------------------
// Summary bar (owner feedback, wave-5): รอทำความสะอาด N — the number a maid
// actually plans her round by, alongside เสร็จแล้ว/กำลังทำ/ทั้งหมด.
// ---------------------------------------------------------------------------

it('shows the count of merged-dirty rooms in the summary bar', async () => {
  await renderList({ legacyStatusStale: false })
  // ROOMS fixture: room 104 dirty, room 203 clean → 1 room needs cleaning.
  const summary = screen.getByTestId('hk-summary')
  expect(within(summary).getByText('รอทำความสะอาด')).toBeInTheDocument()
  expect(within(summary).getByText('1')).toBeInTheDocument()
  // The pre-existing counters must stay put.
  expect(within(summary).getByText('เสร็จแล้ว')).toBeInTheDocument()
  expect(within(summary).getByText('กำลังทำ')).toBeInTheDocument()
  expect(within(summary).getByText('ทั้งหมด')).toBeInTheDocument()
})

// ---------------------------------------------------------------------------
// Guest occupancy indicator (header top-right slot) — replaces the old red
// dirty dot. Room number answers "where", this slot answers "can I enter",
// the chip row (unchanged, exactly two chips) answers "what work".
// ---------------------------------------------------------------------------

it('shows มีแขกพัก on an occupied room', async () => {
  await renderList({ legacyStatusStale: false })
  const occupiedCard = screen.getByText('104').closest('li') as HTMLElement
  expect(within(occupiedCard).getByText('มีแขกพัก')).toBeInTheDocument()
})

it('shows ว่าง on a vacant room', async () => {
  await renderList({ legacyStatusStale: false })
  const vacantCard = screen.getByText('203').closest('li') as HTMLElement
  expect(within(vacantCard).getByText('ว่าง')).toBeInTheDocument()
})

// Deploy skew: an older backend has not shipped `occupancy` yet. Silence must
// never be read as either state.
it('shows NOTHING for occupancy when the field is absent (deploy skew)', async () => {
  mockHkFetchMe.mockResolvedValue(
    jsonResponse({
      success: true,
      badge: 'Q1001',
      displayName: null,
      branches: [{ id: 'hfhotel', labelTh: 'ฮาร์เบอร์ฟร้อนท์' }],
      markDirtyEnabled: true,
      branchesUnavailableReason: null,
    })
  )
  mockHkFetch.mockResolvedValue(
    jsonResponse({
      success: true,
      data: [
        { roomId: 9, roomNo: '999', floor: 9, building: null, roomClean: true, cleaning: null },
      ],
    })
  )
  render(<HkRoomListPage />)
  await screen.findByText('999')
  expect(screen.queryByText('มีแขกพัก')).not.toBeInTheDocument()
  expect(screen.queryByText('ว่าง')).not.toBeInTheDocument()
})

// The red dirty dot this indicator replaced (wave-5) is gone entirely — it
// was redundant with the always-rendered dirty chip.
it('no longer renders the old red dirty-dot marker', async () => {
  await renderList({ legacyStatusStale: false })
  expect(screen.queryByTitle('ห้องยังไม่สะอาด')).not.toBeInTheDocument()
})

// ---------------------------------------------------------------------------
// Movement tags (phase 2 delta): แขกออกวันนี้ / แขกเข้าวันนี้, the row between
// the header and the chip row. Canonical-side, different axis from occupancy
// and cleanliness.
// ---------------------------------------------------------------------------

describe('movement tags row', () => {
  const baseRoom = { roomId: 1, roomNo: '104', floor: 1, building: null, roomClean: false, cleaning: null }

  it('renders both tags, departure first, for a back-to-back room', async () => {
    await renderRooms(
      [{ ...baseRoom, expectedArrival: true, expectedDeparture: true }],
      '104'
    )
    const card = screen.getByText('104').closest('li') as HTMLElement
    const departure = within(card).getByText('แขกออกวันนี้')
    const arrival = within(card).getByText('แขกเข้าวันนี้')
    expect(departure).toBeInTheDocument()
    expect(arrival).toBeInTheDocument()
    // Departure first — the day's chronology.
    expect(
      departure.compareDocumentPosition(arrival) & Node.DOCUMENT_POSITION_FOLLOWING
    ).toBeTruthy()
  })

  it('renders only the departure tag for a departure-only room', async () => {
    await renderRooms([{ ...baseRoom, expectedDeparture: true }], '104')
    const card = screen.getByText('104').closest('li') as HTMLElement
    expect(within(card).getByText('แขกออกวันนี้')).toBeInTheDocument()
    expect(within(card).queryByText('แขกเข้าวันนี้')).not.toBeInTheDocument()
  })

  it('renders only the arrival tag for an arrival-only room', async () => {
    await renderRooms([{ ...baseRoom, expectedArrival: true }], '104')
    const card = screen.getByText('104').closest('li') as HTMLElement
    expect(within(card).getByText('แขกเข้าวันนี้')).toBeInTheDocument()
    expect(within(card).queryByText('แขกออกวันนี้')).not.toBeInTheDocument()
  })

  // No placeholder row, and no tag row, when the fields are absent (skew) —
  // and the chip row must still carry exactly two chips.
  it('renders no tag row when the fields are absent, and leaves the chip row at exactly two chips', async () => {
    await renderRooms([baseRoom], '104')
    expect(screen.queryByText('แขกออกวันนี้')).not.toBeInTheDocument()
    expect(screen.queryByText('แขกเข้าวันนี้')).not.toBeInTheDocument()
    const card = screen.getByText('104').closest('li') as HTMLElement
    expect(within(card).getByText(HK_STATUS_LABELS.dirty)).toBeInTheDocument()
    expect(within(card).getByText('ยังไม่เริ่ม')).toBeInTheDocument()
  })
})

// ---------------------------------------------------------------------------
// ขาดผ้า tag — a room with at least one linen shortage reported TODAY.
//
// The reason this tag exists at all is the finished room: cleaning เสร็จแล้ว
// and still short of linen is the case the list used to hide, because a green
// "done" chip is where a maid's eye stops. So the tag must be visible ALONGSIDE
// every cleaning state, and the done+ขาดผ้า pairing is the assertion that
// actually protects the feature.
// ---------------------------------------------------------------------------

describe('ขาดผ้า tag', () => {
  const doneCleaning = {
    status: 'done',
    badge: 'Q1001',
    name: null,
    at: '2026-09-01T03:00:00.000Z',
  }
  const finishedRoom = {
    roomId: 5,
    roomNo: '301',
    floor: 3,
    building: null,
    roomClean: true,
    cleaning: doneCleaning,
  }

  // THE case: a room that looks finished but is not.
  it('renders alongside the เสร็จแล้ว chip on a finished room', async () => {
    await renderRooms([{ ...finishedRoom, linenShortageToday: true }], '301')
    const card = screen.getByText('301').closest('li') as HTMLElement
    // Both visible together — the done chip is NOT replaced or hidden.
    expect(within(card).getByText('เสร็จแล้ว')).toBeInTheDocument()
    expect(within(card).getByText('ขาดผ้า')).toBeInTheDocument()
    // And the clean/dirty chip is untouched by either.
    expect(within(card).getByText(HK_STATUS_LABELS.clean)).toBeInTheDocument()
  })

  it('renders on an unfinished room too — the tag is not tied to a cleaning state', async () => {
    await renderRooms(
      [{ ...finishedRoom, roomClean: false, cleaning: null, linenShortageToday: true }],
      '301'
    )
    const card = screen.getByText('301').closest('li') as HTMLElement
    expect(within(card).getByText('ยังไม่เริ่ม')).toBeInTheDocument()
    expect(within(card).getByText('ขาดผ้า')).toBeInTheDocument()
  })

  it('renders NO tag when the room reported no shortage today', async () => {
    await renderRooms([{ ...finishedRoom, linenShortageToday: false }], '301')
    const card = screen.getByText('301').closest('li') as HTMLElement
    expect(within(card).getByText('เสร็จแล้ว')).toBeInTheDocument()
    expect(within(card).queryByText('ขาดผ้า')).not.toBeInTheDocument()
  })

  // Deploy skew: an older backend has not shipped the field. Silence is not
  // evidence of a shortage — and must not become a tag on every room.
  it('renders NO tag when the backend omits the field entirely', async () => {
    await renderRooms([finishedRoom], '301')
    const card = screen.getByText('301').closest('li') as HTMLElement
    expect(within(card).queryByText('ขาดผ้า')).not.toBeInTheDocument()
  })

  // Per-room, not per-list: one flagged room must not tag its neighbours.
  it('tags only the rooms the server flagged', async () => {
    await renderRooms(
      [
        { ...finishedRoom, linenShortageToday: true },
        { roomId: 6, roomNo: '302', floor: 3, building: null, roomClean: true, cleaning: null },
      ],
      '301'
    )
    const flagged = screen.getByText('301').closest('li') as HTMLElement
    const plain = screen.getByText('302').closest('li') as HTMLElement
    expect(within(flagged).getByText('ขาดผ้า')).toBeInTheDocument()
    expect(within(plain).queryByText('ขาดผ้า')).not.toBeInTheDocument()
  })

  // The chip now answers "is anything OPEN", not "was anything reported today"
  // (owner request 2026-09-01). The day-scoped field survives only as the
  // skew fallback for an older backend.
  it('tags a room with an open shortage of any age', async () => {
    await renderRooms([{ ...finishedRoom, linenShortageOpen: true }], '301')
    const card = screen.getByText('301').closest('li') as HTMLElement
    expect(within(card).getByText('ขาดผ้า')).toBeInTheDocument()
  })

  // Restocked this morning: reported today, but nothing is owed. The chip must
  // clear, or เติมผ้าแล้ว would visibly do nothing.
  it('drops the tag once an open shortage is resolved, even if it was reported today', async () => {
    await renderRooms(
      [{ ...finishedRoom, linenShortageOpen: false, linenShortageToday: true }],
      '301'
    )
    const card = screen.getByText('301').closest('li') as HTMLElement
    expect(within(card).queryByText('ขาดผ้า')).not.toBeInTheDocument()
  })
})

// ---------------------------------------------------------------------------
// ขาดผ้า WORK QUEUE panel (owner request, 2026-09-01) — the one thing on this
// screen that must be impossible to miss.
//
// A shortage now survives the day rollover, so a pale chip buried in one card
// of a ~58-card two-column grid is no longer enough: the rooms that are still
// short are lifted out into a panel at the top of the list, one tappable row
// each, straight into the room screen where เติมผ้าแล้ว lives.
//
// The other half of these assertions is the skew half: with the new field
// ABSENT the screen must look EXACTLY as it did before this feature — no
// panel, chip unchanged — because an older backend has no resolve endpoint
// behind the rows the panel would offer.
// ---------------------------------------------------------------------------

describe('ขาดผ้า work-queue panel', () => {
  const openRoom = {
    roomId: 5,
    roomNo: '301',
    floor: 3,
    building: null,
    roomClean: true,
    cleaning: null,
    linenShortageOpen: true,
  }
  const plainRoom = {
    roomId: 6,
    roomNo: '302',
    floor: 3,
    building: null,
    roomClean: true,
    cleaning: null,
    linenShortageOpen: false,
  }

  /** The panel, or a failure that says it was not rendered. */
  function panel() {
    return screen.getByTestId('hk-linen-panel')
  }

  it('lists every room with an open shortage, and nothing else', async () => {
    await renderRooms([openRoom, plainRoom, { ...openRoom, roomId: 1, roomNo: '104' }], '302')
    expect(within(panel()).getByText('ขาดผ้า')).toBeInTheDocument()
    expect(within(panel()).getByText('ห้อง 104')).toBeInTheDocument()
    expect(within(panel()).getByText('ห้อง 301')).toBeInTheDocument()
    expect(within(panel()).queryByText('ห้อง 302')).not.toBeInTheDocument()
  })

  it('counts the rooms in the heading', async () => {
    await renderRooms([openRoom, plainRoom, { ...openRoom, roomId: 1, roomNo: '104' }], '302')
    expect(within(panel()).getByText('2 ห้อง')).toBeInTheDocument()
  })

  // The row IS the way to the completion button — a panel that only announced
  // the problem would leave the maid to find the room in the grid herself.
  it('links each row to that room’s screen', async () => {
    await renderRooms([openRoom, { ...openRoom, roomId: 1, roomNo: '104' }], '301')
    const links = within(panel()).getAllByRole('link')
    expect(links.map((a) => a.getAttribute('href'))).toEqual([
      '/hk/rooms/1',
      '/hk/rooms/5',
    ])
  })

  // Walking order, not server order.
  it('orders the rows by room number', async () => {
    await renderRooms(
      [
        { ...openRoom, roomId: 3, roomNo: '301' },
        { ...openRoom, roomId: 1, roomNo: '104' },
        { ...openRoom, roomId: 2, roomNo: '95' },
      ],
      '301'
    )
    expect(
      within(panel())
        .getAllByRole('link')
        .map((a) => a.textContent?.trim())
    ).toEqual(['ห้อง 95', 'ห้อง 104', 'ห้อง 301'])
  })

  // Above the floor groups: the queue is the first thing after the header, not
  // something to scroll for.
  it('renders above the floor groups', async () => {
    await renderRooms([openRoom], '301')
    const floorHeading = screen.getByText('ชั้น 3')
    expect(
      panel().compareDocumentPosition(floorHeading) & Node.DOCUMENT_POSITION_FOLLOWING
    ).toBeTruthy()
  })

  it('renders no panel when nothing is open', async () => {
    await renderRooms([plainRoom], '302')
    expect(screen.queryByTestId('hk-linen-panel')).not.toBeInTheDocument()
  })

  // DEPLOY SKEW — the assertion that protects everyone mid-rollout. An older
  // backend sends only the day-scoped field: chip yes, queue no, because there
  // is no endpoint behind the row it would offer.
  it('renders no panel when the backend sends only the day-scoped flag', async () => {
    await renderRooms([{ ...openRoom, linenShortageOpen: undefined, linenShortageToday: true }], '301')
    expect(screen.queryByTestId('hk-linen-panel')).not.toBeInTheDocument()
    const card = screen.getByText('301').closest('li') as HTMLElement
    expect(within(card).getByText('ขาดผ้า')).toBeInTheDocument()
  })

  it('renders no panel when the backend omits both fields entirely', async () => {
    await renderRooms([{ ...openRoom, linenShortageOpen: undefined }], '301')
    expect(screen.queryByTestId('hk-linen-panel')).not.toBeInTheDocument()
  })

  // The panel is a lift, not a move: the room keeps its chip in the grid, so a
  // maid scanning by floor still sees which of her rooms is short.
  it('keeps the per-room chip in the grid alongside the panel', async () => {
    await renderRooms([openRoom], '301')
    const card = screen.getByText('301').closest('li') as HTMLElement
    expect(within(card).getByText('ขาดผ้า')).toBeInTheDocument()
    expect(within(panel()).getByText('ห้อง 301')).toBeInTheDocument()
  })
})
