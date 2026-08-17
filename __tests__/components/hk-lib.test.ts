/**
 * @jest-environment jsdom
 *
 * Pure-helper tests for the maid-facing housekeeping surface (/hk).
 * Covers the display/grouping/branch logic in app/hk/hk-lib.ts — the API-shape
 * mapping is exercised end-to-end by the backend tests in
 * hotel-backend/src/routes/hk.rs. jsdom is required here (not the repo
 * default `node` test environment) because the branch helpers below touch
 * `localStorage` and `hkFetch`/`hkFetchMe` touch `fetch` via jsdom's window.
 */

import {
  branchesUnavailableMessage,
  countRoomsNeedingClean,
  groupRoomsByFloor,
  HK_API_BASE,
  hkFetch,
  hkFetchMe,
  HOUSEKEEPING_URL,
  LEGACY_STATUS_STALE_NOTE,
  legacyStatusNote,
  markDirtyConfirmMessage,
  progressLabel,
  readStoredBranch,
  resolveInitialBranch,
  roomCleanChip,
  storeBranch,
  timeLabel,
  type Branch,
  type HkBranchOption,
  type HkRoom,
} from '@/app/hk/hk-lib'
import { HK_STATUS_LABELS } from '@/lib/v2/status'

function room(overrides: Partial<HkRoom>): HkRoom {
  return {
    roomId: 1,
    roomNo: '101',
    floor: 1,
    building: null,
    roomClean: true,
    cleaning: null,
    ...overrides,
  }
}

function branchOption(id: Branch, labelTh: string): HkBranchOption {
  return { id, labelTh }
}

// jsdom's test environment provides neither a real `fetch` nor a global
// `Response` constructor, so — matching this repo's existing convention in
// __tests__/components/AuthContext.test.tsx — fetch responses are built as
// plain objects cast to `Response`, not `new Response(...)`.
function jsonResponse(status: number, body: unknown = { success: true }): Response {
  return {
    ok: status >= 200 && status < 300,
    status,
    json: () => Promise.resolve(body),
  } as unknown as Response
}

describe('progressLabel', () => {
  it('maps started/done/dirty/none to the Thai labels', () => {
    expect(progressLabel('started').label).toBe('กำลังทำความสะอาด')
    expect(progressLabel('done').label).toBe('เสร็จแล้ว')
    expect(progressLabel('dirty').label).toBe('แจ้งห้องไม่สะอาด')
    expect(progressLabel(null).label).toBe('ยังไม่เริ่ม')
    expect(progressLabel(undefined).label).toBe('ยังไม่เริ่ม')
  })

  it('assigns distinct badge styles per status', () => {
    const classes = new Set([
      progressLabel('started').className,
      progressLabel('done').className,
      progressLabel('dirty').className,
      progressLabel(null).className,
    ])
    expect(classes.size).toBe(4)
  })
})

// ---------------------------------------------------------------------------
// roomCleanChip / countRoomsNeedingClean — owner feedback (wave-5):
// "I don't see status from iHOTEL at แม่บ้าน". Every room now gets an
// explicit clean/dirty chip, using the SAME Thai words reception reads off
// lib/v2/status.ts, so the two audiences never disagree about the vocabulary.
// ---------------------------------------------------------------------------

describe('roomCleanChip', () => {
  it('labels a clean room with the exact reception vocabulary (lib/v2/status.ts)', () => {
    expect(roomCleanChip(true).label).toBe(HK_STATUS_LABELS.clean)
    expect(roomCleanChip(true).label).toBe('สะอาด')
  })

  it('labels a dirty room with the exact reception vocabulary (lib/v2/status.ts)', () => {
    expect(roomCleanChip(false).label).toBe(HK_STATUS_LABELS.dirty)
    expect(roomCleanChip(false).label).toBe('รอทำความสะอาด')
  })

  it('assigns visually distinct styles for clean vs dirty', () => {
    expect(roomCleanChip(true).className).not.toBe(roomCleanChip(false).className)
  })
})

describe('countRoomsNeedingClean', () => {
  it('counts only rooms whose merged roomClean is false', () => {
    const rooms = [
      room({ roomId: 1, roomClean: false }),
      room({ roomId: 2, roomClean: true }),
      room({ roomId: 3, roomClean: false }),
    ]
    expect(countRoomsNeedingClean(rooms)).toBe(2)
  })

  it('returns 0 when every room is clean', () => {
    expect(countRoomsNeedingClean([room({ roomClean: true })])).toBe(0)
  })

  it('returns 0 for an empty list', () => {
    expect(countRoomsNeedingClean([])).toBe(0)
  })
})

describe('groupRoomsByFloor', () => {
  it('groups by floor ascending with floorless rooms last', () => {
    const rooms = [
      room({ roomId: 1, roomNo: '301', floor: 3 }),
      room({ roomId: 2, roomNo: '101', floor: 1 }),
      room({ roomId: 3, roomNo: 'ANNEX', floor: null }),
      room({ roomId: 4, roomNo: '102', floor: 1 }),
    ]
    const groups = groupRoomsByFloor(rooms)
    expect(groups.map((g) => g.floor)).toEqual([1, 3, null])
    expect(groups[0].rooms.map((r) => r.roomNo)).toEqual(['101', '102'])
  })

  it('sorts room numbers numerically within a floor', () => {
    const rooms = [
      room({ roomId: 1, roomNo: '110', floor: 1 }),
      room({ roomId: 2, roomNo: '102', floor: 1 }),
      room({ roomId: 3, roomNo: '19', floor: 1 }),
    ]
    const [group] = groupRoomsByFloor(rooms)
    expect(group.rooms.map((r) => r.roomNo)).toEqual(['19', '102', '110'])
  })

  it('returns an empty list for no rooms', () => {
    expect(groupRoomsByFloor([])).toEqual([])
  })
})

describe('HOUSEKEEPING_URL', () => {
  it('defaults to the housekeeping ops app', () => {
    expect(HOUSEKEEPING_URL).toBe('https://housekeeping.thehfhotel.org')
  })

  it('composes the แจ้งซ่อม / เบิกของ deep links', () => {
    expect(`${HOUSEKEEPING_URL}/staff/report`).toBe(
      'https://housekeeping.thehfhotel.org/staff/report'
    )
    expect(`${HOUSEKEEPING_URL}/staff/stock`).toBe(
      'https://housekeeping.thehfhotel.org/staff/stock'
    )
  })
})

describe('timeLabel', () => {
  it('renders a parseable ISO instant as hh:mm', () => {
    // Exact rendering is locale/timezone dependent; assert shape only.
    expect(timeLabel('2026-07-09T03:15:00Z')).toMatch(/\d{2}:\d{2}/)
  })

  it('returns empty string for garbage', () => {
    expect(timeLabel('not-a-date')).toBe('')
  })
})

// ---------------------------------------------------------------------------
// Branch selection (§A3, §A4)
// ---------------------------------------------------------------------------

describe('readStoredBranch / storeBranch', () => {
  beforeEach(() => {
    window.localStorage.clear()
  })

  it('round-trips a stored branch', () => {
    storeBranch('hfville')
    expect(readStoredBranch()).toBe('hfville')
    expect(window.localStorage.getItem('hk.branch')).toBe('hfville')
  })

  it('returns null when nothing is stored', () => {
    expect(readStoredBranch()).toBeNull()
  })

  it('treats an unknown stored value as unset, never as a default', () => {
    window.localStorage.setItem('hk.branch', 'hfbogus')
    expect(readStoredBranch()).toBeNull()
  })

  it('falls back to null (never a default) when localStorage.getItem throws', () => {
    const spy = jest
      .spyOn(window.localStorage.__proto__, 'getItem')
      .mockImplementation(() => {
        throw new Error('private mode')
      })
    try {
      expect(readStoredBranch()).toBeNull()
    } finally {
      spy.mockRestore()
    }
  })

  it('storeBranch swallows a throwing localStorage.setItem (private mode) without crashing', () => {
    const spy = jest
      .spyOn(window.localStorage.__proto__, 'setItem')
      .mockImplementation(() => {
        throw new Error('private mode')
      })
    try {
      expect(() => storeBranch('hfhotel')).not.toThrow()
    } finally {
      spy.mockRestore()
    }
  })
})

describe('resolveInitialBranch', () => {
  const hfhotelOnly = [branchOption('hfhotel', 'ฮาร์เบอร์ฟร้อนท์')]
  const both = [branchOption('hfhotel', 'ฮาร์เบอร์ฟร้อนท์'), branchOption('hfville', 'วิลล์')]

  it('auto-selects the single configured branch when nothing is stored (the shipping state)', () => {
    expect(resolveInitialBranch(hfhotelOnly, null)).toBe('hfhotel')
  })

  it('uses the stored branch when multiple branches are configured and it is still valid', () => {
    expect(resolveInitialBranch(both, 'hfville')).toBe('hfville')
    expect(resolveInitialBranch(both, 'hfhotel')).toBe('hfhotel')
  })

  it('keeps a stored branch that is still the only configured one', () => {
    expect(resolveInitialBranch(hfhotelOnly, 'hfhotel')).toBe('hfhotel')
  })

  it('returns null (never a default) when multiple branches are configured and nothing valid is stored', () => {
    expect(resolveInitialBranch(both, null)).toBeNull()
    expect(resolveInitialBranch(both, 'hfhotel' as Branch)).toBe('hfhotel')
  })

  it('discards a stale stored branch no longer in the configured list', () => {
    expect(resolveInitialBranch([branchOption('hfville', 'วิลล์')], 'hfhotel')).toBeNull()
  })

  // The rollback path this ordering exists for: HK_BRANCHES goes
  // hfhotel,hfville → hfhotel while a Ville maid has 'hfville' stored. The
  // single-branch auto-select must NOT run first and silently move her to
  // HF Hotel — she gets the picker (rule 1 beats rule 3).
  it('re-asks instead of auto-selecting the last branch standing after a rollback', () => {
    expect(resolveInitialBranch(hfhotelOnly, 'hfville')).toBeNull()
  })
})

// ---------------------------------------------------------------------------
// hkFetch / hkFetchMe (§A2, §A4)
// ---------------------------------------------------------------------------

describe('hkFetch', () => {
  beforeEach(() => {
    // jsdom's test environment does not polyfill `fetch` itself, so there is
    // no existing property for `jest.spyOn` to wrap — assign a fresh mock.
    global.fetch = jest.fn().mockResolvedValue(jsonResponse(200))
  })

  afterEach(() => {
    jest.restoreAllMocks()
  })

  it('appends ?branch= for a path with no existing query string', async () => {
    await hkFetch('/rooms', 'hfville')
    expect(global.fetch).toHaveBeenCalledWith(`${HK_API_BASE}/rooms?branch=hfville`, undefined)
  })

  it('appends &branch= for a path that already has a query string', async () => {
    await hkFetch('/rooms?limit=50', 'hfhotel')
    expect(global.fetch).toHaveBeenCalledWith(
      `${HK_API_BASE}/rooms?limit=50&branch=hfhotel`,
      undefined
    )
  })

  it('passes init through unchanged', async () => {
    const init = { method: 'POST', body: '{"status":"done"}' }
    await hkFetch('/rooms/12/cleaning', 'hfhotel', init)
    expect(global.fetch).toHaveBeenCalledWith(
      `${HK_API_BASE}/rooms/12/cleaning?branch=hfhotel`,
      init
    )
  })

  it('throws WITHOUT calling fetch when branch is null — a bug must never become a wrong-hotel request', async () => {
    await expect(hkFetch('/rooms', null)).rejects.toThrow()
    expect(global.fetch).not.toHaveBeenCalled()
  })

  it('throws a Thai message on 401', async () => {
    ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(401))
    await expect(hkFetch('/rooms', 'hfhotel')).rejects.toThrow(/ยืนยันตัวตน/)
  })

  it('throws a Thai message on 403', async () => {
    ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(403))
    await expect(hkFetch('/rooms', 'hfhotel')).rejects.toThrow(/ไม่มีสิทธิ์/)
  })

  it('throws a DISTINCT Thai message on 400 (branch rejected server-side) — not the same text as 401/403', async () => {
    ;(global.fetch as jest.Mock).mockResolvedValueOnce(jsonResponse(400))
    const message400 = await hkFetch('/rooms', 'hfhotel').catch((e: Error) => e.message)

    ;(global.fetch as jest.Mock).mockResolvedValueOnce(jsonResponse(401))
    const message401 = await hkFetch('/rooms', 'hfhotel').catch((e: Error) => e.message)

    expect(message400).toMatch(/สาขา/)
    expect(message400).not.toBe(message401)
  })

  it('returns the response as-is on other statuses (e.g. 404), no throw', async () => {
    ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(404))
    const res = await hkFetch('/rooms/999', 'hfhotel')
    expect(res.status).toBe(404)
  })
})

describe('hkFetchMe', () => {
  beforeEach(() => {
    global.fetch = jest.fn().mockResolvedValue(jsonResponse(200))
  })

  afterEach(() => {
    jest.restoreAllMocks()
  })

  it('calls /me with NO ?branch= — the one /hk endpoint that must work before a branch is chosen', async () => {
    await hkFetchMe()
    expect(global.fetch).toHaveBeenCalledWith(`${HK_API_BASE}/me`, undefined)
  })

  it('still fails closed on 401', async () => {
    ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(401))
    await expect(hkFetchMe()).rejects.toThrow(/ยืนยันตัวตน/)
  })
})

// ---------------------------------------------------------------------------
// Employee-location enforcement (wave-4 §C) — the empty-branches contract
// ---------------------------------------------------------------------------

describe('branchesUnavailableMessage', () => {
  it('gives no_location an admin-facing action, not a retry', () => {
    const message = branchesUnavailableMessage('no_location')
    expect(message).toMatch(/ผู้ดูแลระบบ/)
    // Retrying cannot fix a missing/unserved location — the copy must not say
    // "try again", or a maid will stand there tapping refresh.
    expect(message).not.toMatch(/ลองใหม่/)
  })

  it('gives lookup_unavailable a RETRY action — it is a transient outage', () => {
    const message = branchesUnavailableMessage('lookup_unavailable')
    expect(message).toMatch(/ลองใหม่/)
  })

  it('renders the two reasons distinctly — collapsing them loses the action', () => {
    expect(branchesUnavailableMessage('no_location')).not.toBe(
      branchesUnavailableMessage('lookup_unavailable')
    )
  })

  it('falls back to a real actionable message for null/unknown, never a blank panel', () => {
    const fallback = branchesUnavailableMessage(null)
    expect(fallback.length).toBeGreaterThan(0)
    expect(fallback).toBe(branchesUnavailableMessage('no_location'))
    expect(branchesUnavailableMessage(undefined)).toBe(fallback)
    // A reason string from a newer backend must not blank the screen either.
    expect(
      branchesUnavailableMessage('something_new' as unknown as 'no_location')
    ).toBe(fallback)
  })
})

describe('resolveInitialBranch with an EMPTY branch list', () => {
  // The whole point of location enforcement: `/me` can now legitimately serve
  // `branches: []`. Nothing may be auto-selected out of an empty list — that
  // would be the wrong-property bug with extra steps.
  it('never auto-selects a branch when none is offered', () => {
    expect(resolveInitialBranch([], null)).toBeNull()
  })

  it('discards a stored branch that is no longer offered at all', () => {
    expect(resolveInitialBranch([], 'hfhotel')).toBeNull()
    expect(resolveInitialBranch([], 'hfville')).toBeNull()
  })
})

describe('hkFetch on 503 (location lookup unavailable)', () => {
  beforeEach(() => {
    global.fetch = jest.fn().mockResolvedValue(jsonResponse(503))
  })

  afterEach(() => {
    jest.restoreAllMocks()
  })

  it('throws a RETRY-flavoured Thai message, distinct from the 403 copy', async () => {
    await expect(hkFetch('/rooms', 'hfhotel')).rejects.toThrow(/ลองใหม่/)

    ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(403))
    const forbidden = await hkFetch('/rooms', 'hfhotel').catch((e: Error) => e.message)
    ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(503))
    const unavailable = await hkFetch('/rooms', 'hfhotel').catch((e: Error) => e.message)
    expect(unavailable).not.toBe(forbidden)
  })
})

// ---------------------------------------------------------------------------
// CR-1 — the iHOTEL-unavailable note (owner decision, wave-5 R2a)
// ---------------------------------------------------------------------------

describe('legacyStatusNote', () => {
  it('returns the Thai note only when the backend says the read fell back', () => {
    expect(legacyStatusNote(true)).toBe(LEGACY_STATUS_STALE_NOTE)
    expect(legacyStatusNote(false)).toBeNull()
  })

  // The load-bearing one. `undefined` is what an OLDER backend (or a rollback)
  // sends. Reading silence as "stale" would paint a permanent warning banner
  // over a perfectly healthy list — and a banner that is always on is a banner
  // the maid stops reading, which costs us the real one.
  it('treats a missing flag as NOT stale, never as a warning', () => {
    expect(legacyStatusNote(undefined)).toBeNull()
    expect(legacyStatusNote(null)).toBeNull()
  })

  // The copy has one job: tell her the status came from PMS, not iHOTEL, and
  // that reception's screen may differ. Pinned so a future edit cannot quietly
  // drop the part that changes her behaviour.
  it('names PMS, names iHOTEL, and warns that reception may differ', () => {
    expect(LEGACY_STATUS_STALE_NOTE).toContain('PMS')
    expect(LEGACY_STATUS_STALE_NOTE).toContain('iHOTEL')
    expect(LEGACY_STATUS_STALE_NOTE).toContain('แผนกต้อนรับ')
  })
})

describe('markDirtyConfirmMessage', () => {
  // The prompt must NAME THE ROOM: a maid in a corridor of near-identical
  // doors is confirming which room, not just the intent.
  it('names the room being flagged', () => {
    expect(markDirtyConfirmMessage('104')).toContain('104')
    expect(markDirtyConfirmMessage('203')).toContain('203')
    expect(markDirtyConfirmMessage('104')).not.toContain('203')
  })

  it('asks a yes/no question in Thai', () => {
    expect(markDirtyConfirmMessage('104')).toContain('ยืนยัน')
    expect(markDirtyConfirmMessage('104')).toContain('?')
  })
})
