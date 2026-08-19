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

jest.mock('@/app/hk/hk-lib', () => {
  const actual = jest.requireActual('@/app/hk/hk-lib')
  return {
    ...actual,
    hkFetch: (...args: unknown[]) => mockHkFetch(...args),
    hkFetchMe: (...args: unknown[]) => mockHkFetchMe(...args),
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
