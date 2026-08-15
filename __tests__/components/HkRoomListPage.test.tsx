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

import { render, screen } from '@testing-library/react'

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
