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
  { roomId: 1, roomNo: '104', floor: 1, building: null, roomClean: false, cleaning: null },
  { roomId: 2, roomNo: '203', floor: 2, building: null, roomClean: true, cleaning: null },
]

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

// The dirty dot is driven purely by the server-merged `roomClean`. A client
// that re-derived it from anything else would put the maid back on a second
// opinion — exactly what CR-1 removes.
it('marks only the rooms the server reported unclean', async () => {
  await renderList({ legacyStatusStale: false })
  const dirty = screen.getAllByTitle('ห้องยังไม่สะอาด')
  expect(dirty).toHaveLength(1)
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
  // The clean room must NOT also carry the dirty dot marker.
  expect(within(cleanCard).queryByTitle('ห้องยังไม่สะอาด')).not.toBeInTheDocument()
})

it('gives a DIRTY room the explicit รอทำความสะอาด chip, alongside the existing dot marker', async () => {
  await renderList({ legacyStatusStale: false })
  const dirtyCard = screen.getByText('104').closest('li') as HTMLElement
  expect(within(dirtyCard).getByText(HK_STATUS_LABELS.dirty)).toBeInTheDocument()
  expect(within(dirtyCard).getByTitle('ห้องยังไม่สะอาด')).toBeInTheDocument()
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
