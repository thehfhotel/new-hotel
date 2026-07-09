/**
 * lib/v2/spatial-grid — placement + guest-move drag eligibility
 * (ADR 0003 U1 / #225 #226).
 *
 * Pinned here:
 * - `computeSpatialLayout` quantizes raw iHOTEL Room_X/Room_y pixel values
 *   into CSS-grid bands (jitter within a column/row collapses; distinct
 *   columns/rows stay distinct).
 * - Rooms with missing coords or (0,0) fall into the `unplaced` bucket.
 * - `moveEligibility` — vacant ok, vacant-dirty warn, everything else blocked;
 *   only an occupied source may move.
 */

import {
  computeSpatialLayout,
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
