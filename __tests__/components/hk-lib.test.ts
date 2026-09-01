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
  canFileReport,
  canReport,
  canReturnReport,
  canSubmitReport,
  canVerifyReport,
  canVerifyReports,
  clampReportQty,
  countRoomsNeedingClean,
  downscaleDimensions,
  fetchHkReport,
  fetchHkReports,
  fetchHkRoomReport,
  hkReportPhotoUrl,
  itemProblemLabel,
  ITEM_PROBLEMS,
  prefillRoomStatus,
  REPORT_ITEMS,
  REPORT_MAX_PHOTOS,
  REPORT_MAX_QTY,
  REPORT_MIN_QTY,
  reportDateLabel,
  reportExceptionDraftFrom,
  reportExceptionItems,
  reportExceptionKey,
  reportExceptionQty,
  reportItemRows,
  reportItemsConsistent,
  reportPhotoCountValid,
  reportRoomPriority,
  reportState,
  reportStateChip,
  reportStateCounts,
  returnHkReport,
  returnReasonLabel,
  RETURN_REASONS,
  ROOM_STATUS_CODES,
  roomStatusLabel,
  sortReportRooms,
  stashHkReportNotice,
  stepReportException,
  submitHkReport,
  takeHkReportNotice,
  toggleReportException,
  uploadHkReportPhoto,
  verifyHkReport,
  type HkReportRoom,
  type HkReportSummary,
  emptyLinenCounts,
  groupRoomsByFloor,
  hasOpenLinenShortage,
  HK_API_BASE,
  hkFetch,
  hkFetchMe,
  HOUSEKEEPING_URL,
  LEGACY_STATUS_STALE_NOTE,
  legacyStatusNote,
  LINEN_KINDS,
  LINEN_OPEN_CARD_TITLE,
  linenKindLabel,
  linenResolveConfirmMessage,
  linenShortageItems,
  linenShortageSummary,
  linenShortageTag,
  markDirtyConfirmMessage,
  movementTags,
  occupancyIndicator,
  openLinenCountLabel,
  openLinenRooms,
  openLinenRows,
  progressLabel,
  readStoredBranch,
  resolveHkLinenShortage,
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
// OPEN linen shortages (owner request, 2026-09-01). ขาดผ้า stopped being a
// day-scoped note and became work: a report stays OPEN until a maid taps
// เติมผ้าแล้ว, whatever day it was filed on.
//
// What is tested here is mostly the SKEW asymmetry, because it is the part a
// reader will assume is a mistake: the chip falls back to the deprecated
// day-scoped flag, and the queue does NOT. A chip is decoration on a room that
// is on screen anyway; a queue row leads to a completion button, and an older
// backend that omits `linenShortageOpen` has no endpoint behind it.
// ---------------------------------------------------------------------------

describe('linenShortageTag — driven by the OPEN flag', () => {
  it('tags a room with an open shortage, whatever day it was filed', () => {
    const tag = linenShortageTag(room({ linenShortageOpen: true }))
    expect(tag?.label).toBe('ขาดผ้า')
    expect(tag?.className).toContain('sky')
  })

  // The completion rule, from the chip's side: a room reported AND restocked
  // today is not short of linen, and the day-scoped record must not resurrect
  // the chip after a maid has cleared it.
  it('does not tag a room whose today-reported shortage is already resolved', () => {
    expect(
      linenShortageTag(room({ linenShortageOpen: false, linenShortageToday: true }))
    ).toBeNull()
  })

  // Bundle/backend skew, the ONE case the deprecated field still serves: an
  // older backend cannot answer "is anything open", and today's answer beats
  // silently claiming a room is clear.
  it('falls back to the day-scoped flag only when the open flag is absent', () => {
    expect(linenShortageTag(room({ linenShortageToday: true }))?.label).toBe('ขาดผ้า')
    expect(linenShortageTag(room({ linenShortageToday: false }))).toBeNull()
    expect(linenShortageTag(room({}))).toBeNull()
  })

  // An open shortage outlives the day, so it must outlive the cleaning state
  // too — the finished-but-still-short room is the whole reason for the chip.
  it('tags a room whose cleaning is already done', () => {
    expect(
      linenShortageTag(
        room({
          roomClean: true,
          cleaning: { status: 'done', badge: 'Q1', name: null, at: '2026-09-01T03:00:00.000Z' },
          linenShortageOpen: true,
        })
      )?.label
    ).toBe('ขาดผ้า')
  })
})

describe('hasOpenLinenShortage / openLinenRooms', () => {
  it('is true only for an explicit open flag', () => {
    expect(hasOpenLinenShortage(room({ linenShortageOpen: true }))).toBe(true)
    expect(hasOpenLinenShortage(room({ linenShortageOpen: false }))).toBe(false)
    expect(hasOpenLinenShortage(room({}))).toBe(false)
  })

  // THE skew rule for the queue, stated against the chip's: today's report is
  // not evidence that a resolvable row exists.
  it('never falls back to the deprecated day-scoped flag', () => {
    expect(hasOpenLinenShortage(room({ linenShortageToday: true }))).toBe(false)
    expect(openLinenRooms([room({ roomId: 1, linenShortageToday: true })])).toEqual([])
  })

  // Numeric-aware, like the floor grouping: the queue reads as a walking order,
  // not as the order the server happened to answer in.
  it('keeps only the open rooms, in room-number order', () => {
    const rooms = [
      room({ roomId: 3, roomNo: '301', linenShortageOpen: true }),
      room({ roomId: 1, roomNo: '104', linenShortageOpen: true }),
      room({ roomId: 2, roomNo: '203' }),
      room({ roomId: 4, roomNo: '95', linenShortageOpen: true }),
    ]
    expect(openLinenRooms(rooms).map((r) => r.roomNo)).toEqual(['95', '104', '301'])
  })

  it('is empty for a list with nothing open', () => {
    expect(openLinenRooms([room({}), room({ roomId: 2, linenShortageOpen: false })])).toEqual([])
    expect(openLinenRooms([])).toEqual([])
  })
})

describe('openLinenCountLabel', () => {
  it('spells the unit once, for the panel heading', () => {
    expect(openLinenCountLabel(1)).toBe('1 ห้อง')
    expect(openLinenCountLabel(12)).toBe('12 ห้อง')
  })
})

describe('openLinenRows', () => {
  it('resolves the Thai labels and keeps the delivered order', () => {
    expect(
      openLinenRows([
        { kind: 'pillowcase', qty: 2 },
        { kind: 'bath_towel', qty: 1 },
      ])
    ).toEqual([
      { kind: 'pillowcase', label: 'ปลอกหมอน', qty: 2 },
      { kind: 'bath_towel', label: 'ผ้าเช็ดตัว', qty: 1 },
    ])
  })

  // Server→client skew again: a kind this bundle predates renders as its raw
  // code rather than disappearing out of a maid's restock list.
  it('keeps an unknown kind as a readable row', () => {
    expect(openLinenRows([{ kind: 'mattress_protector', qty: 1 }])).toEqual([
      { kind: 'mattress_protector', label: 'mattress_protector', qty: 1 },
    ])
  })

  // Nothing open and an older backend are the same on screen: no card, and the
  // day-scoped totals line renders in its place.
  it('is empty for an empty list, null and undefined', () => {
    expect(openLinenRows([])).toEqual([])
    expect(openLinenRows(null)).toEqual([])
    expect(openLinenRows(undefined)).toEqual([])
  })
})

describe('linenResolveConfirmMessage', () => {
  // Names the ROOM, like the mark-dirty confirm: a corridor of near-identical
  // doors is the actual failure mode, not a misread intent.
  it('names the room in the confirm', () => {
    expect(linenResolveConfirmMessage('301')).toBe('ยืนยันว่าเติมผ้าให้ ห้อง 301 แล้ว?')
  })

  it('keeps the card title distinct from the day-scoped line', () => {
    expect(LINEN_OPEN_CARD_TITLE).toBe('ขาดผ้าค้างอยู่')
    expect(linenShortageSummary([{ kind: 'pillowcase', qty: 1 }])).not.toContain(
      LINEN_OPEN_CARD_TITLE
    )
  })
})

describe('resolveHkLinenShortage', () => {
  beforeEach(() => {
    global.fetch = jest.fn().mockResolvedValue(jsonResponse(200, { success: true, roomId: 7, resolved: 2 }))
  })

  afterEach(() => {
    jest.restoreAllMocks()
  })

  function lastCall() {
    const calls = (global.fetch as jest.Mock).mock.calls
    const [url, init] = calls[calls.length - 1]
    return { url: url as string, init: init as RequestInit }
  }

  // SIX path segments, one more than every sibling — the room-scoped action
  // hangs off the linen-shortage resource rather than being a second verb on
  // it. Pinned here because the backend's ville-guard matcher had to be widened
  // for exactly this shape.
  it('posts to the room-scoped resolve path with the branch', async () => {
    const body = await resolveHkLinenShortage('hfhotel', 7)
    const { url, init } = lastCall()
    expect(url).toBe(`${HK_API_BASE}/rooms/7/linen-shortage/resolve?branch=hfhotel`)
    expect(init.method).toBe('POST')
    expect(body.resolved).toBe(2)
  })

  // Nothing to choose and nothing to stamp: the room in the path IS the
  // request, and the resolver's identity is the verified Access assertion.
  it('sends no body at all', async () => {
    await resolveHkLinenShortage('hfhotel', 7)
    expect(lastCall().init.body).toBeUndefined()
  })

  // Two maids tapping the same room seconds apart, or a retry after a dropped
  // response. The second one is DONE, not broken.
  it('treats resolved: 0 as a success', async () => {
    ;(global.fetch as jest.Mock).mockResolvedValue(
      jsonResponse(200, { success: true, roomId: 7, resolved: 0 })
    )
    await expect(resolveHkLinenShortage('hfhotel', 7)).resolves.toEqual({
      success: true,
      roomId: 7,
      resolved: 0,
    })
  })

  // Same rule as the report and the signals: a green banner over a write that
  // never landed sends a maid away believing a room is stocked.
  it('throws on a 200 that carries success: false', async () => {
    ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(200, { success: false }))
    await expect(resolveHkLinenShortage('hfhotel', 7)).rejects.toThrow(/บันทึกไม่สำเร็จ/)
  })

  it('throws on a non-2xx', async () => {
    ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(500))
    await expect(resolveHkLinenShortage('hfhotel', 7)).rejects.toThrow(/บันทึกไม่สำเร็จ/)
  })

  // A viewer's POST is refused server-side whatever the UI offered, and the
  // refusal keeps the standing Thai message rather than a write-failure one.
  it('surfaces a 403 as the standing permission message', async () => {
    ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(403))
    await expect(resolveHkLinenShortage('hfhotel', 7)).rejects.toThrow(/ไม่มีสิทธิ์/)
  })

  it('refuses to fire at all without a branch', async () => {
    await expect(resolveHkLinenShortage(null, 7)).rejects.toThrow()
    expect(global.fetch).not.toHaveBeenCalled()
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

// ---------------------------------------------------------------------------
// Room signals — ADR 0008 (`docs/adr/0008-room-signals-not-chat.md`).
//
// The rendering is covered by `HkRoomSignals.test.tsx`; what lives here is the
// part that decides WHO may do WHAT (the direction rules), and the exact
// request each helper puts on the wire. Both are places where a quiet mistake
// is expensive: a maid able to close her own report, or an answer body missing
// its `problems` array, is a guest charge that never gets raised.
// ---------------------------------------------------------------------------

import {
  actingDirection,
  actOnHkSignal,
  answerHkRoomCheck,
  canActOnSignal,
  canCancelSignal,
  fetchHkSignals,
  isIncomingSignal,
  isLiveSignal,
  isRoomCheck,
  liveSignals,
  mergeSignal,
  mergeSignals,
  openSignalCount,
  readSignalSoundMuted,
  roomSignalChip,
  sendableSignals,
  sendHkSignal,
  sentDirection,
  signalActorLabel,
  signalCountsByRoom,
  signalOriginLabel,
  signalRole,
  signalsForRoom,
  signalStatusLabel,
  storeSignalSoundMuted,
  DESK_SIGNALS,
  MAID_SIGNALS,
  type RoomSignal,
} from '@/app/hk/hk-lib'

function testSignal(overrides: Partial<RoomSignal> = {}): RoomSignal {
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

describe('signal roles and directions', () => {
  it('maps the /me canReport flag onto a side of the conversation', () => {
    expect(signalRole(true)).toBe('maid')
    expect(signalRole(false)).toBe('reception')
  })

  // The whole rule in two lines: you send one direction and act on the other.
  it('sends one direction and acts on the other', () => {
    expect(sentDirection('maid')).toBe('maid_to_desk')
    expect(actingDirection('maid')).toBe('desk_to_maid')
    expect(sentDirection('reception')).toBe('desk_to_maid')
    expect(actingDirection('reception')).toBe('maid_to_desk')
  })

  it('offers each role exactly its own vocabulary', () => {
    expect(sendableSignals('maid')).toBe(MAID_SIGNALS)
    expect(sendableSignals('reception')).toBe(DESK_SIGNALS)
  })
})

describe('canActOnSignal / canCancelSignal', () => {
  it('lets a maid act on a live desk→maid signal', () => {
    expect(canActOnSignal(testSignal(), 'maid')).toBe(true)
    expect(canActOnSignal(testSignal({ status: 'acked' }), 'maid')).toBe(true)
  })

  // "Nobody acts on their own direction's signals" — the corridor-confusion
  // rule. A maid closing her own มีของหาย report would erase the desk's only
  // notice that a guest may owe for something.
  it('never lets a role act on its own direction', () => {
    const own = testSignal({ direction: 'maid_to_desk' })
    expect(canActOnSignal(own, 'maid')).toBe(false)
    expect(canActOnSignal(testSignal(), 'reception')).toBe(false)
  })

  it('never lets anyone act on a finished signal', () => {
    expect(canActOnSignal(testSignal({ status: 'done' }), 'maid')).toBe(false)
    expect(canActOnSignal(testSignal({ status: 'cancelled' }), 'maid')).toBe(false)
  })

  // Cancel is the ONE exception, and it closes the moment the other side has
  // picked the signal up: they are already walking to the room.
  it('lets the sender cancel while open, and not once acked', () => {
    expect(canCancelSignal(testSignal(), 'reception')).toBe(true)
    expect(canCancelSignal(testSignal({ status: 'acked' }), 'reception')).toBe(false)
    expect(canCancelSignal(testSignal(), 'maid')).toBe(false)
  })
})

describe('live-signal helpers', () => {
  it('counts open and acked as live, done and cancelled as gone', () => {
    expect(isLiveSignal(testSignal({ status: 'open' }))).toBe(true)
    expect(isLiveSignal(testSignal({ status: 'acked' }))).toBe(true)
    expect(isLiveSignal(testSignal({ status: 'done' }))).toBe(false)
    expect(isLiveSignal(testSignal({ status: 'cancelled' }))).toBe(false)
  })

  it('drops terminal signals from a list', () => {
    const list = [testSignal(), testSignal({ signalId: 2, status: 'done' })]
    expect(liveSignals(list).map((s) => s.signalId)).toEqual([1])
  })

  // Oldest first: the order they have to be worked, not the order they arrived
  // over a stream that may have reconnected halfway through.
  it('narrows to one room, oldest first', () => {
    const list = [
      testSignal({ signalId: 3, createdAt: '2026-09-01T05:00:00.000Z' }),
      testSignal({ signalId: 2, roomId: 8 }),
      testSignal({ signalId: 1, createdAt: '2026-09-01T03:00:00.000Z' }),
    ]
    expect(signalsForRoom(list, 7).map((s) => s.signalId)).toEqual([1, 3])
    expect(openSignalCount(list, 7)).toBe(2)
    expect(openSignalCount(list, 999)).toBe(0)
  })

  it('counts every room in one pass, both directions together', () => {
    const counts = signalCountsByRoom([
      testSignal(),
      testSignal({ signalId: 2, direction: 'maid_to_desk' }),
      testSignal({ signalId: 3, roomId: 8 }),
      testSignal({ signalId: 4, status: 'done' }),
    ])
    expect(counts.get(7)).toBe(2)
    expect(counts.get(8)).toBe(1)
    expect(counts.has(999)).toBe(false)
  })
})

describe('roomSignalChip', () => {
  it('renders nothing for a room with no live signals', () => {
    expect(roomSignalChip(0)).toBeNull()
  })

  it('names the count, and is the row’s only SOLID chip', () => {
    const chip = roomSignalChip(3)
    expect(chip?.label).toBe('แจ้ง 3')
    // Solid indigo: clear of red (dirty), emerald (done), amber (in progress),
    // sky (ขาดผ้า) and violet/orange (today's movement).
    expect(chip?.className).toContain('bg-indigo-600')
    expect(chip?.className).toContain('text-white')
  })
})

describe('signal display helpers', () => {
  it('says which side a signal came from, in the reader’s terms', () => {
    expect(signalOriginLabel(testSignal(), 'maid')).toBe('จากแผนกต้อนรับ')
    expect(signalOriginLabel(testSignal({ direction: 'maid_to_desk' }), 'maid')).toBe(
      'ส่งถึงแผนกต้อนรับ'
    )
    expect(signalOriginLabel(testSignal({ direction: 'maid_to_desk' }), 'reception')).toBe(
      'จากแม่บ้าน'
    )
    expect(signalOriginLabel(testSignal(), 'reception')).toBe('ส่งถึงแม่บ้าน')
  })

  it('names who has an acked signal, and says nobody has an open one', () => {
    expect(signalStatusLabel(testSignal())).toBe('รอรับเรื่อง')
    expect(
      signalStatusLabel(
        testSignal({ status: 'acked', ackedBy: { badge: 'Q1001', name: 'สมศรี' } })
      )
    ).toBe('รับเรื่องแล้ว โดย สมศรี')
  })

  // A nameless identity must still be identifiable — the badge is what the
  // office can look up.
  it('falls back to the badge when a name is missing', () => {
    expect(
      signalStatusLabel(testSignal({ status: 'acked', ackedBy: { badge: 'Q1001', name: null } }))
    ).toBe('รับเรื่องแล้ว โดย Q1001')
    expect(signalActorLabel({ badge: 'Q1001', name: null })).toBe('Q1001')
    expect(signalActorLabel({ badge: 'Q1001', name: 'สมศรี' })).toBe('สมศรี')
    expect(signalActorLabel(null)).toBe('')
  })

  it('recognises the one type that may not be closed by a tap', () => {
    expect(isRoomCheck(testSignal({ type: 'room_check' }))).toBe(true)
    expect(isRoomCheck(testSignal({ type: 'priority_clean' }))).toBe(false)
  })
})

describe('isIncomingSignal (the sound cue’s gate)', () => {
  it('is true only for a NEW signal pointed at this role', () => {
    expect(isIncomingSignal(testSignal(), 'maid')).toBe(true)
    // Her own report echoing back: silence.
    expect(isIncomingSignal(testSignal({ direction: 'maid_to_desk' }), 'maid')).toBe(false)
    // An ack/done echo of something already on screen: silence.
    expect(isIncomingSignal(testSignal({ status: 'acked' }), 'maid')).toBe(false)
    expect(isIncomingSignal(testSignal({ status: 'done' }), 'maid')).toBe(false)
  })
})

describe('mergeSignal', () => {
  it('adds a new signal in createdAt order', () => {
    const earlier = testSignal({ signalId: 1, createdAt: '2026-09-01T03:00:00.000Z' })
    const later = testSignal({ signalId: 2, createdAt: '2026-09-01T05:00:00.000Z' })
    expect(mergeSignal([later], earlier).map((s) => s.signalId)).toEqual([1, 2])
  })

  it('replaces a signal it already holds rather than duplicating it', () => {
    const acked = testSignal({ status: 'acked' })
    const merged = mergeSignal([testSignal()], acked)
    expect(merged).toHaveLength(1)
    expect(merged[0].status).toBe('acked')
  })

  // A stream event carrying `done` is HOW a signal leaves the other role's
  // screen. Keeping it would leave a finished room looking busy forever.
  it('removes a signal that has become terminal', () => {
    expect(mergeSignal([testSignal()], testSignal({ status: 'done' }))).toEqual([])
    expect(mergeSignal([testSignal()], testSignal({ status: 'cancelled' }))).toEqual([])
  })

  it('merges a whole batch — an answer’s children arrive together', () => {
    const merged = mergeSignals(
      [testSignal({ type: 'room_check' })],
      [
        testSignal({ type: 'room_check', status: 'done' }),
        testSignal({ signalId: 21, direction: 'maid_to_desk', type: 'item_missing' }),
        testSignal({ signalId: 22, direction: 'maid_to_desk', type: 'item_damaged' }),
      ]
    )
    expect(merged.map((s) => s.signalId)).toEqual([21, 22])
  })
})

// ---------------------------------------------------------------------------
// The signal endpoints. `hkFetch` construction itself is covered above; these
// pin the paths and the BODIES — the answer body in particular is the record a
// guest charge is raised from.
// ---------------------------------------------------------------------------

describe('signal fetch helpers', () => {
  beforeEach(() => {
    global.fetch = jest.fn().mockResolvedValue(jsonResponse(200))
  })

  afterEach(() => {
    jest.restoreAllMocks()
  })

  function lastCall() {
    const calls = (global.fetch as jest.Mock).mock.calls
    const [url, init] = calls[calls.length - 1]
    return { url: url as string, init: init as RequestInit }
  }

  it('reads the branch’s signals and keeps only the live ones', async () => {
    ;(global.fetch as jest.Mock).mockResolvedValue(
      jsonResponse(200, {
        success: true,
        signals: [testSignal(), testSignal({ signalId: 2, status: 'done' })],
      })
    )
    const signals = await fetchHkSignals('hfhotel')
    expect(lastCall().url).toBe(`${HK_API_BASE}/signals?branch=hfhotel`)
    expect(signals.map((s) => s.signalId)).toEqual([1])
  })

  // A newer/older backend that answers without the array must not crash a
  // maid's screen — the section renders empty instead.
  it('returns an empty list when the body carries no signals array', async () => {
    ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(200, { success: true }))
    await expect(fetchHkSignals('hfhotel')).resolves.toEqual([])
  })

  it('throws on a 200 that carries success: false', async () => {
    ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(200, { success: false }))
    await expect(fetchHkSignals('hfhotel')).rejects.toThrow(/ไม่สามารถดึงรายการแจ้งได้/)
  })

  // Direction is NEVER sent: the server derives it from the caller's role, so
  // a client bug cannot file a desk signal in a maid's name.
  it('sends only the type when filing a signal', async () => {
    ;(global.fetch as jest.Mock).mockResolvedValue(
      jsonResponse(200, { success: true, signal: testSignal() })
    )
    await sendHkSignal('hfhotel', 7, 'guest_in_room')
    const { url, init } = lastCall()
    expect(url).toBe(`${HK_API_BASE}/rooms/7/signals?branch=hfhotel`)
    expect(init.method).toBe('POST')
    expect(JSON.parse(init.body as string)).toEqual({ type: 'guest_in_room' })
  })

  it.each(['ack', 'done', 'cancel'] as const)('posts the %s transition', async (action) => {
    ;(global.fetch as jest.Mock).mockResolvedValue(
      jsonResponse(200, { success: true, signal: testSignal() })
    )
    await actOnHkSignal('hfhotel', 12, action)
    expect(lastCall().url).toBe(`${HK_API_BASE}/signals/12/${action}?branch=hfhotel`)
  })

  // The wire says what was FOUND, never what was fine: a clear answer carries
  // no `problems` key at all.
  it('answers เคลียร์ with outcome alone', async () => {
    ;(global.fetch as jest.Mock).mockResolvedValue(
      jsonResponse(200, { success: true, signal: testSignal(), spawned: [] })
    )
    const { spawned } = await answerHkRoomCheck('hfhotel', 12, { outcome: 'clear' })
    const { url, init } = lastCall()
    expect(url).toBe(`${HK_API_BASE}/signals/12/answer?branch=hfhotel`)
    expect(JSON.parse(init.body as string)).toEqual({ outcome: 'clear' })
    expect(spawned).toEqual([])
  })

  it('answers with both problems and returns the spawned children', async () => {
    const child = testSignal({ signalId: 21, direction: 'maid_to_desk', type: 'item_missing' })
    ;(global.fetch as jest.Mock).mockResolvedValue(
      jsonResponse(200, { success: true, signal: testSignal(), spawned: [child] })
    )
    const result = await answerHkRoomCheck('hfhotel', 12, {
      outcome: 'problems',
      problems: ['item_missing', 'item_damaged'],
    })
    expect(JSON.parse(lastCall().init.body as string)).toEqual({
      outcome: 'problems',
      problems: ['item_missing', 'item_damaged'],
    })
    expect(result.spawned).toEqual([child])
  })

  // A signal that never landed must never read as sent — the same rule the
  // linen report follows, and for the same reason: somebody is waiting on it.
  it('treats a 200 carrying success: false as a failed write', async () => {
    ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(200, { success: false }))
    await expect(sendHkSignal('hfhotel', 7, 'guest_in_room')).rejects.toThrow(/ส่งไม่สำเร็จ/)
  })

  it('refuses to fire at all without a branch', async () => {
    await expect(sendHkSignal(null, 7, 'guest_in_room')).rejects.toThrow()
    expect(global.fetch).not.toHaveBeenCalled()
  })
})

describe('signal sound mute (localStorage)', () => {
  beforeEach(() => localStorage.clear())

  // Unmuted by default: a maid holding a silent phone in a corridor is the
  // reason the cue exists at all.
  it('is audible until somebody mutes it', () => {
    expect(readSignalSoundMuted()).toBe(false)
    storeSignalSoundMuted(true)
    expect(readSignalSoundMuted()).toBe(true)
    storeSignalSoundMuted(false)
    expect(readSignalSoundMuted()).toBe(false)
  })
})

// ---------------------------------------------------------------------------
// Report HK — the daily room report (CONTEXT.md §Housekeeping "Room report").
//
// These are the helpers that decide what a maid is SHOWN before she taps, and
// what leaves the phone afterwards: the status prefill, the day overview's
// state chips and queue order, the exception draft the checklist is built from,
// the validation that keeps a report from being filed without evidence, and the
// exact bodies of the three writes.
// ---------------------------------------------------------------------------

function reportRow(overrides: Partial<HkReportRoom> = {}): HkReportRoom {
  return {
    roomId: 1,
    roomNo: '101',
    floor: 1,
    building: null,
    report: null,
    ...overrides,
  }
}

function summary(overrides: Partial<HkReportSummary> = {}): HkReportSummary {
  return {
    reportId: 11,
    roomId: 1,
    status: 'submitted',
    ...overrides,
  }
}

// ---------------------------------------------------------------------------
// The room-status prefill — the mapping table, asserted as a table.
//
// This decides which of VC/CO/OO/SO starts pressed on a form the maid can
// override with one tap, so a wrong answer is cheap; getting it right is what
// keeps the common room a single tap away from filed.
// ---------------------------------------------------------------------------

describe('prefillRoomStatus (the mapping table)', () => {
  it.each([
    ['occupied ⇒ SO (พักต่อ)', { occupancy: 'occupied' as const }, 'so'],
    [
      'occupied AND due out today ⇒ still SO — a departure the guest has not made is not a checkout',
      { occupancy: 'occupied' as const, expectedDeparture: true },
      'so',
    ],
    [
      'vacant AND due out today ⇒ CO (เช็คเอาท์)',
      { occupancy: 'vacant' as const, expectedDeparture: true },
      'co',
    ],
    ['vacant, nobody due out ⇒ VC', { occupancy: 'vacant' as const }, 'vc'],
    [
      'departure flag alone (occupancy absent — deploy skew) ⇒ CO',
      { expectedDeparture: true },
      'co',
    ],
    ['nothing known at all (deploy skew) ⇒ VC, never a guess', {}, 'vc'],
    [
      'a DIRTY vacant room still prefills VC — roomClean is deliberately not read, the report is filed after the work',
      { occupancy: 'vacant' as const, roomClean: false },
      'vc',
    ],
    [
      'an arrival due today changes nothing on its own',
      { occupancy: 'vacant' as const, expectedArrival: true },
      'vc',
    ],
  ])('%s', (_name, facts, expected) => {
    expect(prefillRoomStatus(room(facts as Partial<HkRoom>))).toBe(expected)
  })

  it('never prefills OO — nothing on a room says "out of order", so it stays a judgement the maid taps', () => {
    const derived = [
      { occupancy: 'occupied' as const },
      { occupancy: 'vacant' as const, expectedDeparture: true },
      { occupancy: 'vacant' as const, roomClean: false },
      {},
    ].map((facts) => prefillRoomStatus(room(facts as Partial<HkRoom>)))
    expect(derived).not.toContain('oo')
  })

  // A null room (the detail read has not answered yet) must not crash the form.
  it('falls back to VC for a room it has not been given', () => {
    expect(prefillRoomStatus(null)).toBe('vc')
    expect(prefillRoomStatus(undefined)).toBe('vc')
  })

  it('only ever returns a code the vocabulary knows', () => {
    const codes = ROOM_STATUS_CODES.map(({ code }) => code as string)
    expect(codes).toContain(prefillRoomStatus(room({ occupancy: 'occupied' })))
    expect(codes).toContain(prefillRoomStatus(room({})))
  })
})

// ---------------------------------------------------------------------------
// The day overview's state chip.
// ---------------------------------------------------------------------------

describe('reportState / reportStateChip', () => {
  it('reads a missing report as ยังไม่ส่ง', () => {
    expect(reportState(null)).toBe('unsent')
    expect(reportStateChip(null).label).toBe('ยังไม่ส่ง')
  })

  it('labels a submitted report ส่งแล้ว รอตรวจ', () => {
    expect(reportStateChip(summary({ status: 'submitted' })).label).toBe('ส่งแล้ว รอตรวจ')
  })

  it('labels a verified report ตรวจแล้ว', () => {
    expect(reportStateChip(summary({ status: 'verified' })).label).toBe('ตรวจแล้ว')
  })

  // The reason is PART of the chip: a maid scanning her queue has to know why
  // a room came back without opening it.
  it('names the canned reason on a returned report', () => {
    expect(
      reportStateChip(summary({ status: 'returned', returnReason: 'photos_unclear' })).label
    ).toBe('ส่งกลับแก้ไข: รูปไม่ชัดเจน')
  })

  it('still says ส่งกลับแก้ไข when the reason is missing', () => {
    expect(reportStateChip(summary({ status: 'returned', returnReason: null })).label).toBe(
      'ส่งกลับแก้ไข'
    )
  })

  // Deploy skew: a lifecycle value this bundle predates. "Somebody has filed
  // something" is the safe reading — it never invites a duplicate.
  it('reads an unknown status as submitted rather than as nothing filed', () => {
    const unknown = { ...summary(), status: 'escalated' } as unknown as HkReportSummary
    expect(reportState(unknown)).toBe('submitted')
  })

  it('gives each state its own colour vocabulary', () => {
    const classes = [
      reportStateChip(null).className,
      reportStateChip(summary({ status: 'submitted' })).className,
      reportStateChip(summary({ status: 'verified' })).className,
      reportStateChip(summary({ status: 'returned' })).className,
    ]
    expect(new Set(classes).size).toBe(4)
  })
})

// ---------------------------------------------------------------------------
// Each role's queue order — the same rooms, sorted by whose move it is.
// ---------------------------------------------------------------------------

describe('sortReportRooms / reportRoomPriority', () => {
  const unsent = reportRow({ roomId: 1, roomNo: '101' })
  const submitted = reportRow({ roomId: 2, roomNo: '102', report: summary({ status: 'submitted' }) })
  const verified = reportRow({ roomId: 3, roomNo: '103', report: summary({ status: 'verified' }) })
  const returned = reportRow({ roomId: 4, roomNo: '104', report: summary({ status: 'returned' }) })

  it("leads a maid's queue with the rooms that came back, then the ones she has not filed", () => {
    expect(
      sortReportRooms([verified, submitted, unsent, returned], 'maid').map((r) => r.roomNo)
    ).toEqual(['104', '101', '102', '103'])
  })

  it("leads reception's queue with what is waiting to be checked", () => {
    expect(
      sortReportRooms([verified, unsent, submitted, returned], 'reception').map((r) => r.roomNo)
    ).toEqual(['102', '104', '101', '103'])
  })

  // A finished room is finished for both roles.
  it('sinks verified rooms to the bottom for both roles', () => {
    for (const role of ['maid', 'reception'] as const) {
      const sorted = sortReportRooms([verified, unsent, submitted, returned], role)
      expect(sorted[sorted.length - 1].roomNo).toBe('103')
    }
  })

  // Walking order inside a state, the same numeric-aware compare every /hk
  // list uses — 95 before 104, not after it.
  it('breaks ties by room number, numerically', () => {
    const rows = [
      reportRow({ roomId: 1, roomNo: '301' }),
      reportRow({ roomId: 2, roomNo: '104' }),
      reportRow({ roomId: 3, roomNo: '95' }),
    ]
    expect(sortReportRooms(rows, 'maid').map((r) => r.roomNo)).toEqual(['95', '104', '301'])
  })

  it('does not mutate the list it was given', () => {
    const rows = [verified, unsent]
    sortReportRooms(rows, 'maid')
    expect(rows.map((r) => r.roomNo)).toEqual(['103', '101'])
  })

  it('ranks by state, not by position', () => {
    expect(reportRoomPriority(returned, 'maid')).toBeLessThan(reportRoomPriority(unsent, 'maid'))
    expect(reportRoomPriority(submitted, 'reception')).toBeLessThan(
      reportRoomPriority(returned, 'reception')
    )
  })
})

describe('reportStateCounts', () => {
  it('counts every room into exactly one state', () => {
    const counts = reportStateCounts([
      reportRow(),
      reportRow({ roomId: 2, roomNo: '102', report: summary({ status: 'submitted' }) }),
      reportRow({ roomId: 3, roomNo: '103', report: summary({ status: 'submitted' }) }),
      reportRow({ roomId: 4, roomNo: '104', report: summary({ status: 'verified' }) }),
      reportRow({ roomId: 5, roomNo: '105', report: summary({ status: 'returned' }) }),
    ])
    expect(counts).toEqual({ unsent: 1, submitted: 2, verified: 1, returned: 1 })
  })

  it('is all zeroes for an empty branch', () => {
    expect(reportStateCounts([])).toEqual({ unsent: 0, submitted: 0, verified: 0, returned: 0 })
  })
})

// ---------------------------------------------------------------------------
// Labels — every server→client code falls back to itself rather than being
// dropped, the same rule `linenKindLabel` follows.
// ---------------------------------------------------------------------------

describe('report labels', () => {
  it('spells the room-status codes the way the paper form does', () => {
    expect(roomStatusLabel('vc')).toBe('VC ห้องทำความสะอาดแล้ว')
    expect(roomStatusLabel('so')).toBe('SO พักต่อ')
  })

  it('falls back to the raw code for a status a newer backend knows', () => {
    expect(roomStatusLabel('xx')).toBe('xx')
    expect(itemProblemLabel('soiled')).toBe('soiled')
    expect(returnReasonLabel('smells')).toBe('smells')
  })

  it('renders nothing at all for an absent code', () => {
    expect(roomStatusLabel(null)).toBe('')
    expect(itemProblemLabel(undefined)).toBe('')
    expect(returnReasonLabel(null)).toBe('')
  })

  it('spells หาย / ชำรุด and the three canned reasons', () => {
    expect(itemProblemLabel('missing')).toBe('หาย')
    expect(itemProblemLabel('damaged')).toBe('ชำรุด')
    expect(returnReasonLabel('not_clean')).toBe('ยังไม่สะอาด')
  })

  // A calendar DAY, not an instant — nothing to convert, and nothing to get
  // wrong. An unparseable value renders as itself, never as "Invalid Date".
  it('renders an unparseable date as itself', () => {
    expect(reportDateLabel('not-a-date')).toBe('not-a-date')
    expect(reportDateLabel(null)).toBe('')
  })

  it('renders a real date as something Thai', () => {
    expect(reportDateLabel('2026-09-02')).not.toBe('')
    expect(reportDateLabel('2026-09-02')).not.toMatch(/Invalid/)
  })
})

// ---------------------------------------------------------------------------
// The exception draft — the sparse `"<item>:<problem>" → qty` record the
// checklist is built from. Keyed by the PAIR because one item can be wrong two
// ways at once (two towels gone, a third torn).
// ---------------------------------------------------------------------------

describe('the exception draft', () => {
  it('turns a pair on at qty 1 and off again', () => {
    let draft = toggleReportException({}, 'bath_towel', 'missing')
    expect(reportExceptionQty(draft, 'bath_towel', 'missing')).toBe(REPORT_MIN_QTY)
    draft = toggleReportException(draft, 'bath_towel', 'missing')
    expect(reportExceptionQty(draft, 'bath_towel', 'missing')).toBe(0)
  })

  // Off DELETES the key. A zero left behind would ship as `qty: 0`, which the
  // wire has no meaning for.
  it('deletes the key on the way off rather than storing a zero', () => {
    const draft = toggleReportException(
      toggleReportException({}, 'kettle', 'damaged'),
      'kettle',
      'damaged'
    )
    expect(Object.keys(draft)).toEqual([])
  })

  it('keeps หาย and ชำรุด for one item as two independent exceptions', () => {
    let draft = toggleReportException({}, 'bath_towel', 'missing')
    draft = toggleReportException(draft, 'bath_towel', 'damaged')
    expect(reportExceptionItems(draft)).toEqual([
      { item: 'bath_towel', problem: 'missing', qty: 1 },
      { item: 'bath_towel', problem: 'damaged', qty: 1 },
    ])
  })

  it('steps a selected pair and clamps in the REDUCER, not just on the buttons', () => {
    let draft = toggleReportException({}, 'pillow', 'missing')
    draft = stepReportException(draft, 'pillow', 'missing', 4)
    expect(reportExceptionQty(draft, 'pillow', 'missing')).toBe(5)
    draft = stepReportException(draft, 'pillow', 'missing', -99)
    expect(reportExceptionQty(draft, 'pillow', 'missing')).toBe(REPORT_MIN_QTY)
    draft = stepReportException(draft, 'pillow', 'missing', 500)
    expect(reportExceptionQty(draft, 'pillow', 'missing')).toBe(REPORT_MAX_QTY)
  })

  it('will not step a pair that is not an exception at all', () => {
    expect(stepReportException({}, 'pillow', 'missing', 1)).toEqual({})
  })

  it('clamps a nonsense quantity to the floor rather than shipping NaN', () => {
    expect(clampReportQty(Number.NaN)).toBe(REPORT_MIN_QTY)
    expect(clampReportQty(2.7)).toBe(2)
  })

  // Order is the vocabulary's, not the tapping order: the report reads the way
  // the paper form does however she filled it in.
  it('emits items in REPORT_ITEMS × ITEM_PROBLEMS order however they were tapped', () => {
    let draft = toggleReportException({}, 'pillow', 'damaged')
    draft = toggleReportException(draft, 'water_glass', 'missing')
    draft = toggleReportException(draft, 'kettle', 'missing')
    expect(reportExceptionItems(draft).map((i) => i.item)).toEqual([
      'water_glass',
      'kettle',
      'pillow',
    ])
    expect(REPORT_ITEMS.map(({ item }) => item).indexOf('water_glass')).toBeLessThan(
      REPORT_ITEMS.map(({ item }) => item).indexOf('kettle')
    )
  })

  it('emits nothing for an empty draft', () => {
    expect(reportExceptionItems({})).toEqual([])
  })

  // The RETURNED-report path: her previous answers come back into the fresh
  // form so she fixes what was wrong instead of re-entering 22 rows.
  it('round-trips a filed report back into a draft', () => {
    const items = [
      { item: 'bath_towel', problem: 'missing', qty: 2 },
      { item: 'tv_remote', problem: 'damaged', qty: 1 },
    ]
    expect(reportExceptionItems(reportExceptionDraftFrom(items))).toEqual([
      { item: 'tv_remote', problem: 'damaged', qty: 1 },
      { item: 'bath_towel', problem: 'missing', qty: 2 },
    ])
  })

  it('clamps an out-of-range quantity coming back from the server', () => {
    const draft = reportExceptionDraftFrom([{ item: 'pillow', problem: 'missing', qty: 9999 }])
    expect(reportExceptionQty(draft, 'pillow', 'missing')).toBe(REPORT_MAX_QTY)
  })

  it('reads an absent item list as an empty draft', () => {
    expect(reportExceptionDraftFrom(null)).toEqual({})
    expect(reportExceptionDraftFrom(undefined)).toEqual({})
  })

  it('spells one key for a pair, everywhere', () => {
    expect(reportExceptionKey('kettle', 'missing')).toBe('kettle:missing')
  })
})

describe('reportItemRows', () => {
  it('resolves Thai labels and keeps the delivered order', () => {
    expect(
      reportItemRows([
        { item: 'tv_remote', problem: 'damaged', qty: 1 },
        { item: 'bath_towel', problem: 'missing', qty: 2 },
      ])
    ).toEqual([
      {
        key: 'tv_remote:damaged',
        item: 'tv_remote',
        problem: 'damaged',
        qty: 1,
        label: 'รีโมทโทรทัศน์',
        problemLabel: 'ชำรุด',
      },
      {
        key: 'bath_towel:missing',
        item: 'bath_towel',
        problem: 'missing',
        qty: 2,
        label: 'ผ้าขนหนู (รวมสีฟ้า)',
        problemLabel: 'หาย',
      },
    ])
  })

  // A readable row beats a dropped one — a newer backend's 23rd item must
  // still render on an older bundle.
  it('renders an item this bundle predates as its raw code', () => {
    expect(reportItemRows([{ item: 'minibar', problem: 'missing', qty: 1 }])[0].label).toBe(
      'minibar'
    )
  })

  it('reads an absent list as no rows', () => {
    expect(reportItemRows(null)).toEqual([])
  })
})

// ---------------------------------------------------------------------------
// Validation — every rule the server would refuse on, checked before the maid
// is told "no" on a screen she has already left.
// ---------------------------------------------------------------------------

describe('canSubmitReport', () => {
  const ok = {
    roomStatus: 'vc',
    allItemsOk: true,
    items: [] as Array<{ item: string; problem: string; qty: number }>,
    photoIds: [1],
  }

  it('accepts a clean, photographed report', () => {
    expect(canSubmitReport(ok)).toBe(true)
  })

  // THE rule: evidence is not optional.
  it('refuses a report with no photo at all', () => {
    expect(canSubmitReport({ ...ok, photoIds: [] })).toBe(false)
  })

  it('refuses more than four photos', () => {
    expect(canSubmitReport({ ...ok, photoIds: [1, 2, 3, 4, 5] })).toBe(false)
    expect(canSubmitReport({ ...ok, photoIds: [1, 2, 3, 4] })).toBe(true)
    expect(REPORT_MAX_PHOTOS).toBe(4)
  })

  it('refuses a report with no room status chosen', () => {
    expect(canSubmitReport({ ...ok, roomStatus: null })).toBe(false)
  })

  it('refuses a room-status code the vocabulary does not carry', () => {
    expect(canSubmitReport({ ...ok, roomStatus: 'zz' })).toBe(false)
  })

  // The toggle and the list are one claim, and they must agree.
  it('refuses ครบทุกรายการ carrying exceptions', () => {
    expect(
      canSubmitReport({
        ...ok,
        allItemsOk: true,
        items: [{ item: 'kettle', problem: 'missing', qty: 1 }],
      })
    ).toBe(false)
  })

  it('refuses มีรายการผิดปกติ that names nothing', () => {
    expect(canSubmitReport({ ...ok, allItemsOk: false, items: [] })).toBe(false)
  })

  it('accepts มีรายการผิดปกติ that names something', () => {
    expect(
      canSubmitReport({
        ...ok,
        allItemsOk: false,
        items: [{ item: 'kettle', problem: 'missing', qty: 1 }],
      })
    ).toBe(true)
  })

  it('refuses a quantity outside 1..99', () => {
    expect(
      canSubmitReport({
        ...ok,
        allItemsOk: false,
        items: [{ item: 'kettle', problem: 'missing', qty: 0 }],
      })
    ).toBe(false)
    expect(
      canSubmitReport({
        ...ok,
        allItemsOk: false,
        items: [{ item: 'kettle', problem: 'missing', qty: 100 }],
      })
    ).toBe(false)
  })
})

describe('reportItemsConsistent / reportPhotoCountValid', () => {
  it('is empty iff ok', () => {
    expect(reportItemsConsistent(true, [])).toBe(true)
    expect(reportItemsConsistent(true, [{ item: 'kettle', problem: 'missing', qty: 1 }])).toBe(false)
    expect(reportItemsConsistent(false, [])).toBe(false)
    expect(reportItemsConsistent(false, [{ item: 'kettle', problem: 'missing', qty: 1 }])).toBe(true)
  })

  it('bounds photos at 1..4 on both sides', () => {
    expect(reportPhotoCountValid(0)).toBe(false)
    expect(reportPhotoCountValid(1)).toBe(true)
    expect(reportPhotoCountValid(4)).toBe(true)
    expect(reportPhotoCountValid(5)).toBe(false)
  })
})

describe('canVerifyReport / canReturnReport', () => {
  // Reception's OWN photos are what make a verify a walk-up rather than a desk
  // stamp — the two-sided evidence IS the feature.
  it('refuses a verify with no photo of its own', () => {
    expect(canVerifyReport([])).toBe(false)
    expect(canVerifyReport([9])).toBe(true)
    expect(canVerifyReport([1, 2, 3, 4, 5])).toBe(false)
  })

  // A return is a rejection, not a walk-up: a reason, no photos, and the
  // reason must be one of exactly three.
  it('refuses a return with no reason, or one outside the vocabulary', () => {
    expect(canReturnReport(null)).toBe(false)
    expect(canReturnReport('too dirty')).toBe(false)
    expect(canReturnReport('not_clean')).toBe(true)
  })

  it('accepts every canned reason and nothing else', () => {
    for (const { reason } of RETURN_REASONS) expect(canReturnReport(reason)).toBe(true)
    expect(RETURN_REASONS).toHaveLength(3)
  })
})

// A maid NEVER verifies — including one who also holds the reception grant,
// which `canReport` has already resolved to the maid side.
describe('canFileReport / canVerifyReports', () => {
  it('gives filing to the maid side and verifying to the desk side, never both', () => {
    expect(canFileReport('maid')).toBe(true)
    expect(canVerifyReports('maid')).toBe(false)
    expect(canFileReport('reception')).toBe(false)
    expect(canVerifyReports('reception')).toBe(true)
  })
})

// ---------------------------------------------------------------------------
// Photo plumbing.
// ---------------------------------------------------------------------------

describe('downscaleDimensions', () => {
  it('leaves a photo that already fits alone', () => {
    expect(downscaleDimensions(800, 600, 1600)).toEqual({ width: 800, height: 600 })
  })

  it('scales by the LONGEST edge, keeping the aspect ratio', () => {
    expect(downscaleDimensions(4000, 3000, 1600)).toEqual({ width: 1600, height: 1200 })
    expect(downscaleDimensions(3000, 4000, 1600)).toEqual({ width: 1200, height: 1600 })
  })

  it('never rounds an edge down to zero', () => {
    expect(downscaleDimensions(4000, 3, 1600).height).toBeGreaterThanOrEqual(1)
  })

  // A broken decoder must read as "don't touch this file", not as a 0×0 canvas.
  it('returns 0×0 for nonsense the caller should refuse to redraw', () => {
    expect(downscaleDimensions(0, 100)).toEqual({ width: 0, height: 0 })
    expect(downscaleDimensions(Number.NaN, 100)).toEqual({ width: 0, height: 0 })
    expect(downscaleDimensions(-4, -4)).toEqual({ width: 0, height: 0 })
  })
})

describe('hkReportPhotoUrl', () => {
  it('is branch-scoped — the browser issues this one itself, so hkFetch cannot scope it for us', () => {
    expect(hkReportPhotoUrl(42, 'hfville')).toBe(`${HK_API_BASE}/report-photos/42?branch=hfville`)
  })

  // A wrong-hotel image URL is the same class of bug as a wrong-hotel report.
  it('renders nothing rather than an unscoped URL when no branch is chosen', () => {
    expect(hkReportPhotoUrl(42, null)).toBe('')
  })
})

// ---------------------------------------------------------------------------
// The wire — what the three writes and the two reads actually send.
// ---------------------------------------------------------------------------

describe('report fetch helpers', () => {
  beforeEach(() => {
    global.fetch = jest.fn().mockResolvedValue(jsonResponse(200, { success: true }))
  })

  afterEach(() => {
    jest.restoreAllMocks()
  })

  function lastCall() {
    const calls = (global.fetch as jest.Mock).mock.calls
    const [url, init] = calls[calls.length - 1]
    return { url: url as string, init: (init ?? {}) as RequestInit }
  }

  function body() {
    return JSON.parse(String(lastCall().init.body))
  }

  describe('fetchHkReports', () => {
    it('asks for the branch’s day sheet, with no date — the server owns "today"', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(
        jsonResponse(200, { success: true, date: '2026-09-02', rooms: [] })
      )
      const result = await fetchHkReports('hfhotel')
      expect(lastCall().url).toBe(`${HK_API_BASE}/reports?branch=hfhotel`)
      expect(result.date).toBe('2026-09-02')
    })

    it('passes an explicit date through as a query parameter', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(
        jsonResponse(200, { success: true, date: '2026-09-01', rooms: [] })
      )
      await fetchHkReports('hfhotel', '2026-09-01')
      expect(lastCall().url).toBe(`${HK_API_BASE}/reports?date=2026-09-01&branch=hfhotel`)
    })

    // A page must be able to render the list unconditionally.
    it('returns [] for a backend that answers without a rooms array', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(200, { success: true }))
      await expect(fetchHkReports('hfhotel')).resolves.toEqual({ date: '', rooms: [] })
    })

    it('throws on a 200 carrying success: false', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(200, { success: false }))
      await expect(fetchHkReports('hfhotel')).rejects.toThrow(/ไม่สามารถดึงรายงานได้/)
    })

    it('refuses to fire at all without a branch', async () => {
      await expect(fetchHkReports(null)).rejects.toThrow()
      expect(global.fetch).not.toHaveBeenCalled()
    })
  })

  describe('fetchHkRoomReport', () => {
    const day = {
      success: true,
      date: '2026-09-02',
      rooms: [
        { roomId: 7, roomNo: '104', floor: 1, building: null, report: { reportId: 55, roomId: 7, status: 'submitted' } },
        { roomId: 8, roomNo: '105', floor: 1, building: null, report: null },
      ],
    }

    // The day list IS the "latest report for this room" index — there is no
    // per-room endpoint — and the summary DTO carries no items, so the full
    // report has to be fetched behind it.
    it('finds the room in the day sheet and fetches its report IN FULL', async () => {
      ;(global.fetch as jest.Mock)
        .mockResolvedValueOnce(jsonResponse(200, day))
        .mockResolvedValueOnce(
          jsonResponse(200, {
            success: true,
            report: { reportId: 55, roomId: 7, status: 'submitted', items: [], maidPhotoIds: [3] },
          })
        )
      const result = await fetchHkRoomReport('hfhotel', 7)
      expect((global.fetch as jest.Mock).mock.calls[1][0]).toBe(
        `${HK_API_BASE}/reports/55?branch=hfhotel`
      )
      expect(result.report?.maidPhotoIds).toEqual([3])
      expect(result.room?.roomNo).toBe('104')
      expect(result.date).toBe('2026-09-02')
    })

    // The ยังไม่ส่ง case: one request, and no report to go looking for.
    it('issues no second request for a room that has filed nothing', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(200, day))
      const result = await fetchHkRoomReport('hfhotel', 8)
      expect((global.fetch as jest.Mock).mock.calls).toHaveLength(1)
      expect(result.report).toBeNull()
      expect(result.room?.roomNo).toBe('105')
    })

    it('returns a null room for one the branch does not serve', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(200, day))
      await expect(fetchHkRoomReport('hfhotel', 999)).resolves.toMatchObject({
        room: null,
        report: null,
      })
    })
  })

  describe('fetchHkReport', () => {
    it('reads one report by id', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(
        jsonResponse(200, { success: true, report: { reportId: 9, roomId: 1, status: 'verified' } })
      )
      await expect(fetchHkReport('hfhotel', 9)).resolves.toMatchObject({ reportId: 9 })
      expect(lastCall().url).toBe(`${HK_API_BASE}/reports/9?branch=hfhotel`)
    })

    it('throws when the answer carries no report', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(200, { success: true }))
      await expect(fetchHkReport('hfhotel', 9)).rejects.toThrow(/ไม่สามารถดึงรายงานได้/)
    })
  })

  describe('submitHkReport', () => {
    beforeEach(() => {
      ;(global.fetch as jest.Mock).mockResolvedValue(
        jsonResponse(200, { success: true, report: { reportId: 77, roomId: 7, status: 'submitted' } })
      )
    })

    it('posts the exact body the contract names, to the room-scoped path', async () => {
      await submitHkReport('hfhotel', 7, {
        roomStatus: 'co',
        allItemsOk: false,
        items: [{ item: 'bath_towel', problem: 'missing', qty: 2 }],
        photoIds: [11, 12],
      })
      expect(lastCall().url).toBe(`${HK_API_BASE}/rooms/7/report?branch=hfhotel`)
      expect(lastCall().init.method).toBe('POST')
      expect(body()).toEqual({
        roomStatus: 'co',
        allItemsOk: false,
        items: [{ item: 'bath_towel', problem: 'missing', qty: 2 }],
        photoIds: [11, 12],
      })
    })

    // Append-only history: a fix POINTS AT what it supersedes.
    it('carries parentReportId when it is fixing a returned report', async () => {
      await submitHkReport('hfhotel', 7, {
        roomStatus: 'vc',
        allItemsOk: true,
        items: [],
        photoIds: [11],
        parentReportId: 55,
      })
      expect(body().parentReportId).toBe(55)
    })

    // 409 is an ANSWER, not a retry: the room already has a report today.
    it('turns a 409 into its own Thai copy, distinct from a write failure', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(409, { success: false }))
      await expect(
        submitHkReport('hfhotel', 7, {
          roomStatus: 'vc',
          allItemsOk: true,
          items: [],
          photoIds: [1],
        })
      ).rejects.toThrow(/ส่งรายงานของวันนี้ไปแล้ว/)
    })

    it('treats a 200 carrying success: false as a failed write', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(200, { success: false }))
      await expect(
        submitHkReport('hfhotel', 7, {
          roomStatus: 'vc',
          allItemsOk: true,
          items: [],
          photoIds: [1],
        })
      ).rejects.toThrow(/บันทึกไม่สำเร็จ/)
    })
  })

  describe('verifyHkReport / returnHkReport', () => {
    beforeEach(() => {
      ;(global.fetch as jest.Mock).mockResolvedValue(
        jsonResponse(200, { success: true, report: { reportId: 55, roomId: 7, status: 'verified' } })
      )
    })

    it('verifies with reception’s own photo ids and nothing else', async () => {
      await verifyHkReport('hfhotel', 55, [21, 22])
      expect(lastCall().url).toBe(`${HK_API_BASE}/reports/55/verify?branch=hfhotel`)
      expect(body()).toEqual({ photoIds: [21, 22] })
    })

    it('returns with a canned reason and NO photos', async () => {
      await returnHkReport('hfhotel', 55, 'photos_unclear')
      expect(lastCall().url).toBe(`${HK_API_BASE}/reports/55/return?branch=hfhotel`)
      expect(body()).toEqual({ reason: 'photos_unclear' })
    })

    it('refuses to fire either without a branch', async () => {
      await expect(verifyHkReport(null, 55, [1])).rejects.toThrow()
      await expect(returnHkReport(null, 55, 'not_clean')).rejects.toThrow()
      expect(global.fetch).not.toHaveBeenCalled()
    })
  })

  describe('uploadHkReportPhoto', () => {
    it('posts multipart under the field name "photo" and returns the id', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(
        jsonResponse(200, { success: true, photoId: 31 })
      )
      const blob = new Blob(['x'], { type: 'image/jpeg' })
      await expect(uploadHkReportPhoto('hfhotel', blob)).resolves.toBe(31)
      expect(lastCall().url).toBe(`${HK_API_BASE}/report-photos?branch=hfhotel`)
      const form = lastCall().init.body as FormData
      expect(form).toBeInstanceOf(FormData)
      expect(form.get('photo')).toBeTruthy()
    })

    // Setting Content-Type by hand is the classic way to break every multipart
    // upload: the boundary the browser generates would no longer match.
    it('sets NO Content-Type header — the browser owns the multipart boundary', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(
        jsonResponse(200, { success: true, photoId: 31 })
      )
      await uploadHkReportPhoto('hfhotel', new Blob(['x'], { type: 'image/jpeg' }))
      expect(lastCall().init.headers).toBeUndefined()
    })

    // Refused HERE, before a slow upload over hotel wifi ends in a bare 413.
    it('refuses an oversized photo without issuing a request at all', async () => {
      const huge = { size: 6 * 1024 * 1024, type: 'image/jpeg' } as Blob
      await expect(uploadHkReportPhoto('hfhotel', huge)).rejects.toThrow(/รูปใหญ่เกินไป/)
      expect(global.fetch).not.toHaveBeenCalled()
    })

    it('turns the server’s own 413 into the same Thai copy', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(413, {}))
      await expect(
        uploadHkReportPhoto('hfhotel', new Blob(['x'], { type: 'image/jpeg' }))
      ).rejects.toThrow(/รูปใหญ่เกินไป/)
    })

    it('throws when the answer carries no photoId', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(200, { success: true }))
      await expect(
        uploadHkReportPhoto('hfhotel', new Blob(['x'], { type: 'image/jpeg' }))
      ).rejects.toThrow(/อัปโหลดรูปไม่สำเร็จ/)
    })
  })
})

// ---------------------------------------------------------------------------
// The cross-screen banner — a landed write sends the user back to the day
// overview, and the confirmation has to survive that navigation.
// ---------------------------------------------------------------------------

describe('the report notice hand-off', () => {
  beforeEach(() => sessionStorage.clear())

  it('hands one message across a navigation and clears it', () => {
    stashHkReportNotice('ส่งรายงานแล้ว')
    expect(takeHkReportNotice()).toBe('ส่งรายงานแล้ว')
    // A banner that survives a reload is a banner about nothing.
    expect(takeHkReportNotice()).toBeNull()
  })

  it('is null when nothing was stashed', () => {
    expect(takeHkReportNotice()).toBeNull()
  })
})

// The checklist vocabulary itself is `report-vocab.ts`'s, and the /hk client
// re-exports it so a page has ONE import — the same arrangement the signal
// vocabulary has. Asserted here because a broken re-export is invisible until
// a screen renders an empty picker.
describe('the re-exported report vocabulary', () => {
  it('carries the paper form’s items, problems, statuses and reasons', () => {
    expect(REPORT_ITEMS.length).toBeGreaterThan(0)
    expect(ITEM_PROBLEMS.map(({ problem }) => problem)).toEqual(['missing', 'damaged'])
    expect(ROOM_STATUS_CODES.map(({ code }) => code)).toEqual(['vc', 'co', 'oo', 'so'])
    expect(RETURN_REASONS.map(({ reason }) => reason)).toEqual([
      'not_clean',
      'items_mismatch',
      'photos_unclear',
    ])
  })
})
