/**
 * @jest-environment jsdom
 *
 * The v2-native RECEPTION housekeeping screen (`app/v2/housekeeping/page.tsx`).
 *
 * Distinct from the maid surface (`/hk`) in two ways this file pins:
 *
 * 1. It groups EVERY active room by the merged `housekeeping[].hkStatus` axis,
 *    so the middle group (กำลังทำความสะอาด) is finally real — on the old v1
 *    board it could only be populated from a same-day `started` event and was
 *    permanently empty otherwise.
 * 2. It SHOWS `divergent`. The maid surface deliberately suppresses it (she
 *    can only act on one answer); reception is the desk that has to reconcile
 *    the two systems, so hiding it there would hide the whole problem.
 *
 * Plus the rollback property inherited from CR-1: the stale note appears on
 * `legacyStatusStale === true` and on nothing else. A backend that stops
 * sending the key must not paint a permanent banner.
 */

import { act, render, screen, within } from '@testing-library/react'

const branchFetchMock = jest.fn()
const useLiveRefreshMock = jest.fn()

jest.mock('@/lib/use-branch-fetch', () => ({
  useBranchFetch: () => branchFetchMock,
}))

jest.mock('@/contexts/BranchContext', () => ({
  useBranch: () => ({ branch: 'hfhotel', canWrite: true }),
}))

// jsdom has no EventSource; the live subscription is exercised through the
// hook's own call signature rather than a fake stream.
jest.mock('@/lib/v2/use-live-refresh', () => ({
  useLiveRefresh: (...args: unknown[]) => useLiveRefreshMock(...args),
}))

import V2Housekeeping, { DIVERGENT_BADGE, groupRoomsByHkStatus } from '@/app/v2/housekeeping/page'
import { LEGACY_STATUS_STALE_NOTE } from '@/app/hk/hk-lib'
import { HK_STATUS_LABELS, type HkStatus } from '@/lib/v2/status'

/** A real instant — `at` is a PG timestamptz, so the screen formats it in the
 *  browser's own local zone (no UTC override, no forced Asia/Bangkok). The
 *  expected string is derived the same way so this test is zone-agnostic;
 *  `RoomCleaningCard.test.tsx` owns the regression pin for the UTC-vs-instant
 *  divergence itself. */
const STARTED_AT = '2026-08-16T03:30:00Z'
const localTime = (iso: string) =>
  new Date(iso).toLocaleTimeString('th-TH', { hour: '2-digit', minute: '2-digit' })

/** Asserted as a literal, not imported: the Thai copy is part of the contract
 *  with reception, so a silent reword should fail this file. */
const LOAD_ERROR = 'ไม่สามารถโหลดสถานะความสะอาดได้'

const BOARD = {
  success: true,
  data: [
    { roomId: 2, roomNo: '102', status: 'started', badge: 'Q1002', name: 'สมหญิง', at: STARTED_AT },
    { roomId: 1, roomNo: '101', status: 'dirty', badge: 'Q1001', name: null, at: '2026-08-16T02:15:00Z' },
  ],
  legacyStatusStale: false,
  legacyClean: [
    { roomNo: '101', clean: false },
    { roomNo: '103', clean: true },
  ],
  housekeeping: [
    { roomNo: '101', hkStatus: 'dirty', divergent: false },
    { roomNo: '102', hkStatus: 'cleaning', divergent: false },
    { roomNo: '103', hkStatus: 'clean', divergent: true },
    { roomNo: '104', hkStatus: 'clean', divergent: false },
  ],
}

function jsonResponse(body: unknown, status = 200) {
  return { ok: status >= 200 && status < 300, status, json: async () => body }
}

/** Render with a scripted payload and wait for the board to settle. */
async function renderBoard(overrides: Record<string, unknown> = {}) {
  branchFetchMock.mockResolvedValue(jsonResponse({ ...BOARD, ...overrides }))
  render(<V2Housekeeping />)
  await screen.findByText('101')
}

/** The header strip of one group — `<h2>` plus its count badge. */
function groupHeader(hkStatusLabel: string): HTMLElement {
  return screen.getByRole('heading', { level: 2, name: hkStatusLabel }).parentElement as HTMLElement
}

beforeEach(() => {
  jest.clearAllMocks()
  useLiveRefreshMock.mockReturnValue(true)
})

describe('V2Housekeeping — the three housekeeping groups', () => {
  it('renders dirty → cleaning → clean, in that order', async () => {
    await renderBoard()
    const headings = screen.getAllByRole('heading', { level: 2 }).map((h) => h.textContent)
    expect(headings).toEqual([
      HK_STATUS_LABELS.dirty,
      HK_STATUS_LABELS.cleaning,
      HK_STATUS_LABELS.clean,
    ])
  })

  it('counts each group off housekeeping[].hkStatus', async () => {
    await renderBoard()
    expect(within(groupHeader(HK_STATUS_LABELS.dirty)).getByText('1')).toBeInTheDocument()
    expect(within(groupHeader(HK_STATUS_LABELS.cleaning)).getByText('1')).toBeInTheDocument()
    expect(within(groupHeader(HK_STATUS_LABELS.clean)).getByText('2')).toBeInTheDocument()
  })

  it('populates the กำลังทำความสะอาด group — the one the old board could never fill', async () => {
    await renderBoard()
    const cleaning = groupHeader(HK_STATUS_LABELS.cleaning).parentElement as HTMLElement
    expect(within(cleaning).getByText('102')).toBeInTheDocument()
    expect(within(cleaning).queryByText('101')).not.toBeInTheDocument()
  })

  it('lists every active room, including ones with no event reported today', async () => {
    await renderBoard()
    for (const roomNo of ['101', '102', '103', '104']) {
      expect(screen.getByText(roomNo)).toBeInTheDocument()
    }
  })

  it('shows an empty-group notice rather than dropping the section', async () => {
    await renderBoard({ housekeeping: [{ roomNo: '101', hkStatus: 'dirty', divergent: false }] })
    expect(screen.getAllByRole('heading', { level: 2 })).toHaveLength(3)
    expect(screen.getAllByText('ไม่มีห้องในกลุ่มนี้')).toHaveLength(2)
  })

  it('shows the empty state when no active rooms come back at all', async () => {
    branchFetchMock.mockResolvedValue(jsonResponse({ ...BOARD, housekeeping: [] }))
    render(<V2Housekeeping />)
    expect(await screen.findByText('ไม่มีข้อมูลห้องพัก')).toBeInTheDocument()
  })
})

describe('V2Housekeeping — who reported progress, and when', () => {
  it('shows the reporter name and the local time of the event', async () => {
    await renderBoard()
    expect(screen.getByText(`สมหญิง · ${localTime(STARTED_AT)}`)).toBeInTheDocument()
  })

  it('falls back to the badge when the IdP forwarded no name', async () => {
    await renderBoard()
    expect(screen.getByText(new RegExp('^Q1001 · '))).toBeInTheDocument()
  })

  it('shows no reporter line for a room with no event today', async () => {
    await renderBoard()
    const cleanGroup = groupHeader(HK_STATUS_LABELS.clean).parentElement as HTMLElement
    expect(within(cleanGroup).queryByText(/·/)).not.toBeInTheDocument()
  })
})

describe('V2Housekeeping — iHOTEL divergence is visible to reception', () => {
  it('badges only the divergent room', async () => {
    await renderBoard()
    expect(screen.getAllByText(DIVERGENT_BADGE)).toHaveLength(1)
    const cleanGroup = groupHeader(HK_STATUS_LABELS.clean).parentElement as HTMLElement
    expect(within(cleanGroup).getByText(DIVERGENT_BADGE)).toBeInTheDocument()
  })

  it('explains the badge so a receptionist knows which system is being shown', async () => {
    await renderBoard()
    expect(screen.getByText(/iHOTEL กับระบบของเรา.*ไม่ตรงกัน/)).toBeInTheDocument()
  })

  it('says nothing about divergence when every room agrees', async () => {
    await renderBoard({
      housekeeping: [{ roomNo: '101', hkStatus: 'dirty', divergent: false }],
    })
    expect(screen.queryByText(DIVERGENT_BADGE)).not.toBeInTheDocument()
    expect(screen.queryByText(/iHOTEL กับระบบของเรา.*ไม่ตรงกัน/)).not.toBeInTheDocument()
  })
})

describe('V2Housekeeping — the legacy-stale note (CR-1 rollback property)', () => {
  it('shows the Thai note when the backend fell back to the PMS mirror', async () => {
    await renderBoard({ legacyStatusStale: true })
    expect(screen.getByText(LEGACY_STATUS_STALE_NOTE)).toBeInTheDocument()
  })

  it('still renders the board underneath the note', async () => {
    await renderBoard({ legacyStatusStale: true })
    expect(screen.getByText('104')).toBeInTheDocument()
  })

  it('does NOT show the note when the flag is false', async () => {
    await renderBoard({ legacyStatusStale: false })
    expect(screen.queryByText(LEGACY_STATUS_STALE_NOTE)).not.toBeInTheDocument()
  })

  it('does NOT show the note when the key is absent — a rollback must not paint a banner', async () => {
    const { legacyStatusStale: _omitted, ...withoutFlag } = BOARD
    branchFetchMock.mockResolvedValue(jsonResponse(withoutFlag))
    render(<V2Housekeeping />)
    await screen.findByText('101')
    expect(screen.queryByText(LEGACY_STATUS_STALE_NOTE)).not.toBeInTheDocument()
  })
})

describe('groupRoomsByHkStatus — the grouping rule on its own', () => {
  it('orders each group the way a human reads room numbers', () => {
    const groups = groupRoomsByHkStatus([
      { roomNo: '1010', hkStatus: 'dirty', divergent: false },
      { roomNo: '102', hkStatus: 'dirty', divergent: false },
    ])
    expect(groups.dirty.map((room) => room.roomNo)).toEqual(['102', '1010'])
  })

  it('skips an hkStatus this build does not know, instead of blanking the screen', () => {
    const groups = groupRoomsByHkStatus([
      { roomNo: '101', hkStatus: 'dirty', divergent: false },
      { roomNo: '102', hkStatus: 'inspected' as HkStatus, divergent: false },
    ])
    expect(groups.dirty.map((room) => room.roomNo)).toEqual(['101'])
    expect(groups.cleaning).toEqual([])
    expect(groups.clean).toEqual([])
  })
})

describe('V2Housekeeping — plumbing', () => {
  it('reads the branch-aware cleaning endpoint', async () => {
    await renderBoard()
    expect(branchFetchMock).toHaveBeenCalledWith('/api/housekeeping/cleaning')
  })

  it('subscribes to the maid-progress live events for the active branch', async () => {
    await renderBoard()
    expect(useLiveRefreshMock).toHaveBeenCalledWith(
      'hfhotel',
      ['RoomMarkedClean', 'RoomMarkedDirty', 'RoomCleaningStarted'],
      expect.any(Function),
    )
  })

  it('is READ-ONLY — it renders no action buttons and issues no writes', async () => {
    await renderBoard()
    expect(screen.queryAllByRole('button')).toHaveLength(0)
    for (const call of branchFetchMock.mock.calls) {
      expect(call[1]).toBeUndefined()
    }
  })

  it('survives a failed fetch without crashing the screen', async () => {
    branchFetchMock.mockResolvedValue(jsonResponse({ error: 'boom' }, 500))
    render(<V2Housekeeping />)
    expect(await screen.findByText(LOAD_ERROR)).toBeInTheDocument()
    expect(screen.queryByText(LEGACY_STATUS_STALE_NOTE)).not.toBeInTheDocument()
  })

  it('survives a rejected fetch without crashing the screen', async () => {
    branchFetchMock.mockRejectedValue(new Error('offline'))
    render(<V2Housekeeping />)
    expect(await screen.findByText(LOAD_ERROR)).toBeInTheDocument()
  })

  it('keeps the last good board on screen when a later live refresh fails', async () => {
    await renderBoard()
    branchFetchMock.mockRejectedValue(new Error('offline'))

    // Fire the callback the page handed to useLiveRefresh — the real refresh path.
    const [, , onRefresh] = useLiveRefreshMock.mock.calls.at(-1) as [
      string,
      string[],
      () => void,
    ]
    await act(async () => {
      onRefresh()
    })

    expect(screen.getByText(LOAD_ERROR)).toBeInTheDocument()
    // Stale-but-shown beats a blank screen (the /hk rule).
    expect(screen.getByText('101')).toBeInTheDocument()
    expect(screen.getByText('104')).toBeInTheDocument()
  })
})
