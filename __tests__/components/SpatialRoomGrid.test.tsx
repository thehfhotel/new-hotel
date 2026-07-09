/**
 * @jest-environment jsdom
 *
 * v2 spatial room grid (ADR 0003 U1 / #225 #226).
 *
 * Pinned here:
 * - Tiles render at their quantized roomX/roomY grid positions; rooms with
 *   (0,0)/missing coords render in the "ยังไม่กำหนดตำแหน่ง" row instead.
 * - A tap (no drag) selects the room (same action-sheet flow as the floor
 *   view).
 * - Guest-move drag: dragging an occupied tile onto a vacant tile fires
 *   `onMoveRequest`; a maintenance tile never does; `canDrag=false`
 *   (read-only branch) disables the gesture entirely.
 * - Layout-edit mode (#236 จัดผัง, `mode='layout'`): EVERY tile drags (any
 *   status, incl. the unplaced row), tap-select is suppressed, drop on a
 *   placed tile fires a swap, drop on a synthetic empty lattice cell fires a
 *   place, and guest-move defaults stay untouched when the prop is absent.
 */

import { fireEvent, render, screen } from '@testing-library/react'
import SpatialRoomGrid from '@/components/v2/SpatialRoomGrid'
import type { SpatialRoom } from '@/lib/v2/spatial-grid'

function room(partial: Partial<SpatialRoom> & { id: number; roomNo: string }): SpatialRoom {
  return {
    roomTypeName: 'Standard',
    floor: 1,
    status: 'available',
    isClean: true,
    isMaintenance: false,
    ...partial,
  } as SpatialRoom
}

const ROOMS: SpatialRoom[] = [
  room({ id: 1, roomNo: '101', status: 'occupied', roomX: 10, roomY: 10 }),
  room({ id: 2, roomNo: '102', status: 'available', roomX: 120, roomY: 10 }),
  room({ id: 3, roomNo: '103', status: 'maintenance', roomX: 230, roomY: 10 }),
  room({ id: 4, roomNo: '201', status: 'available', roomX: 10, roomY: 140 }),
  room({ id: 5, roomNo: 'X99', status: 'available' }), // no coords → unplaced
]

function tile(roomId: number): HTMLElement {
  const el = document.querySelector(`[data-room-id="${roomId}"]`)
  if (!el) throw new Error(`tile ${roomId} not rendered`)
  return el as HTMLElement
}

/** Simulate a pointer drag from one tile to another. jsdom has no layout, so
 *  document.elementFromPoint is mocked to return the drop target. */
function dragTile(source: HTMLElement, target: HTMLElement | null) {
  document.elementFromPoint = jest.fn(() => target) as typeof document.elementFromPoint
  fireEvent.pointerDown(source, { clientX: 10, clientY: 10, pointerId: 1, button: 0 })
  // First move crosses the 8px threshold and starts the drag …
  fireEvent.pointerMove(source, { clientX: 40, clientY: 12, pointerId: 1 })
  // … second move hit-tests the drop target under the pointer.
  fireEvent.pointerMove(source, { clientX: 120, clientY: 14, pointerId: 1 })
  fireEvent.pointerUp(source, { clientX: 120, clientY: 14, pointerId: 1 })
}

describe('SpatialRoomGrid — placement (#226)', () => {
  it('renders placed tiles on the board and coordless rooms in the unplaced row', () => {
    render(
      <SpatialRoomGrid rooms={ROOMS} onSelect={jest.fn()} onMoveRequest={jest.fn()} canDrag />,
    )

    const board = screen.getByTestId('spatial-board')
    const unplaced = screen.getByTestId('spatial-unplaced')

    // 4 positioned rooms on the board, 1 in the unplaced row.
    expect(board.querySelectorAll('[data-room-id]')).toHaveLength(4)
    expect(unplaced.querySelectorAll('[data-room-id]')).toHaveLength(1)
    expect(screen.getByText('ยังไม่กำหนดตำแหน่ง')).toBeInTheDocument()
    expect(screen.getByText('X99')).toBeInTheDocument()

    // Quantized positions: 101 top-left, 102 next column, 201 next row.
    const pos = (id: number) => [tile(id).getAttribute('data-col'), tile(id).getAttribute('data-row')]
    expect(pos(1)).toEqual(['1', '1'])
    expect(pos(2)).toEqual(['2', '1'])
    expect(pos(3)).toEqual(['3', '1'])
    expect(pos(4)).toEqual(['1', '2'])
  })

  it('omits the unplaced row when every room has a position', () => {
    render(
      <SpatialRoomGrid
        rooms={ROOMS.slice(0, 4)}
        onSelect={jest.fn()}
        onMoveRequest={jest.fn()}
        canDrag
      />,
    )
    expect(screen.queryByTestId('spatial-unplaced')).not.toBeInTheDocument()
  })

  it('shows the empty state when no rooms match', () => {
    render(<SpatialRoomGrid rooms={[]} onSelect={jest.fn()} onMoveRequest={jest.fn()} canDrag />)
    expect(screen.getByText('ไม่พบห้องที่ตรงกับเงื่อนไข')).toBeInTheDocument()
  })
})

describe('SpatialRoomGrid — tap vs guest-move drag (#225)', () => {
  it('a plain tap selects the room', () => {
    const onSelect = jest.fn()
    render(
      <SpatialRoomGrid rooms={ROOMS} onSelect={onSelect} onMoveRequest={jest.fn()} canDrag />,
    )
    fireEvent.click(tile(2))
    expect(onSelect).toHaveBeenCalledWith(expect.objectContaining({ id: 2, roomNo: '102' }))
  })

  it('dropping an occupied tile on a vacant tile fires onMoveRequest (and not onSelect)', () => {
    const onSelect = jest.fn()
    const onMoveRequest = jest.fn()
    render(
      <SpatialRoomGrid rooms={ROOMS} onSelect={onSelect} onMoveRequest={onMoveRequest} canDrag />,
    )

    dragTile(tile(1), tile(2))

    expect(onMoveRequest).toHaveBeenCalledTimes(1)
    const [from, to] = onMoveRequest.mock.calls[0]
    expect(from.id).toBe(1)
    expect(to.id).toBe(2)

    // The click that follows the drag must NOT open the action sheet.
    fireEvent.click(tile(1))
    expect(onSelect).not.toHaveBeenCalled()
  })

  it('dropping on a maintenance tile is blocked', () => {
    const onMoveRequest = jest.fn()
    render(
      <SpatialRoomGrid rooms={ROOMS} onSelect={jest.fn()} onMoveRequest={onMoveRequest} canDrag />,
    )
    dragTile(tile(1), tile(3))
    expect(onMoveRequest).not.toHaveBeenCalled()
  })

  it('marks drop candidates during a drag: vacant ok, dirty warn, maintenance blocked', () => {
    const dirtyRooms = ROOMS.map((r) =>
      r.id === 4 ? { ...r, isClean: false } : r,
    )
    render(
      <SpatialRoomGrid
        rooms={dirtyRooms}
        onSelect={jest.fn()}
        onMoveRequest={jest.fn()}
        canDrag
      />,
    )

    // Start a drag but don't drop.
    document.elementFromPoint = jest.fn(() => null) as typeof document.elementFromPoint
    fireEvent.pointerDown(tile(1), { clientX: 10, clientY: 10, pointerId: 1, button: 0 })
    fireEvent.pointerMove(tile(1), { clientX: 40, clientY: 12, pointerId: 1 })

    expect(tile(2).dataset.elig).toBe('ok') // vacant-clean
    expect(tile(3).dataset.elig).toBe('blocked') // maintenance
    expect(tile(4).dataset.elig).toBe('warn') // vacant-dirty
  })

  it('never starts a drag when canDrag is false (read-only branch)', () => {
    const onMoveRequest = jest.fn()
    render(
      <SpatialRoomGrid
        rooms={ROOMS}
        onSelect={jest.fn()}
        onMoveRequest={onMoveRequest}
        canDrag={false}
      />,
    )
    dragTile(tile(1), tile(2))
    expect(onMoveRequest).not.toHaveBeenCalled()
  })

  it('renders NO synthetic lattice cells in guest mode (perf: 58-tile board stays 58 nodes)', () => {
    render(
      <SpatialRoomGrid rooms={ROOMS} onSelect={jest.fn()} onMoveRequest={jest.fn()} canDrag />,
    )
    expect(document.querySelectorAll('[data-cell]')).toHaveLength(0)
  })
})

describe('SpatialRoomGrid — layout-edit mode (#236 จัดผัง)', () => {
  function renderLayout(overrides: Partial<Parameters<typeof SpatialRoomGrid>[0]> = {}) {
    const onSelect = jest.fn()
    const onMoveRequest = jest.fn()
    const onLayoutDrop = jest.fn()
    render(
      <SpatialRoomGrid
        rooms={ROOMS}
        onSelect={onSelect}
        onMoveRequest={onMoveRequest}
        canDrag
        mode="layout"
        onLayoutDrop={onLayoutDrop}
        {...overrides}
      />,
    )
    return { onSelect, onMoveRequest, onLayoutDrop }
  }

  function cell(col: number, row: number): HTMLElement {
    const el = document.querySelector(`[data-cell][data-col="${col}"][data-row="${row}"]`)
    if (!el) throw new Error(`cell (${col},${row}) not rendered`)
    return el as HTMLElement
  }

  it('renders the empty-cell lattice with ONE extrapolation band beyond each edge', () => {
    renderLayout()
    // Base board is 3 cols × 2 rows → lattice (3+1) × (2+1) = 12 cells.
    expect(document.querySelectorAll('[data-cell]')).toHaveLength(12)
    expect(cell(4, 3)).toBeTruthy()
  })

  it('suppresses tap-select while the mode is active', () => {
    const { onSelect } = renderLayout()
    fireEvent.click(tile(2))
    expect(onSelect).not.toHaveBeenCalled()
  })

  it('drops a VACANT tile onto another placed tile → swap callback (guest-move untouched)', () => {
    const { onLayoutDrop, onMoveRequest } = renderLayout()
    dragTile(tile(2), tile(4)) // vacant → vacant: undraggable in guest mode
    expect(onLayoutDrop).toHaveBeenCalledTimes(1)
    const [source, target] = onLayoutDrop.mock.calls[0]
    expect(source.id).toBe(2)
    expect(target).toMatchObject({ type: 'swap', room: expect.objectContaining({ id: 4 }) })
    expect(onMoveRequest).not.toHaveBeenCalled()
  })

  it('drops a tile onto an empty lattice cell → place callback with the cell coords', () => {
    const { onLayoutDrop } = renderLayout()
    dragTile(tile(1), cell(2, 2))
    expect(onLayoutDrop).toHaveBeenCalledTimes(1)
    const [source, target] = onLayoutDrop.mock.calls[0]
    expect(source.id).toBe(1)
    expect(target).toEqual({ type: 'cell', col: 2, row: 2 })
  })

  it('drags an UNPLACED-row tile onto a cell → place (this is how it gets on the board)', () => {
    const { onLayoutDrop } = renderLayout()
    dragTile(tile(5), cell(4, 3)) // X99 into the extrapolation corner
    expect(onLayoutDrop).toHaveBeenCalledWith(
      expect.objectContaining({ id: 5, roomNo: 'X99' }),
      { type: 'cell', col: 4, row: 3 },
    )
  })

  it('ignores a drop onto an UNPLACED tile (no pixel pair to swap with)', () => {
    const { onLayoutDrop } = renderLayout()
    dragTile(tile(1), tile(5))
    expect(onLayoutDrop).not.toHaveBeenCalled()
  })

  it('never starts a layout drag when canDrag is false (read-only branch)', () => {
    const { onLayoutDrop } = renderLayout({ canDrag: false })
    dragTile(tile(2), tile(4))
    expect(onLayoutDrop).not.toHaveBeenCalled()
  })
})
