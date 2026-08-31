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
  canReport,
  countRoomsNeedingClean,
  emptyLinenCounts,
  groupRoomsByFloor,
  HK_API_BASE,
  hkFetch,
  hkFetchMe,
  HOUSEKEEPING_URL,
  LEGACY_STATUS_STALE_NOTE,
  legacyStatusNote,
  LINEN_KINDS,
  linenKindLabel,
  linenShortageItems,
  linenShortageSummary,
  linenShortageTag,
  markDirtyConfirmMessage,
  movementTags,
  occupancyIndicator,
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

// ---------------------------------------------------------------------------
// occupancyIndicator — header-slot "can I enter" answer, distinct from the
// clean/dirty chips ("what work"). undefined = an older backend during
// deploy skew, which must render as nothing, never a guess.
// ---------------------------------------------------------------------------

describe('occupancyIndicator', () => {
  it('labels an occupied room มีแขกพัก in sky', () => {
    expect(occupancyIndicator('occupied')).toEqual({
      label: 'มีแขกพัก',
      className: 'text-sky-700',
    })
  })

  it('labels a vacant room ว่าง in gray', () => {
    expect(occupancyIndicator('vacant')).toEqual({
      label: 'ว่าง',
      className: 'text-gray-400',
    })
  })

  it('renders nothing for undefined (older backend, deploy skew) or null', () => {
    expect(occupancyIndicator(undefined)).toBeNull()
    expect(occupancyIndicator(null)).toBeNull()
  })
})

// ---------------------------------------------------------------------------
// movementTags — day-scoped arrival/departure tags (phase 2 delta). A
// different axis from occupancy (right now) and the clean/dirty chips (what
// work): this answers "what changes today". Canonical-side, NOT covered by
// legacyStatusStale.
// ---------------------------------------------------------------------------

describe('movementTags', () => {
  it('returns both tags, departure first, for a back-to-back room', () => {
    const tags = movementTags({ expectedArrival: true, expectedDeparture: true })
    expect(tags.map((t) => t.key)).toEqual(['departure', 'arrival'])
    expect(tags[0].label).toBe('แขกออกวันนี้')
    expect(tags[1].label).toBe('แขกเข้าวันนี้')
  })

  it('returns only the departure tag when just expectedDeparture is true', () => {
    const tags = movementTags({ expectedArrival: false, expectedDeparture: true })
    expect(tags).toHaveLength(1)
    expect(tags[0].key).toBe('departure')
  })

  it('returns only the arrival tag when just expectedArrival is true', () => {
    const tags = movementTags({ expectedArrival: true, expectedDeparture: false })
    expect(tags).toHaveLength(1)
    expect(tags[0].key).toBe('arrival')
  })

  it('returns an empty list when both flags are false', () => {
    expect(movementTags({ expectedArrival: false, expectedDeparture: false })).toEqual([])
  })

  // Old-backend skew: the fields do not exist on the wire at all yet.
  it('returns an empty list when both flags are undefined (deploy skew)', () => {
    expect(movementTags({})).toEqual([])
    expect(movementTags({ expectedArrival: undefined, expectedDeparture: undefined })).toEqual([])
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

  // The note now covers BOTH room-status columns the maid sees — cleanliness
  // AND occupancy — not cleanliness alone, since the merged PMS-mirror
  // fallback applies to both.
  it('names both cleanliness and occupancy as the affected room status', () => {
    expect(LEGACY_STATUS_STALE_NOTE).toContain('ความสะอาด')
    expect(LEGACY_STATUS_STALE_NOTE).toContain('การเข้าพัก')
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

// ---------------------------------------------------------------------------
// แจ้งขาดผ้า vocabulary and its two display helpers.
//
// LINEN_KINDS is the single source of the kinds: the wire codes, the Thai
// labels, the stepper rows, the request body's order and the totals line all
// derive from it. These tests pin the properties the rest of the surface is
// allowed to rely on — including that everything below still derives after a
// sixth kind was added, rather than needing its own edit.
// ---------------------------------------------------------------------------

describe('LINEN_KINDS', () => {
  it('leads with ผ้าปูที่นอน (bed linen largest-first)', () => {
    expect(LINEN_KINDS[0]).toEqual({ kind: 'bed_sheet', label: 'ผ้าปูที่นอน' })
  })

  it('carries the six reportable kinds, in display order', () => {
    expect(LINEN_KINDS.map((k) => k.kind)).toEqual([
      'bed_sheet',
      'pillowcase',
      'duvet_cover',
      'bath_towel',
      'face_towel',
      'foot_towel',
    ])
  })

  // A duplicated code would silently collapse two stepper rows into one
  // request row; a missing label would render a wire code to a maid.
  it('has a unique code and a Thai label for every row', () => {
    const codes = LINEN_KINDS.map((k) => k.kind)
    expect(new Set(codes).size).toBe(codes.length)
    for (const { label } of LINEN_KINDS) {
      expect(label.trim()).not.toBe('')
      expect(label).toMatch(/[ก-๙]/)
    }
  })

  // The derivation the "everything derives from LINEN_KINDS" claim rests on.
  it('drives emptyLinenCounts and linenShortageItems without a second list', () => {
    const counts = emptyLinenCounts()
    expect(Object.keys(counts).sort()).toEqual(LINEN_KINDS.map((k) => k.kind).sort())
    expect(Object.values(counts).every((v) => v === 0)).toBe(true)
    expect(linenShortageItems(counts)).toEqual([])

    // Non-zero rows ship in LINEN_KINDS order regardless of entry order.
    const filled = { ...counts, foot_towel: 1, bed_sheet: 2 }
    expect(linenShortageItems(filled)).toEqual([
      { kind: 'bed_sheet', qty: 2 },
      { kind: 'foot_towel', qty: 1 },
    ])
  })
})

describe('linenKindLabel', () => {
  it('gives the Thai label for a known code', () => {
    expect(linenKindLabel('bed_sheet')).toBe('ผ้าปูที่นอน')
    expect(linenKindLabel('bath_towel')).toBe('ผ้าเช็ดตัว')
  })

  // Server→client skew: a newer backend ships a kind this bundle predates.
  // A readable row beats a dropped one.
  it('falls back to the raw code for a kind it does not know', () => {
    expect(linenKindLabel('mattress_protector')).toBe('mattress_protector')
  })
})

describe('linenShortageTag', () => {
  it('tags a room that reported a shortage today', () => {
    const tag = linenShortageTag(room({ linenShortageToday: true }))
    expect(tag?.label).toBe('ขาดผ้า')
    // Sky-toned, matching the แจ้งขาดผ้า button that files the report — not
    // the red of a dirty room nor the emerald of a finished one.
    expect(tag?.className).toContain('sky')
  })

  // The pairing the tag exists for: a finished room can still be short of
  // linen, and the tag is independent of every cleaning state.
  it('tags a room whose cleaning is already done', () => {
    const tag = linenShortageTag(
      room({
        roomClean: true,
        cleaning: { status: 'done', badge: 'Q1', name: null, at: '2026-09-01T03:00:00.000Z' },
        linenShortageToday: true,
      })
    )
    expect(tag?.label).toBe('ขาดผ้า')
  })

  it('returns null for false and for an absent field (older backend)', () => {
    expect(linenShortageTag(room({ linenShortageToday: false }))).toBeNull()
    expect(linenShortageTag(room({}))).toBeNull()
  })
})

describe('linenShortageSummary', () => {
  it('reads as one Thai line, labels and quantities, in the delivered order', () => {
    expect(
      linenShortageSummary([
        { kind: 'pillowcase', qty: 2 },
        { kind: 'bath_towel', qty: 1 },
      ])
    ).toBe('วันนี้แจ้งขาดผ้า: ปลอกหมอน 2, ผ้าเช็ดตัว 1')
  })

  it('does not re-sort what the server ordered', () => {
    expect(
      linenShortageSummary([
        { kind: 'foot_towel', qty: 1 },
        { kind: 'bed_sheet', qty: 4 },
      ])
    ).toBe('วันนี้แจ้งขาดผ้า: ผ้าเช็ดเท้า 1, ผ้าปูที่นอน 4')
  })

  // Nothing reported and an older backend are the same on screen: no line.
  it('returns null for an empty list, null and undefined', () => {
    expect(linenShortageSummary([])).toBeNull()
    expect(linenShortageSummary(null)).toBeNull()
    expect(linenShortageSummary(undefined)).toBeNull()
  })
})

// ---------------------------------------------------------------------------
// canReport — the ONE place the viewer/maid skew rule lives. `/hk` now admits
// two grants: `housekeeping` (files reports) and `reception` (reads only).
// ---------------------------------------------------------------------------

describe('canReport', () => {
  it('is true for a maid the backend explicitly admits as a reporter', () => {
    expect(canReport({ canReport: true })).toBe(true)
  })

  it('is false for a reception-only viewer', () => {
    expect(canReport({ canReport: false })).toBe(false)
  })

  // THE skew rule, and the reason this helper exists rather than a bare
  // `me.canReport` read at each call site: before the `reception` viewer grant
  // existed, /hk only ever admitted maids, so a backend that omits the field is
  // one where every admitted identity could report. Defaulting to `false` would
  // strip the buttons from every maid on the floor for the length of a rollback.
  it('is true when the field is absent (older backend ⇒ maid)', () => {
    expect(canReport({})).toBe(true)
  })

  // The `/me` call has not answered yet, or failed outright. Same rule, same
  // reason — and nothing renders in that state anyway.
  it('is true for null and undefined', () => {
    expect(canReport(null)).toBe(true)
    expect(canReport(undefined)).toBe(true)
  })

  // Only an explicit `false` is a viewer. A newer backend that ever sent
  // something odd must not silently demote a maid.
  it('treats an explicitly undefined field the same as an absent one', () => {
    expect(canReport({ canReport: undefined })).toBe(true)
  })
})
