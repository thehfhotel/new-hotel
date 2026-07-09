/**
 * lib/v2/spatial-grid — placement + guest-move drag eligibility
 * (ADR 0003 U1 / #225 #226) + layout-edit pixel derivation (#236).
 *
 * Pinned here:
 * - `computeSpatialLayout` quantizes raw iHOTEL Room_X/Room_y pixel values
 *   into CSS-grid bands (jitter within a column/row collapses; distinct
 *   columns/rows stay distinct).
 * - Rooms with missing coords or (0,0) fall into the `unplaced` bucket.
 * - `moveEligibility` — vacant ok, vacant-dirty warn, everything else blocked;
 *   only an occupied source may move.
 * - `deriveBoardPixels` round-trip invariant (#236 decision 2): the derived
 *   pixel pair re-lands the moved room in EXACTLY the intended cell through
 *   `computeSpatialLayout`, moves NO other tile, never chain-merges bands
 *   (neighbor value reuse, no midpoints), and never emits the (0,0) sentinel.
 */

import {
  computeSpatialLayout,
  deriveBoardPixels,
  isUnplaced,
  moveEligibility,
  type SpatialRoom,
} from '@/lib/v2/spatial-grid'

function room(partial: Partial<SpatialRoom> & { id: number }): SpatialRoom {
  return {
    roomNo: String(partial.id),
    roomTypeName: 'Standard',
    floor: 1,
    status: 'available',
    isClean: true,
    isMaintenance: false,
    ...partial,
  } as SpatialRoom
}

describe('isUnplaced', () => {
  it('treats missing coords as unplaced', () => {
    expect(isUnplaced({ roomX: null, roomY: null })).toBe(true)
    expect(isUnplaced({ roomX: undefined, roomY: undefined })).toBe(true)
    expect(isUnplaced({ roomX: 120, roomY: null })).toBe(true)
  })

  it('treats (0,0) as unplaced (legacy default)', () => {
    expect(isUnplaced({ roomX: 0, roomY: 0 })).toBe(true)
  })

  it('treats a real position as placed — including a 0 on one axis only', () => {
    expect(isUnplaced({ roomX: 120, roomY: 40 })).toBe(false)
    expect(isUnplaced({ roomX: 0, roomY: 120 })).toBe(false)
  })
})

describe('computeSpatialLayout', () => {
  it('maps distinct coordinate bands to grid columns/rows in order', () => {
    const layout = computeSpatialLayout([
      room({ id: 1, roomX: 10, roomY: 10 }),
      room({ id: 2, roomX: 120, roomY: 10 }),
      room({ id: 3, roomX: 10, roomY: 130 }),
    ])
    const byId = new Map(layout.placed.map((p) => [p.room.id, p]))
    expect(byId.get(1)).toMatchObject({ col: 1, row: 1 })
    expect(byId.get(2)).toMatchObject({ col: 2, row: 1 })
    expect(byId.get(3)).toMatchObject({ col: 1, row: 2 })
    expect(layout.cols).toBe(2)
    expect(layout.rows).toBe(2)
    expect(layout.unplaced).toHaveLength(0)
  })

  it('collapses pixel jitter within one visual column (WinForms free drag)', () => {
    const layout = computeSpatialLayout([
      room({ id: 1, roomX: 100, roomY: 10 }),
      room({ id: 2, roomX: 112, roomY: 130 }), // same column, off by 12px
      room({ id: 3, roomX: 230, roomY: 10 }),
    ])
    const byId = new Map(layout.placed.map((p) => [p.room.id, p]))
    expect(byId.get(1)!.col).toBe(byId.get(2)!.col)
    expect(byId.get(3)!.col).toBe(byId.get(1)!.col + 1)
  })

  it('sends (0,0) and missing-coord rooms to the unplaced bucket', () => {
    const layout = computeSpatialLayout([
      room({ id: 1, roomX: 100, roomY: 100 }),
      room({ id: 2, roomX: 0, roomY: 0 }),
      room({ id: 3 }), // no coords in payload at all
    ])
    expect(layout.placed.map((p) => p.room.id)).toEqual([1])
    expect(layout.unplaced.map((r) => r.id).sort()).toEqual([2, 3])
  })

  it('handles an all-unplaced set without producing a board', () => {
    const layout = computeSpatialLayout([room({ id: 1 }), room({ id: 2 })])
    expect(layout.placed).toHaveLength(0)
    expect(layout.cols).toBe(0)
    expect(layout.unplaced).toHaveLength(2)
  })
})

describe('deriveBoardPixels (#236 จัดผัง — round-trip invariant)', () => {
  /** Realistic synthetic board — WinForms pixel jitter inside every band,
   *  3 columns × 3 rows with holes at (2,3), (3,2), (3,3). */
  const BOARD: SpatialRoom[] = [
    room({ id: 1, roomX: 10, roomY: 12 }),
    room({ id: 2, roomX: 118, roomY: 10 }),
    room({ id: 3, roomX: 231, roomY: 8 }),
    room({ id: 4, roomX: 14, roomY: 130 }),
    room({ id: 5, roomX: 122, roomY: 128 }),
    room({ id: 6, roomX: 8, roomY: 262 }),
  ]
  const MOVER = 99

  it('PROPERTY: every cell target — incl. one band beyond each edge — lands the room in exactly that cell and moves nothing else', () => {
    const base = computeSpatialLayout(BOARD)
    expect(base.cols).toBe(3)
    expect(base.rows).toBe(3)
    for (let col = 1; col <= base.cols + 1; col += 1) {
      for (let row = 1; row <= base.rows + 1; row += 1) {
        const { x, y } = deriveBoardPixels(BOARD, { col, row })
        const after = computeSpatialLayout([
          ...BOARD,
          room({ id: MOVER, roomX: x, roomY: y }),
        ])
        const byId = new Map(after.placed.map((p) => [p.room.id, p]))
        expect(byId.get(MOVER)).toMatchObject({ col, row })
        // Every pre-existing tile keeps its exact cell (no band merge/shift).
        for (const p of base.placed) {
          expect(byId.get(p.room.id)).toMatchObject({ col: p.col, row: p.row })
        }
      }
    }
  })

  it('moving an EXISTING room to another cell round-trips without disturbing the rest', () => {
    // Move room 5 from (2,2) to the empty (3,2).
    const { x, y } = deriveBoardPixels(BOARD, { col: 3, row: 2 })
    const after = computeSpatialLayout(
      BOARD.map((r) => (r.id === 5 ? { ...r, roomX: x, roomY: y } : r)),
    )
    const byId = new Map(after.placed.map((p) => [p.room.id, p]))
    expect(byId.get(5)).toMatchObject({ col: 3, row: 2 })
    expect(byId.get(1)).toMatchObject({ col: 1, row: 1 })
    expect(byId.get(2)).toMatchObject({ col: 2, row: 1 })
    expect(byId.get(3)).toMatchObject({ col: 3, row: 1 })
    expect(byId.get(4)).toMatchObject({ col: 1, row: 2 })
    expect(byId.get(6)).toMatchObject({ col: 1, row: 3 })
  })

  it('reuses a NEIGHBOR raw value verbatim — never a midpoint that could chain-merge bands', () => {
    // Column band 1 is chain-transitive: 100↔135↔170 (each gap 35 ≤ 40) even
    // though 100↔170 differ by 70. A midpoint between band 1 and band 2
    // (250) could bridge them; exact reuse cannot.
    const chained: SpatialRoom[] = [
      room({ id: 1, roomX: 100, roomY: 10 }),
      room({ id: 2, roomX: 135, roomY: 10 }),
      room({ id: 3, roomX: 170, roomY: 10 }),
      room({ id: 4, roomX: 250, roomY: 10 }),
    ]
    expect([100, 135, 170]).toContain(deriveBoardPixels(chained, { col: 1, row: 1 }).x)
    expect(deriveBoardPixels(chained, { col: 2, row: 1 }).x).toBe(250)
    // Extrapolated third column sits a full pitch beyond the outermost value
    // — strictly more than the 40px tolerance, so it forms its own band.
    const x3 = deriveBoardPixels(chained, { col: 3, row: 1 }).x
    expect(x3 - 250).toBeGreaterThan(40)
    const after = computeSpatialLayout([
      ...chained,
      room({ id: 9, roomX: x3, roomY: 10 }),
    ])
    const byId = new Map(after.placed.map((p) => [p.room.id, p]))
    expect(byId.get(9)!.col).toBe(3)
    expect(byId.get(1)!.col).toBe(1)
    expect(byId.get(4)!.col).toBe(2)
  })

  it('a swap needs no derivation: exchanging pairs verbatim exchanges exactly the two cells', () => {
    const swapped = BOARD.map((r) => {
      if (r.id === 2) return { ...r, roomX: BOARD[4].roomX, roomY: BOARD[4].roomY } // takes 5's pair
      if (r.id === 5) return { ...r, roomX: BOARD[1].roomX, roomY: BOARD[1].roomY } // takes 2's pair
      return r
    })
    const byId = new Map(computeSpatialLayout(swapped).placed.map((p) => [p.room.id, p]))
    expect(byId.get(2)).toMatchObject({ col: 2, row: 2 }) // was (2,1)
    expect(byId.get(5)).toMatchObject({ col: 2, row: 1 }) // was (2,2)
    expect(byId.get(1)).toMatchObject({ col: 1, row: 1 }) // bystander untouched
  })

  it('never derives the (0,0) unplaced sentinel — single-axis zeros nudge in-band', () => {
    // Both band-1 exemplars are literally 0 → naive reuse would produce the
    // sentinel and silently unplace the room.
    const zeroCorner: SpatialRoom[] = [
      room({ id: 1, roomX: 0, roomY: 120 }),
      room({ id: 2, roomX: 120, roomY: 0 }),
    ]
    const { x, y } = deriveBoardPixels(zeroCorner, { col: 1, row: 1 })
    expect(x === 0 && y === 0).toBe(false)
    expect(isUnplaced({ roomX: x, roomY: y })).toBe(false)
    const after = computeSpatialLayout([
      ...zeroCorner,
      room({ id: 9, roomX: x, roomY: y }),
    ])
    const byId = new Map(after.placed.map((p) => [p.room.id, p]))
    expect(byId.get(9)).toMatchObject({ col: 1, row: 1 })
    // The nudge must not merge bands: the originals keep their cells.
    expect(byId.get(1)).toMatchObject({ col: 1, row: 2 })
    expect(byId.get(2)).toMatchObject({ col: 2, row: 1 })
  })

  it('seeds an empty board with a nonzero-pair origin that lands at (1,1)', () => {
    const { x, y } = deriveBoardPixels([], { col: 1, row: 1 })
    expect(isUnplaced({ roomX: x, roomY: y })).toBe(false)
    const after = computeSpatialLayout([room({ id: 9, roomX: x, roomY: y })])
    expect(after.placed[0]).toMatchObject({ col: 1, row: 1 })
    // An all-unplaced list behaves like an empty board (no bands to reuse).
    const unplacedOnly = [room({ id: 1 }), room({ id: 2, roomX: 0, roomY: 0 })]
    expect(deriveBoardPixels(unplacedOnly, { col: 1, row: 1 })).toEqual({ x, y })
  })
})

describe('moveEligibility (#225 decision comment)', () => {
  const source = room({ id: 1, status: 'occupied' })

  it('vacant-clean target → ok', () => {
    expect(moveEligibility(source, room({ id: 2, status: 'available', isClean: true }))).toBe('ok')
  })

  it('vacant-dirty target → warn (allowed but visually warned)', () => {
    expect(moveEligibility(source, room({ id: 2, status: 'available', isClean: false }))).toBe('warn')
  })

  it('maintenance target → blocked (both the status and the flag)', () => {
    expect(moveEligibility(source, room({ id: 2, status: 'maintenance' }))).toBe('blocked')
    expect(
      moveEligibility(source, room({ id: 2, status: 'available', isMaintenance: true })),
    ).toBe('blocked')
  })

  it('occupied / booked / checkout_pending targets → blocked', () => {
    for (const status of ['occupied', 'booked', 'checkout_pending']) {
      expect(moveEligibility(source, room({ id: 2, status }))).toBe('blocked')
    }
  })

  it('self-drop and non-occupied sources → blocked', () => {
    expect(moveEligibility(source, room({ id: 1, status: 'available' }))).toBe('blocked')
    expect(
      moveEligibility(room({ id: 3, status: 'available' }), room({ id: 2, status: 'available' })),
    ).toBe('blocked')
  })
})
