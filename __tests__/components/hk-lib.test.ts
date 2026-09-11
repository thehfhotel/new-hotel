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
  applyZoneCapture,
  bindTickPhoto,
  buildReportTicks,
  canFileReport,
  canReport,
  canReturnReport,
  canSubmitReport,
  canVerifyReport,
  canVerifyReports,
  clampReportQty,
  clearReportDraft,
  countRoomsNeedingClean,
  cycleTickPhoto,
  cycleTickState,
  deleteHkReportPhoto,
  downscaleDimensions,
  downscalePhoto,
  fetchHkReport,
  fetchHkReportPhotoMeta,
  fetchHkReports,
  fetchHkRoomReport,
  hkReportPhotoUrl,
  itemProblemLabel,
  ITEM_PROBLEMS,
  nextUploadPhoto,
  nextUploadWakeMs,
  photoChipLabel,
  prefillRoomStatus,
  readReportDraft,
  reconcileReportDraft,
  reduceUploadQueue,
  REPORT_ITEMS,
  REPORT_MAX_PHOTOS,
  REPORT_MAX_PHOTOS_TOTAL,
  REPORT_MAX_QTY,
  REPORT_MIN_PHOTOS_TOTAL,
  REPORT_MIN_QTY,
  REPORT_UPLOAD_MAX_ATTEMPTS,
  REPORT_ZONES,
  reportAllOk,
  reportDateLabel,
  reportDraftKey,
  reportExtraPhotoIds,
  reportItemRows,
  reportItemZone,
  reportPhotoCountValid,
  reportPhotoGroups,
  reportPhotoIds,
  reportPhotoTotalValid,
  reportProblemCount,
  reportRoomPriority,
  reportSidePhotoIds,
  reportState,
  reportStateChip,
  reportStateCounts,
  reportTickRows,
  reportTicksByZone,
  reportTicksSubmission,
  reportZoneItems,
  reportZoneProgress,
  returnHkReport,
  returnReasonLabel,
  RETURN_REASONS,
  ROOM_STATUS_CODES,
  roomStatusLabel,
  sortReportRooms,
  stashHkReportNotice,
  stepTickQty,
  submitHkReport,
  takeHkReportNotice,
  tickDraftFromReport,
  ticksBackedBy,
  unbindPhotoTicks,
  uploadBackoffMs,
  uploadCounts,
  uploadHkReportPhoto,
  uploadProgressLabel,
  uploadsSettled,
  verifyHkReport,
  writeReportDraft,
  type HkLocalPhoto,
  type HkReportRoom,
  type HkReportSummary,
  type HkTickDraft,
  type RoomStatusCode,
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
// THE TICK MODEL — v2's replacement for the exception draft.
//
// The maid's working state is two plain records: item → tick, and the list of
// shots she has taken. What these pin is the behaviour that makes a perfect
// room four taps: a zone's photo PRE-TICKS its items ครบ, a tap cycles one item
// through the three states, and removing a photo unbinds the ticks it backed
// WITHOUT losing them.
// ---------------------------------------------------------------------------

function localPhoto(
  key: string,
  zone: string,
  overrides: Partial<HkLocalPhoto> = {}
): HkLocalPhoto {
  return {
    key,
    zone,
    photoId: null,
    bytes: null,
    attempts: 0,
    status: 'queued',
    failedAt: null,
    ...overrides,
  }
}

function uploadedPhoto(key: string, zone: string, photoId: number): HkLocalPhoto {
  return localPhoto(key, zone, { photoId, status: 'uploaded', attempts: 1, bytes: 2048 })
}

/** A whole perfect room: one landed shot per zone, every item pre-ticked ครบ
 *  against it. The shape the submit path is asserted against. */
function perfectRoom(): { ticks: HkTickDraft; photos: HkLocalPhoto[] } {
  let ticks: HkTickDraft = {}
  const photos: HkLocalPhoto[] = []
  REPORT_ZONES.forEach((zone, index) => {
    const key = 'photo-' + index
    photos.push(uploadedPhoto(key, zone.zone, 100 + index))
    ticks = applyZoneCapture(ticks, zone.zone, key)
  })
  return { ticks, photos }
}

describe('the capture zones', () => {
  // The zones ARE the shooting order, and the stepper walks them in this
  // sequence — bed first because it is the biggest surface in the room.
  it('shoots เตียง → โต๊ะและมินิบาร์ → ห้องน้ำ → ทั่วไป', () => {
    expect(REPORT_ZONES.map(({ zone }) => zone)).toEqual(['bed', 'desk', 'bathroom', 'general'])
    expect(REPORT_ZONES.map(({ label }) => label)).toEqual([
      'เตียง',
      'โต๊ะและมินิบาร์',
      'ห้องน้ำ',
      'ทั่วไป',
    ])
  })

  // Load-bearing: an item in two zones would be pre-ticked twice and an item in
  // none could never be ticked at all.
  it('puts every one of the 22 items in exactly ONE zone', () => {
    const zoned = REPORT_ZONES.flatMap(({ items }) => items as readonly string[])
    expect(zoned).toHaveLength(REPORT_ITEMS.length)
    expect(new Set(zoned).size).toBe(REPORT_ITEMS.length)
    for (const { item } of REPORT_ITEMS) expect(reportItemZone(item)).not.toBeNull()
  })

  it('answers null for an item code this bundle predates', () => {
    expect(reportItemZone('minibar')).toBeNull()
  })
})

describe('applyZoneCapture (the pre-tick)', () => {
  it('ticks every item of the zone ครบ against the shot, and nothing else', () => {
    const ticks = applyZoneCapture({}, 'bed', 'photo-0')
    expect(Object.keys(ticks).sort()).toEqual([...reportZoneItems('bed')].sort())
    expect(ticks.pillow).toEqual({ state: 'ok', qty: null, photo: 'photo-0' })
    expect(ticks.kettle).toBeUndefined()
  })

  // A second shot of the same zone is an EXTRA, not a reset: it must not undo a
  // หาย she has already tapped.
  it('leaves a decision she has already made alone', () => {
    let ticks = applyZoneCapture({}, 'bed', 'photo-0')
    ticks = cycleTickState(ticks, 'pillow')
    ticks = applyZoneCapture(ticks, 'bed', 'photo-1')
    expect(ticks.pillow).toEqual({ state: 'missing', qty: 1, photo: 'photo-0' })
  })

  // ...but a tick whose photo she REMOVED is repaired by the retake, which is
  // the whole reason remove unbinds instead of deleting.
  it('re-binds a tick left without a photo', () => {
    let ticks = applyZoneCapture({}, 'bed', 'photo-0')
    ticks = unbindPhotoTicks(ticks, 'photo-0')
    expect(ticks.pillow.photo).toBeNull()
    ticks = applyZoneCapture(ticks, 'bed', 'photo-9')
    expect(ticks.pillow).toEqual({ state: 'ok', qty: null, photo: 'photo-9' })
  })
})

describe('cycling a tick', () => {
  it('goes ครบ → หาย → ชำรุด → ครบ', () => {
    let ticks = applyZoneCapture({}, 'bed', 'p')
    expect(ticks.duvet.state).toBe('ok')
    ticks = cycleTickState(ticks, 'duvet')
    expect(ticks.duvet.state).toBe('missing')
    ticks = cycleTickState(ticks, 'duvet')
    expect(ticks.duvet.state).toBe('damaged')
    ticks = cycleTickState(ticks, 'duvet')
    expect(ticks.duvet.state).toBe('ok')
  })

  it('opens a problem at qty 1 and drops the quantity on the way back to ครบ', () => {
    let ticks = cycleTickState(applyZoneCapture({}, 'bed', 'p'), 'pillow')
    expect(ticks.pillow.qty).toBe(REPORT_MIN_QTY)
    ticks = stepTickQty(ticks, 'pillow', 2)
    expect(ticks.pillow.qty).toBe(3)
    // หาย → ชำรุด keeps the count: three towels gone and three torn is the
    // same three she already counted.
    ticks = cycleTickState(ticks, 'pillow')
    expect(ticks.pillow).toMatchObject({ state: 'damaged', qty: 3 })
    ticks = cycleTickState(ticks, 'pillow')
    expect(ticks.pillow).toMatchObject({ state: 'ok', qty: null })
  })

  it('clamps the quantity in the REDUCER, not just on the buttons', () => {
    let ticks = cycleTickState(applyZoneCapture({}, 'bed', 'p'), 'pillow')
    ticks = stepTickQty(ticks, 'pillow', 500)
    expect(ticks.pillow.qty).toBe(REPORT_MAX_QTY)
    ticks = stepTickQty(ticks, 'pillow', -500)
    expect(ticks.pillow.qty).toBe(REPORT_MIN_QTY)
  })

  it('will not step an ok tick, and will not cycle an item the zone has not shot', () => {
    const ticks = applyZoneCapture({}, 'bed', 'p')
    expect(stepTickQty(ticks, 'pillow', 1)).toBe(ticks)
    expect(cycleTickState(ticks, 'kettle')).toBe(ticks)
  })
})

describe('binding a tick to a photo', () => {
  // The close-up: a second shot that backs ONE tick instead of the zone.
  it('rebinds one tick to a close-up without touching its neighbours', () => {
    let ticks = applyZoneCapture({}, 'bathroom', 'zone-shot')
    ticks = cycleTickState(ticks, 'bath_towel')
    ticks = bindTickPhoto(ticks, 'bath_towel', 'close-up')
    expect(ticks.bath_towel.photo).toBe('close-up')
    expect(ticks.hairdryer.photo).toBe('zone-shot')
  })

  it('cycles through the zone’s own shots and wraps', () => {
    const photos = [
      uploadedPhoto('a', 'bed', 1),
      uploadedPhoto('b', 'bed', 2),
      uploadedPhoto('c', 'desk', 3),
    ]
    let ticks = applyZoneCapture({}, 'bed', 'a')
    ticks = cycleTickPhoto(ticks, 'pillow', photos)
    expect(ticks.pillow.photo).toBe('b')
    // 'c' belongs to another zone and is never offered.
    ticks = cycleTickPhoto(ticks, 'pillow', photos)
    expect(ticks.pillow.photo).toBe('a')
  })

  it('labels which of the zone’s shots backs a tick', () => {
    const photos = [uploadedPhoto('a', 'bed', 1), uploadedPhoto('b', 'bed', 2)]
    expect(photoChipLabel(photos, 'a')).toBe('รูปที่ 1/2')
    expect(photoChipLabel(photos, 'b')).toBe('รูปที่ 2/2')
    expect(photoChipLabel(photos, null)).toBe('')
  })
})

// Removing a photo is the one action that could silently un-attest a room, and
// it must not.
describe('removing a photo unbinds, never deletes', () => {
  it('keeps every tick and only takes their evidence away', () => {
    let ticks = applyZoneCapture({}, 'bed', 'photo-0')
    ticks = cycleTickState(ticks, 'pillow')
    ticks = stepTickQty(ticks, 'pillow', 1)
    const after = unbindPhotoTicks(ticks, 'photo-0')
    expect(Object.keys(after).sort()).toEqual(Object.keys(ticks).sort())
    expect(after.pillow).toEqual({ state: 'missing', qty: 2, photo: null })
  })

  it('leaves ticks backed by another photo alone', () => {
    let ticks = applyZoneCapture({}, 'bed', 'photo-0')
    ticks = bindTickPhoto(ticks, 'pillow', 'photo-1')
    const after = unbindPhotoTicks(ticks, 'photo-0')
    expect(after.pillow.photo).toBe('photo-1')
    expect(after.duvet.photo).toBeNull()
  })

  // The caption of the full-screen viewer, and what makes the cost of a
  // removal visible before she taps it.
  it('names what one photo backs, in the paper form’s order', () => {
    const ticks = applyZoneCapture({}, 'bed', 'photo-0')
    expect(ticksBackedBy(ticks, 'photo-0')).toEqual([
      'duvet',
      'bed_sheet',
      'pillowcase',
      'duvet_cover',
      'pillow',
    ])
    expect(ticksBackedBy(ticks, 'nothing')).toEqual([])
  })
})

describe('zone progress', () => {
  it('reports each zone’s shots, ticks and problems in shooting order', () => {
    const { ticks, photos } = perfectRoom()
    const progress = reportZoneProgress(ticks, photos)
    expect(progress.map((p) => p.zone)).toEqual(['bed', 'desk', 'bathroom', 'general'])
    expect(progress.every((p) => p.done)).toBe(true)
    expect(progress[0].photoCount).toBe(1)
    expect(progress[0].backedCount).toBe(progress[0].itemCount)
    expect(progress.reduce((n, p) => n + p.itemCount, 0)).toBe(REPORT_ITEMS.length)
  })

  it('counts a problem and an unbacked tick separately', () => {
    let ticks = applyZoneCapture({}, 'bed', 'photo-0')
    ticks = cycleTickState(ticks, 'pillow')
    ticks = unbindPhotoTicks(ticks, 'photo-0')
    const [bed] = reportZoneProgress(ticks, [])
    expect(bed).toMatchObject({ problemCount: 1, backedCount: 0, done: false })
    expect(bed.unbackedCount).toBe(bed.itemCount)
  })
})

describe('building the submit body', () => {
  it('emits all 22 ticks in the paper form’s order, photo-backed', () => {
    const { ticks, photos } = perfectRoom()
    const drafts = buildReportTicks(ticks, photos)
    expect(drafts.map((t) => t.item)).toEqual(REPORT_ITEMS.map(({ item }) => item))
    const body = reportTicksSubmission(drafts)
    expect(body).toHaveLength(REPORT_ITEMS.length)
    expect(body?.every((tick) => tick.state === 'ok')).toBe(true)
    // An ok tick carries NO quantity — the server refuses one that does.
    expect(body?.every((tick) => !('qty' in tick))).toBe(true)
    expect(body?.[0]).toEqual({ item: 'water_glass', state: 'ok', photoId: 101 })
  })

  it('carries a quantity on problems only', () => {
    const { photos } = perfectRoom()
    let { ticks } = perfectRoom()
    ticks = stepTickQty(cycleTickState(ticks, 'bath_towel'), 'bath_towel', 1)
    const body = reportTicksSubmission(buildReportTicks(ticks, photos))
    expect(body?.find((tick) => tick.item === 'bath_towel')).toEqual({
      item: 'bath_towel',
      state: 'missing',
      qty: 2,
      photoId: 102,
    })
  })

  // Null, not a partial body: a tick with no photo is exactly what the server
  // refuses, and she must be told here rather than after she leaves the room.
  it('refuses a body while one tick has no photo', () => {
    const { ticks, photos } = perfectRoom()
    const drafts = buildReportTicks(unbindPhotoTicks(ticks, 'photo-0'), photos)
    expect(reportTicksSubmission(drafts)).toBeNull()
  })

  it('refuses a body while a photo has not finished uploading', () => {
    const { ticks } = perfectRoom()
    const photos = REPORT_ZONES.map((zone, index) =>
      index === 0
        ? localPhoto('photo-0', zone.zone)
        : uploadedPhoto('photo-' + index, zone.zone, 100 + index)
    )
    expect(reportTicksSubmission(buildReportTicks(ticks, photos))).toBeNull()
  })

  it('refuses a body that is short of the 22', () => {
    const ticks = applyZoneCapture({}, 'bed', 'photo-0')
    const photos = [uploadedPhoto('photo-0', 'bed', 1)]
    expect(reportTicksSubmission(buildReportTicks(ticks, photos))).toBeNull()
  })

  // An extra shot of a zone is evidence and travels with the report.
  it('sends photos no tick names as extras', () => {
    const { ticks, photos } = perfectRoom()
    const withExtra = [...photos, uploadedPhoto('extra', 'bed', 999)]
    const drafts = buildReportTicks(ticks, withExtra)
    expect(reportExtraPhotoIds(drafts, withExtra)).toEqual([999])
    // First appearance order, which is REPORT_ITEMS' order: desk, bathroom,
    // general, bed — then the extra.
    expect(reportPhotoIds(drafts, [999])).toEqual([101, 102, 103, 100, 999])
  })

  it('counts one shared photo ONCE however many ticks name it', () => {
    const { ticks, photos } = perfectRoom()
    const drafts = buildReportTicks(ticks, photos)
    expect(reportPhotoIds(drafts)).toHaveLength(REPORT_ZONES.length)
  })
})

describe('tickDraftFromReport (the returned-report prefill)', () => {
  it('brings her ticks back with NO photos — re-sending rejected evidence is not a fix', () => {
    const draft = tickDraftFromReport({
      ticks: [
        { item: 'pillow', state: 'ok', qty: null, photoId: 5 },
        { item: 'tv_remote', state: 'damaged', qty: 2, photoId: 6 },
      ],
    })
    expect(draft.pillow).toEqual({ state: 'ok', qty: null, photo: null })
    expect(draft.tv_remote).toEqual({ state: 'damaged', qty: 2, photo: null })
  })

  // A legacy v1 report has no ticks at all: its exceptions come back, and the
  // items it never named individually stay untouched rather than being
  // invented as ครบ.
  it('falls back to a v1 report’s exceptions', () => {
    const draft = tickDraftFromReport({
      ticks: [],
      items: [{ item: 'kettle', problem: 'missing', qty: 3 }],
    })
    expect(draft.kettle).toEqual({ state: 'missing', qty: 3, photo: null })
    expect(draft.pillow).toBeUndefined()
  })

  it('drops codes and states this bundle does not know, and reads null as empty', () => {
    const draft = tickDraftFromReport({
      ticks: [
        { item: 'minibar', state: 'ok', photoId: 1 },
        { item: 'pillow', state: 'exploded', photoId: 1 },
      ],
    })
    expect(draft).toEqual({})
    expect(tickDraftFromReport(null)).toEqual({})
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
  function draft(overrides: Partial<{ roomStatus: string | null; extraPhotoIds: number[] }> = {}) {
    const { ticks, photos } = perfectRoom()
    return {
      roomStatus: 'vc' as string | null,
      ticks: buildReportTicks(ticks, photos),
      ...overrides,
    }
  }

  it('accepts a perfect room — four shots, twenty-two ticks', () => {
    expect(canSubmitReport(draft())).toBe(true)
  })

  it('refuses a report with no room status chosen', () => {
    expect(canSubmitReport(draft({ roomStatus: null }))).toBe(false)
  })

  it('refuses a room-status code the vocabulary does not carry', () => {
    expect(canSubmitReport(draft({ roomStatus: 'zz' }))).toBe(false)
  })

  // THE v2 rule: a tick without a picture is not evidence.
  it('refuses while any tick has lost its photo', () => {
    const { ticks, photos } = perfectRoom()
    expect(
      canSubmitReport({
        roomStatus: 'vc',
        ticks: buildReportTicks(unbindPhotoTicks(ticks, 'photo-2'), photos),
      })
    ).toBe(false)
  })

  it('refuses a checklist that is short of the 22', () => {
    const photos = [uploadedPhoto('photo-0', 'bed', 1)]
    expect(
      canSubmitReport({
        roomStatus: 'vc',
        ticks: buildReportTicks(applyZoneCapture({}, 'bed', 'photo-0'), photos),
      })
    ).toBe(false)
  })

  // The server bounds the DISTINCT photo count, not the tick count: one shot
  // per zone is the floor, and 24 is the ceiling.
  it('bounds the report’s distinct photos at one per zone .. 24', () => {
    expect(REPORT_MIN_PHOTOS_TOTAL).toBe(REPORT_ZONES.length)
    expect(REPORT_MAX_PHOTOS_TOTAL).toBe(24)
    expect(reportPhotoTotalValid(3)).toBe(false)
    expect(reportPhotoTotalValid(4)).toBe(true)
    expect(reportPhotoTotalValid(24)).toBe(true)
    expect(reportPhotoTotalValid(25)).toBe(false)
  })

  // A room shot with ONE picture for all four zones is under the floor even
  // though every tick is backed — the server would refuse it.
  it('refuses a whole room backed by a single photo', () => {
    let ticks: HkTickDraft = {}
    for (const zone of REPORT_ZONES) ticks = applyZoneCapture(ticks, zone.zone, 'one')
    const photos = [uploadedPhoto('one', 'bed', 7)]
    expect(canSubmitReport({ roomStatus: 'vc', ticks: buildReportTicks(ticks, photos) })).toBe(
      false
    )
  })

  it('accepts extras up to the ceiling and refuses past it', () => {
    const base = draft()
    const extras = Array.from({ length: 20 }, (_, i) => 500 + i)
    expect(canSubmitReport({ ...base, extraPhotoIds: extras })).toBe(true)
    expect(canSubmitReport({ ...base, extraPhotoIds: [...extras, 999] })).toBe(false)
  })
})

describe('reportPhotoCountValid', () => {
  // Reception's own evidence is still 1..4 — the tick model is about the
  // maid's side.
  it('bounds a verify’s photos at 1..4', () => {
    expect(reportPhotoCountValid(0)).toBe(false)
    expect(reportPhotoCountValid(1)).toBe(true)
    expect(reportPhotoCountValid(4)).toBe(true)
    expect(reportPhotoCountValid(5)).toBe(false)
    expect(REPORT_MAX_PHOTOS).toBe(4)
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

/**
 * The DOM half of the downscale — the half the HF Ville bug lived in.
 *
 * A maid's phone (old Android, LINE's in-app WebView) has no
 * `createImageBitmap`, so the old code handed the 3–7MB camera original
 * straight back and the 5MB client cap refused it before a single request left
 * the phone — zero failed uploads in the server log, "cannot attach a photo" on
 * the phone. What is asserted here is that EVERY WebView has a path that works,
 * and that the original is returned only when they have all been tried.
 */
describe('downscalePhoto', () => {
  type Slot = Record<string, unknown>
  const globals = globalThis as unknown as Slot
  const canvasProto = HTMLCanvasElement.prototype as unknown as Slot
  const urlGlobal = URL as unknown as Slot

  /** Every `drawImage` this test's canvas saw, as the size it was drawn at. */
  let drawn: Array<{ width: number; height: number }> = []
  /** Object URLs handed out and given back — a WebView that keeps a whole
   *  round's photos alive is the leak this guards. */
  let objectUrls: string[] = []
  let revokedUrls: string[] = []
  /** How many `<img>` elements the fallback path built. Zero proves the fast
   *  path was taken. */
  let imagesBuilt = 0

  const savedGlobals: Slot = {}
  const savedCanvas: Record<string, PropertyDescriptor | undefined> = {}

  function saveAll() {
    for (const key of ['createImageBitmap', 'Image']) {
      savedGlobals[key] = globals[key]
    }
    for (const key of ['getContext', 'toBlob', 'toDataURL']) {
      savedCanvas[key] = Object.getOwnPropertyDescriptor(canvasProto, key)
    }
    savedGlobals.createObjectURL = urlGlobal.createObjectURL
    savedGlobals.revokeObjectURL = urlGlobal.revokeObjectURL
  }

  function restoreAll() {
    for (const key of ['createImageBitmap', 'Image']) {
      if (savedGlobals[key] === undefined) delete globals[key]
      else globals[key] = savedGlobals[key]
    }
    for (const key of ['getContext', 'toBlob', 'toDataURL']) {
      const descriptor = savedCanvas[key]
      if (descriptor) Object.defineProperty(canvasProto, key, descriptor)
      else delete canvasProto[key]
    }
    if (savedGlobals.createObjectURL === undefined) delete urlGlobal.createObjectURL
    else urlGlobal.createObjectURL = savedGlobals.createObjectURL
    if (savedGlobals.revokeObjectURL === undefined) delete urlGlobal.revokeObjectURL
    else urlGlobal.revokeObjectURL = savedGlobals.revokeObjectURL
  }

  /** An 8MP camera original: the thing that has to come back smaller. */
  function cameraOriginal() {
    return new File([new Uint8Array(64)], 'IMG_0042.JPG', { type: 'image/jpeg' })
  }

  /** jsdom has no object URLs at all, so the `<img>` path needs them installed
   *  before it can even start. */
  function installObjectUrls() {
    urlGlobal.createObjectURL = (blob: Blob) => {
      const url = `blob:hk/${objectUrls.length}/${blob.size}`
      objectUrls.push(url)
      return url
    }
    urlGlobal.revokeObjectURL = (url: string) => {
      revokedUrls.push(url)
    }
  }

  /** The canvas: a 2D context that only records, plus whichever exporters this
   *  WebView is pretending to have. */
  function installCanvas({
    context = true,
    toBlob = true,
    toDataURL = 'data:image/jpeg;base64,' + btoa('resized-jpeg-bytes'),
  }: { context?: boolean; toBlob?: boolean; toDataURL?: string | false } = {}) {
    canvasProto.getContext = context
      ? () => ({
          drawImage: (_source: unknown, _x: number, _y: number, w: number, h: number) => {
            drawn.push({ width: w, height: h })
          },
        })
      : () => null
    if (toBlob) {
      canvasProto.toBlob = (cb: (blob: Blob | null) => void) => {
        cb(new Blob([new Uint8Array(8)], { type: 'image/jpeg' }))
      }
    } else {
      delete canvasProto.toBlob
    }
    if (toDataURL === false) delete canvasProto.toDataURL
    else canvasProto.toDataURL = () => toDataURL
  }

  /** `createImageBitmap`, as a modern WebView has it. */
  function installImageBitmap(
    result: { width: number; height: number } | 'reject' = { width: 4000, height: 3000 }
  ) {
    const calls: unknown[][] = []
    globals.createImageBitmap = (...args: unknown[]) => {
      calls.push(args)
      if (result === 'reject') return Promise.reject(new Error('no decoder'))
      return Promise.resolve({ ...result, close: () => undefined })
    }
    return calls
  }

  /** `new Image()` on a WebView that decodes (`ok`) or gives up (`onerror`). */
  function installImageElement({
    ok = true,
    width = 4000,
    height = 3000,
    decode = false,
    loadEvent = true,
  }: {
    ok?: boolean
    width?: number
    height?: number
    decode?: boolean
    /** false = a WebView that fires neither `load` nor `error`, so only
     *  `decode()` can say the picture arrived. */
    loadEvent?: boolean
  } = {}) {
    class FakeImage {
      onload: (() => void) | null = null
      onerror: (() => void) | null = null
      naturalWidth = ok ? width : 0
      naturalHeight = ok ? height : 0
      width = 0
      height = 0
      decode = decode ? () => Promise.resolve() : undefined
      private innerSrc = ''
      constructor() {
        imagesBuilt += 1
      }
      set src(value: string) {
        this.innerSrc = value
        if (!loadEvent) return
        setTimeout(() => (ok ? this.onload?.() : this.onerror?.()), 0)
      }
      get src() {
        return this.innerSrc
      }
    }
    globals.Image = FakeImage
  }

  beforeAll(saveAll)
  afterAll(restoreAll)

  beforeEach(() => {
    drawn = []
    objectUrls = []
    revokedUrls = []
    imagesBuilt = 0
    delete globals.createImageBitmap
    delete urlGlobal.createObjectURL
    delete urlGlobal.revokeObjectURL
  })

  afterEach(restoreAll)

  it('takes the createImageBitmap path when the WebView has one, EXIF-corrected', async () => {
    const calls = installImageBitmap()
    installImageElement()
    installObjectUrls()
    installCanvas()

    const file = cameraOriginal()
    const out = await downscalePhoto(file)

    expect(out).not.toBe(file)
    expect(out.type).toBe('image/jpeg')
    expect(calls).toHaveLength(1)
    // A portrait shot must not arrive sideways where the decoder can help.
    expect(calls[0][1]).toEqual({ imageOrientation: 'from-image' })
    expect(drawn).toEqual([{ width: 1600, height: 1200 }])
    // The fast path never touches an object URL or an <img>.
    expect(imagesBuilt).toBe(0)
    expect(objectUrls).toHaveLength(0)
  })

  // THE HF VILLE BUG. No `createImageBitmap` used to mean no downscale at all.
  it('falls back to <img> + canvas when createImageBitmap is missing', async () => {
    installImageElement()
    installObjectUrls()
    installCanvas()

    const file = cameraOriginal()
    const out = await downscalePhoto(file)

    expect(out).not.toBe(file)
    expect(out.type).toBe('image/jpeg')
    expect(imagesBuilt).toBe(1)
    expect(drawn).toEqual([{ width: 1600, height: 1200 }])
  })

  it('uses the <img> path when createImageBitmap exists but gives up', async () => {
    installImageBitmap('reject')
    installImageElement()
    installObjectUrls()
    installCanvas()

    const file = cameraOriginal()
    expect(await downscalePhoto(file)).not.toBe(file)
    expect(imagesBuilt).toBe(1)
  })

  // A WebView that fires no load event at all still gets its photo resized,
  // because decode() is asked too — and it is asked AFTER the src is set,
  // which is the only order in which it can resolve.
  it('accepts decode() as the decoder where no load event ever fires', async () => {
    installImageElement({ decode: true, loadEvent: false })
    installObjectUrls()
    installCanvas()

    const file = cameraOriginal()
    expect(await downscalePhoto(file)).not.toBe(file)
    expect(drawn).toEqual([{ width: 1600, height: 1200 }])
  })

  // Android WebViews before ~5.0 draw the canvas but cannot export a Blob.
  it('falls back to toDataURL when the canvas has no toBlob', async () => {
    installImageElement()
    installObjectUrls()
    installCanvas({ toBlob: false })

    const file = cameraOriginal()
    const out = await downscalePhoto(file)

    expect(out).not.toBe(file)
    expect(out.type).toBe('image/jpeg')
    expect(out.size).toBe('resized-jpeg-bytes'.length)
  })

  it('falls back to toDataURL when toBlob hands back nothing', async () => {
    installImageElement()
    installObjectUrls()
    installCanvas()
    canvasProto.toBlob = (cb: (blob: Blob | null) => void) => cb(null)

    const file = cameraOriginal()
    expect(await downscalePhoto(file)).not.toBe(file)
  })

  it('scales by the LONGEST edge on the <img> path too', async () => {
    installImageElement({ width: 3000, height: 4000 })
    installObjectUrls()
    installCanvas()

    await downscalePhoto(cameraOriginal())
    expect(drawn).toEqual([{ width: 1200, height: 1600 }])
  })

  it('honours a caller-supplied max', async () => {
    installImageElement()
    installObjectUrls()
    installCanvas()

    await downscalePhoto(cameraOriginal(), 800)
    expect(drawn).toEqual([{ width: 800, height: 600 }])
  })

  // A maid must always be able to file. The original is the LAST answer, never
  // an early one.
  it('returns the ORIGINAL only when every path has failed', async () => {
    installImageBitmap('reject')
    installImageElement({ ok: false })
    installObjectUrls()
    installCanvas()

    const file = cameraOriginal()
    expect(await downscalePhoto(file)).toBe(file)
    expect(imagesBuilt).toBe(1)
  })

  it('returns the original when there is no 2D context', async () => {
    installImageElement()
    installObjectUrls()
    installCanvas({ context: false })

    const file = cameraOriginal()
    expect(await downscalePhoto(file)).toBe(file)
  })

  it('returns the original when neither exporter exists', async () => {
    installImageElement()
    installObjectUrls()
    installCanvas({ toBlob: false, toDataURL: false })

    const file = cameraOriginal()
    expect(await downscalePhoto(file)).toBe(file)
  })

  // jsdom itself, and the oldest WebViews: no object URLs, so no <img> path.
  it('returns the original where there are no object URLs at all', async () => {
    installImageElement()
    installCanvas()

    const file = cameraOriginal()
    expect(await downscalePhoto(file)).toBe(file)
  })

  it('revokes the object URL it took — on success AND on failure', async () => {
    installImageElement()
    installObjectUrls()
    installCanvas()
    await downscalePhoto(cameraOriginal())
    expect(objectUrls).toHaveLength(1)
    expect(revokedUrls).toEqual(objectUrls)

    installImageElement({ ok: false })
    await downscalePhoto(cameraOriginal())
    expect(objectUrls).toHaveLength(2)
    expect(revokedUrls).toEqual(objectUrls)
  })

  it('never throws, whatever the runtime does', async () => {
    globals.createImageBitmap = () => {
      throw new Error('synchronous nonsense')
    }
    installImageElement()
    installObjectUrls()
    canvasProto.getContext = () => {
      throw new Error('no canvas here')
    }

    const file = cameraOriginal()
    await expect(downscalePhoto(file)).resolves.toBe(file)
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

    // The v2 body: TICKS, each photo-backed, and the extras that back nothing.
    it('posts the exact v2 body the contract names, to the room-scoped path', async () => {
      await submitHkReport('hfhotel', 7, {
        roomStatus: 'co',
        ticks: [
          { item: 'water_glass', state: 'ok', photoId: 11 },
          { item: 'bath_towel', state: 'missing', qty: 2, photoId: 12 },
        ],
        extraPhotoIds: [13],
      })
      expect(lastCall().url).toBe(`${HK_API_BASE}/rooms/7/report?branch=hfhotel`)
      expect(lastCall().init.method).toBe('POST')
      expect(body()).toEqual({
        roomStatus: 'co',
        ticks: [
          { item: 'water_glass', state: 'ok', photoId: 11 },
          { item: 'bath_towel', state: 'missing', qty: 2, photoId: 12 },
        ],
        extraPhotoIds: [13],
      })
    })

    // Append-only history: a fix POINTS AT what it supersedes.
    it('carries parentReportId when it is fixing a returned report', async () => {
      await submitHkReport('hfhotel', 7, {
        roomStatus: 'vc',
        ticks: [{ item: 'water_glass', state: 'ok', photoId: 11 }],
        parentReportId: 55,
      })
      expect(body().parentReportId).toBe(55)
      expect(body()).not.toHaveProperty('extraPhotoIds')
    })

    // 409 is an ANSWER, not a retry: the room already has a report today.
    it('turns a 409 into its own Thai copy, distinct from a write failure', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(409, { success: false }))
      await expect(
        submitHkReport('hfhotel', 7, {
          roomStatus: 'vc',
          ticks: [{ item: 'water_glass', state: 'ok', photoId: 1 }],
        })
      ).rejects.toThrow(/ส่งรายงานของวันนี้ไปแล้ว/)
    })

    it('treats a 200 carrying success: false as a failed write', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(200, { success: false }))
      await expect(
        submitHkReport('hfhotel', 7, {
          roomStatus: 'vc',
          ticks: [{ item: 'water_glass', state: 'ok', photoId: 1 }],
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
    it('posts multipart under "photo", carries the capture zone, and returns id + size', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(
        jsonResponse(200, { success: true, photoId: 31, bytes: 90_112 })
      )
      const blob = new Blob(['x'], { type: 'image/jpeg' })
      await expect(uploadHkReportPhoto('hfhotel', blob, { zone: 'bed' })).resolves.toEqual({
        photoId: 31,
        bytes: 90_112,
      })
      expect(lastCall().url).toBe(`${HK_API_BASE}/report-photos?branch=hfhotel`)
      const form = lastCall().init.body as FormData
      expect(form).toBeInstanceOf(FormData)
      expect(form.get('photo')).toBeTruthy()
      expect(form.get('zone')).toBe('bed')
    })

    it('sends no zone field at all when the caller has none', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(
        jsonResponse(200, { success: true, photoId: 31 })
      )
      await expect(
        uploadHkReportPhoto('hfhotel', new Blob(['x'], { type: 'image/jpeg' }))
      ).resolves.toEqual({ photoId: 31, bytes: null })
      expect((lastCall().init.body as FormData).get('zone')).toBeNull()
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

  // The retake primitive: uploader-only, and only while the photo is still
  // unattached — both enforced server-side.
  describe('deleteHkReportPhoto', () => {
    it('DELETEs the branch-scoped photo and resolves true', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(200, { success: true }))
      await expect(deleteHkReportPhoto('hfhotel', 31)).resolves.toBe(true)
      expect(lastCall().url).toBe(`${HK_API_BASE}/report-photos/31?branch=hfhotel`)
      expect(lastCall().init.method).toBe('DELETE')
    })

    // A photo that is already part of a filed report answers 400. That is not
    // an error the maid can act on — she has already moved on — so it resolves
    // false and the caller shrugs.
    it('resolves FALSE for a photo the server will not delete', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(404, { success: false }))
      await expect(deleteHkReportPhoto('hfhotel', 31)).resolves.toBe(false)
      ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(200, { success: false }))
      await expect(deleteHkReportPhoto('hfhotel', 31)).resolves.toBe(false)
    })
  })

  // What a restored draft asks before it trusts a photo id it remembers.
  describe('fetchHkReportPhotoMeta', () => {
    it('reads one photo’s metadata', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(
        jsonResponse(200, {
          success: true,
          photo: {
            photoId: 31,
            side: 'maid',
            zone: 'bed',
            bytes: 900,
            attached: false,
            uploadedAt: '2026-09-02T03:00:00.000Z',
          },
        })
      )
      await expect(fetchHkReportPhotoMeta('hfhotel', 31)).resolves.toMatchObject({
        photoId: 31,
        zone: 'bed',
        attached: false,
      })
      expect(lastCall().url).toBe(`${HK_API_BASE}/report-photos/31/meta?branch=hfhotel`)
    })

    // Null means "cannot be used": the draft drops it and unbinds its ticks,
    // which is kinder than a submit refused after she has left the room.
    it('resolves null for anything that is not a clean answer', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(404, { success: false }))
      await expect(fetchHkReportPhotoMeta('hfhotel', 31)).resolves.toBeNull()
      ;(global.fetch as jest.Mock).mockResolvedValue(jsonResponse(200, { success: true }))
      await expect(fetchHkReportPhotoMeta('hfhotel', 31)).resolves.toBeNull()
      ;(global.fetch as jest.Mock).mockRejectedValue(new Error('offline'))
      await expect(fetchHkReportPhotoMeta('hfhotel', 31)).resolves.toBeNull()
      await expect(fetchHkReportPhotoMeta(null, 31)).resolves.toBeNull()
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

// ---------------------------------------------------------------------------
// THE UPLOAD QUEUE. Pure reducer, so the retry arithmetic a maid depends on in
// a lift lobby is arithmetic a test can see.
// ---------------------------------------------------------------------------

describe('the upload queue', () => {
  it('adds a shot as queued, once', () => {
    let photos = reduceUploadQueue([], { type: 'add', key: 'a', zone: 'bed' })
    photos = reduceUploadQueue(photos, { type: 'add', key: 'a', zone: 'bed' })
    expect(photos).toHaveLength(1)
    expect(photos[0]).toMatchObject({ key: 'a', zone: 'bed', status: 'queued', attempts: 0 })
  })

  it('counts an attempt on start and lands with the id and the size', () => {
    let photos = reduceUploadQueue([], { type: 'add', key: 'a', zone: 'bed' })
    photos = reduceUploadQueue(photos, { type: 'start', key: 'a' })
    expect(photos[0]).toMatchObject({ status: 'uploading', attempts: 1 })
    photos = reduceUploadQueue(photos, { type: 'uploaded', key: 'a', photoId: 42, bytes: 900 })
    expect(photos[0]).toMatchObject({ status: 'uploaded', photoId: 42, bytes: 900 })
  })

  // One radio, one upload: four parallel 1600px JPEGs is how the whole queue
  // stalls together.
  it('sends one photo at a time', () => {
    let photos = reduceUploadQueue([], { type: 'add', key: 'a', zone: 'bed' })
    photos = reduceUploadQueue(photos, { type: 'add', key: 'b', zone: 'desk' })
    photos = reduceUploadQueue(photos, { type: 'start', key: 'a' })
    expect(nextUploadPhoto(photos, 0)).toBeNull()
  })

  it('backs off exponentially and comes back when the wait is up', () => {
    expect(uploadBackoffMs(1)).toBe(1000)
    expect(uploadBackoffMs(2)).toBe(2000)
    expect(uploadBackoffMs(5)).toBe(16000)
    expect(uploadBackoffMs(99)).toBe(30000)

    let photos = reduceUploadQueue([], { type: 'add', key: 'a', zone: 'bed' })
    photos = reduceUploadQueue(photos, { type: 'start', key: 'a' })
    photos = reduceUploadQueue(photos, { type: 'failed', key: 'a', at: 1000 })
    expect(nextUploadPhoto(photos, 1500)).toBeNull()
    expect(nextUploadWakeMs(photos, 1500)).toBe(500)
    expect(nextUploadPhoto(photos, 2000)?.key).toBe('a')
  })

  // Five attempts is where a silent loop stops eating her battery and starts
  // waiting for a deliberate tap.
  it('stops retrying after five attempts, and resumes on the tap', () => {
    let photos = reduceUploadQueue([], { type: 'add', key: 'a', zone: 'bed' })
    for (let i = 0; i < REPORT_UPLOAD_MAX_ATTEMPTS; i += 1) {
      photos = reduceUploadQueue(photos, { type: 'start', key: 'a' })
      photos = reduceUploadQueue(photos, { type: 'failed', key: 'a', at: 0 })
    }
    expect(photos[0].attempts).toBe(REPORT_UPLOAD_MAX_ATTEMPTS)
    expect(nextUploadPhoto(photos, 10_000_000)).toBeNull()
    expect(nextUploadWakeMs(photos, 10_000_000)).toBeNull()
    expect(uploadCounts(photos).stuck).toBe(1)

    // The tap puts it back in the queue and it goes IMMEDIATELY — making her
    // wait out another backoff for the retry she just asked for is not a retry.
    photos = reduceUploadQueue(photos, { type: 'resume' })
    expect(photos[0]).toMatchObject({ status: 'queued', attempts: 0 })
    expect(nextUploadPhoto(photos, 0)?.key).toBe('a')
  })

  it('drops a removed photo, and ignores an action for a key it no longer holds', () => {
    let photos = reduceUploadQueue([], { type: 'add', key: 'a', zone: 'bed' })
    photos = reduceUploadQueue(photos, { type: 'remove', key: 'a' })
    expect(photos).toEqual([])
    expect(reduceUploadQueue(photos, { type: 'uploaded', key: 'a', photoId: 1 })).toEqual([])
  })

  it('says how far along it is, and when it is done', () => {
    let photos = reduceUploadQueue([], { type: 'add', key: 'a', zone: 'bed' })
    photos = reduceUploadQueue(photos, { type: 'add', key: 'b', zone: 'desk' })
    expect(uploadProgressLabel(photos)).toBe('อัปโหลดแล้ว 0/2')
    expect(uploadsSettled(photos)).toBe(false)
    photos = reduceUploadQueue(photos, { type: 'uploaded', key: 'a', photoId: 1 })
    expect(uploadProgressLabel(photos)).toBe('อัปโหลดแล้ว 1/2')
    photos = reduceUploadQueue(photos, { type: 'uploaded', key: 'b', photoId: 2 })
    expect(uploadsSettled(photos)).toBe(true)
    expect(uploadCounts(photos)).toEqual({ total: 2, uploaded: 2, pending: 0, stuck: 0 })
  })

  it('never mutates the array it was given', () => {
    const photos = reduceUploadQueue([], { type: 'add', key: 'a', zone: 'bed' })
    const snapshot = JSON.parse(JSON.stringify(photos))
    reduceUploadQueue(photos, { type: 'start', key: 'a' })
    expect(photos).toEqual(snapshot)
  })
})

// ---------------------------------------------------------------------------
// THE DRAFT — the phone that locks mid-room is the design case.
// ---------------------------------------------------------------------------

describe('the report draft', () => {
  beforeEach(() => sessionStorage.clear())

  it('keys by branch, room and day — 7 is a different room at each hotel', () => {
    expect(reportDraftKey('hfhotel', 7, '2026-09-02')).toBe('hk.reportDraft.hfhotel.7.2026-09-02')
    expect(reportDraftKey('hfville', 7, '2026-09-02')).not.toBe(
      reportDraftKey('hfhotel', 7, '2026-09-02')
    )
  })

  it('round-trips a half-filled room', () => {
    const { ticks, photos } = perfectRoom()
    writeReportDraft('hfhotel', 7, '2026-09-02', {
      roomStatus: 'co',
      step: 2,
      ticks,
      photos,
      seq: 4,
    })
    const back = readReportDraft('hfhotel', 7, '2026-09-02')
    expect(back).toMatchObject({ roomStatus: 'co', step: 2, seq: 4 })
    expect(back?.photos).toHaveLength(4)
    expect(back?.ticks.pillow).toEqual({ state: 'ok', qty: null, photo: 'photo-0' })
  })

  it('reads nothing, junk and another bundle’s shape as no draft', () => {
    expect(readReportDraft('hfhotel', 7, '2026-09-02')).toBeNull()
    sessionStorage.setItem('hk.reportDraft.hfhotel.7.2026-09-02', 'not json')
    expect(readReportDraft('hfhotel', 7, '2026-09-02')).toBeNull()
    sessionStorage.setItem('hk.reportDraft.hfhotel.7.2026-09-02', '{"step":1}')
    expect(readReportDraft('hfhotel', 7, '2026-09-02')).toBeNull()
  })

  it('is cleared by a landed submit', () => {
    const { ticks, photos } = perfectRoom()
    writeReportDraft('hfhotel', 7, '2026-09-02', {
      roomStatus: 'vc',
      step: 4,
      ticks,
      photos,
      seq: 4,
    })
    clearReportDraft('hfhotel', 7, '2026-09-02')
    expect(readReportDraft('hfhotel', 7, '2026-09-02')).toBeNull()
  })

  // The reconciliation: a photo id she remembers is only usable while the
  // server still says it is hers and unattached.
  it('drops photos the server no longer offers and UNBINDS their ticks', () => {
    const { ticks, photos } = perfectRoom()
    const draft = { roomStatus: 'vc' as RoomStatusCode, step: 1, ticks, photos, seq: 4 }
    const reconciled = reconcileReportDraft(draft, [101, 102, 103])
    expect(reconciled.photos.map((p) => p.photoId)).toEqual([101, 102, 103])
    // The bed shot (100) is gone: its five ticks survive, without evidence.
    expect(reconciled.ticks.pillow).toEqual({ state: 'ok', qty: null, photo: null })
    expect(reconciled.ticks.water_glass.photo).toBe('photo-1')
    expect(Object.keys(reconciled.ticks)).toHaveLength(REPORT_ITEMS.length)
  })

  it('drops a photo that never got an id — its bytes died with the page', () => {
    let ticks = applyZoneCapture({}, 'bed', 'photo-0')
    ticks = cycleTickState(ticks, 'pillow')
    const draft = {
      roomStatus: 'vc' as RoomStatusCode,
      step: 0,
      ticks,
      photos: [localPhoto('photo-0', 'bed')],
      seq: 1,
    }
    const reconciled = reconcileReportDraft(draft, [])
    expect(reconciled.photos).toEqual([])
    // Her judgement survives; only the picture is owed.
    expect(reconciled.ticks.pillow).toEqual({ state: 'missing', qty: 1, photo: null })
  })
})

// ---------------------------------------------------------------------------
// Reading a filed report — v2's ticks, and v1's exceptions, from one screen.
// ---------------------------------------------------------------------------

describe('reading ticks off a filed report', () => {
  const ticks = [
    { item: 'pillow', state: 'ok', qty: null, photoId: 10 },
    { item: 'bath_towel', state: 'missing', qty: 2, photoId: 12 },
    { item: 'water_glass', state: 'ok', qty: null, photoId: 11 },
  ]

  it('renders rows in the paper form’s order with Thai labels', () => {
    const rows = reportTickRows(ticks)
    expect(rows.map((r) => r.item)).toEqual(['water_glass', 'bath_towel', 'pillow'])
    expect(rows[1]).toMatchObject({
      label: 'ผ้าขนหนู (รวมสีฟ้า)',
      stateLabel: 'หาย',
      qty: 2,
      problem: true,
      zoneLabel: 'ห้องน้ำ',
    })
  })

  it('groups them by capture zone, dropping the zones a report does not touch', () => {
    const groups = reportTicksByZone(ticks)
    expect(groups.map((g) => g.zone)).toEqual(['bed', 'desk', 'bathroom'])
    expect(groups[0].ticks.map((t) => t.item)).toEqual(['pillow'])
  })

  // A newer backend's 23rd item must render, not vanish.
  it('keeps an unknown item under อื่น ๆ', () => {
    const groups = reportTicksByZone([{ item: 'minibar', state: 'missing', qty: 1, photoId: 1 }])
    expect(groups.map((g) => g.label)).toEqual(['อื่น ๆ'])
    expect(groups[0].ticks[0].label).toBe('minibar')
  })

  it('reads an absent tick list as no rows at all', () => {
    expect(reportTickRows(null)).toEqual([])
    expect(reportTicksByZone(undefined)).toEqual([])
  })
})

describe('reportPhotoGroups (the verify view’s layout)', () => {
  const photos = [
    { photoId: 10, side: 'maid', zone: 'bed', bytes: 900 },
    { photoId: 12, side: 'maid', zone: 'bathroom', bytes: 800 },
    { photoId: 20, side: 'reception', zone: null, bytes: 700 },
  ]
  const ticks = [
    { item: 'pillow', state: 'ok', qty: null, photoId: 10 },
    { item: 'duvet', state: 'ok', qty: null, photoId: 10 },
    { item: 'bath_towel', state: 'missing', qty: 2, photoId: 12 },
  ]

  it('groups ONE side’s photos by zone, each with the items it backs', () => {
    const groups = reportPhotoGroups(photos, ticks, 'maid')
    expect(groups.map((g) => g.zone)).toEqual(['bed', 'bathroom'])
    expect(groups[0].photos[0].photoId).toBe(10)
    // One photo backing several ticks is the point of the shared-photo rule.
    expect(groups[0].photos[0].ticks.map((t) => t.item)).toEqual(['duvet', 'pillow'])
    expect(groups[1].photos[0].ticks[0]).toMatchObject({ problem: true, qty: 2 })
  })

  it('never shows the other side’s photos', () => {
    expect(reportPhotoGroups(photos, ticks, 'reception').map((g) => g.label)).toEqual(['อื่น ๆ'])
  })

  it('still shows a photo that backs nothing — an extra shot is evidence', () => {
    const groups = reportPhotoGroups(
      [{ photoId: 31, side: 'maid', zone: 'bed', bytes: null }],
      [],
      'maid'
    )
    expect(groups[0].photos[0].ticks).toEqual([])
  })
})

describe('reportProblemCount / reportAllOk / reportSidePhotoIds', () => {
  it('prefers the server’s count, then the ticks, then v1’s exceptions', () => {
    expect(reportProblemCount({ reportId: 1, roomId: 1, status: 'submitted', problemCount: 3 })).toBe(3)
    expect(
      reportProblemCount({
        reportId: 1,
        roomId: 1,
        status: 'submitted',
        ticks: [
          { item: 'pillow', state: 'ok' },
          { item: 'kettle', state: 'damaged', qty: 1 },
        ],
      })
    ).toBe(1)
    expect(
      reportProblemCount({
        reportId: 1,
        roomId: 1,
        status: 'submitted',
        items: [{ item: 'kettle', problem: 'missing', qty: 1 }],
      })
    ).toBe(1)
    expect(reportProblemCount(null)).toBe(0)
  })

  // The one failure here that could cost a guest a charge nobody can explain:
  // "ครบทุกรายการ" printed over a list of missing things.
  it('lets the ticks — then the rows — beat the flag', () => {
    expect(
      reportAllOk({
        reportId: 1,
        roomId: 1,
        status: 'submitted',
        allItemsOk: true,
        ticks: [{ item: 'kettle', state: 'missing', qty: 1, photoId: 1 }],
      })
    ).toBe(false)
    expect(
      reportAllOk({
        reportId: 1,
        roomId: 1,
        status: 'submitted',
        allItemsOk: true,
        items: [{ item: 'kettle', problem: 'missing', qty: 1 }],
      })
    ).toBe(false)
    expect(reportAllOk({ reportId: 1, roomId: 1, status: 'submitted', allItemsOk: true })).toBe(true)
  })

  it('reads a side’s photos from the metadata, falling back to v1’s id arrays', () => {
    expect(
      reportSidePhotoIds(
        {
          reportId: 1,
          roomId: 1,
          status: 'submitted',
          photos: [
            { photoId: 5, side: 'maid' },
            { photoId: 6, side: 'reception' },
          ],
        },
        'maid'
      )
    ).toEqual([5])
    expect(
      reportSidePhotoIds(
        { reportId: 1, roomId: 1, status: 'submitted', maidPhotoIds: [31, 32] },
        'maid'
      )
    ).toEqual([31, 32])
  })
})
