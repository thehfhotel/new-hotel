'use client'

import { useMemo, useRef, useState } from 'react'
import { roomStatusView } from '@/lib/v2/status'
import {
  computeSpatialLayout,
  moveEligibility,
  type MoveEligibility,
  type SpatialRoom,
} from '@/lib/v2/spatial-grid'

/**
 * Spatial room grid (ADR 0003 U1 / #226) — renders tiles at the
 * receptionist-arranged `roomX`/`roomY` board positions mirrored from iHOTEL
 * FormRoomMain. Rooms without a position fall into the "unplaced" row below.
 *
 * Guest-move drag (#225) is the DEFAULT drag gesture: dragging an occupied
 * tile onto an eligible target (vacant ok; vacant-dirty warned; maintenance
 * blocked) fires `onMoveRequest` — the parent opens the confirm dialog which
 * reuses the existing change-room endpoint. Implemented with pointer events so
 * tablets work. A tap (no movement) selects the room like the floor view.
 *
 * Layout-edit drag (#236, จัดผัง) is the explicit `mode='layout'` variant:
 * EVERY tile is draggable (any status, incl. the unplaced row), tap-select is
 * suppressed, and the drop resolution changes — drop on another PLACED tile
 * fires a swap, drop on a synthetic empty-cell target (the cols+1 × rows+1
 * lattice rendered only in this mode) fires a place. Eligibility styling is
 * bypassed. All the pointer/ghost machinery below is mode-agnostic and shared
 * verbatim (perf contract).
 */

const DRAG_THRESHOLD_PX = 8

/** Where a layout-edit drop landed (#236). `cell` = empty lattice cell
 *  (1-based board coords, may be one band beyond the current extent);
 *  `swap` = another placed tile. */
export type LayoutDropTarget =
  | { type: 'cell'; col: number; row: number }
  | { type: 'swap'; room: SpatialRoom }

/**
 * Perf contract (2026-07-10, "drag isn't snappy" fix): React state changes at
 * most a handful of times per drag — once at threshold-cross and once per
 * hover-target CHANGE. The ghost's translate() is applied imperatively to the
 * source DOM node inside requestAnimationFrame, so pointermove never triggers
 * a 58-tile re-render, and `transition: none` on the source stops the
 * `transition-transform` class from easing the ghost behind the pointer.
 */
interface DragState {
  source: SpatialRoom
  startX: number
  startY: number
  hoverId: number | null
  /** Layout mode only — the empty lattice cell under the pointer. */
  hoverCell: { col: number; row: number } | null
}

export default function SpatialRoomGrid({
  rooms,
  onSelect,
  onMoveRequest,
  canDrag,
  mode = 'guest',
  onLayoutDrop,
}: {
  rooms: SpatialRoom[]
  onSelect: (room: SpatialRoom) => void
  /** Fired when an occupied tile is dropped on an eligible target. */
  onMoveRequest: (from: SpatialRoom, to: SpatialRoom) => void
  /** False on read-only branches — tiles stay tappable, never draggable. */
  canDrag: boolean
  /** 'layout' = จัดผัง layout-edit mode (#236); default 'guest' keeps the
   *  guest-move behavior byte-identical. */
  mode?: 'guest' | 'layout'
  /** Layout mode only — fired on every completed drop (immediate write, no
   *  confirm dialog per decision 4). */
  onLayoutDrop?: (source: SpatialRoom, target: LayoutDropTarget) => void
}) {
  const layout = useMemo(() => computeSpatialLayout(rooms), [rooms])
  const [drag, setDrag] = useState<DragState | null>(null)
  // Pre-drag tracking (pointer down, threshold not yet crossed).
  const pendingRef = useRef<{ room: SpatialRoom; x: number; y: number } | null>(null)
  // Suppresses the click that follows a completed drag.
  const justDraggedRef = useRef(false)
  // Imperative ghost position — lives outside React state (see perf contract).
  const sourceElRef = useRef<HTMLElement | null>(null)
  const posRef = useRef({ dx: 0, dy: 0 })
  const rafRef = useRef<number | null>(null)

  const isLayoutMode = mode === 'layout'

  const applyGhostTransform = () => {
    rafRef.current = null
    const el = sourceElRef.current
    if (!el) return
    el.style.transform = `translate(${posRef.current.dx}px, ${posRef.current.dy}px) scale(1.04)`
  }

  const byId = useMemo(() => {
    const m = new Map<number, SpatialRoom>()
    for (const r of rooms) m.set(r.id, r)
    return m
  }, [rooms])

  // Layout mode: which cell each placed room occupies (for the source-drop
  // no-op check) and which cells are occupied at all.
  const cellByRoomId = useMemo(() => {
    const m = new Map<number, { col: number; row: number }>()
    for (const p of layout.placed) m.set(p.room.id, { col: p.col, row: p.row })
    return m
  }, [layout])

  const endDrag = () => {
    pendingRef.current = null
    if (rafRef.current != null) {
      window.cancelAnimationFrame?.(rafRef.current)
      rafRef.current = null
    }
    if (sourceElRef.current) sourceElRef.current.style.transform = ''
    sourceElRef.current = null
    posRef.current = { dx: 0, dy: 0 }
    setDrag(null)
    // The click that follows pointerup fires synchronously; clear the
    // suppression flag on the next tick so a later genuine tap still selects.
    window.setTimeout(() => {
      justDraggedRef.current = false
    }, 0)
  }

  const handlePointerDown = (e: React.PointerEvent<HTMLElement>, room: SpatialRoom) => {
    if (!canDrag) return
    // Guest-move only drags occupied tiles; layout mode drags EVERY tile
    // (any status, incl. the unplaced row — decision 3).
    if (!isLayoutMode && room.status !== 'occupied') return
    if (e.button !== 0 && e.pointerType === 'mouse') return
    pendingRef.current = { room, x: e.clientX, y: e.clientY }
  }

  const handlePointerMove = (e: React.PointerEvent<HTMLElement>) => {
    const pending = pendingRef.current
    if (!drag && pending) {
      const dist = Math.hypot(e.clientX - pending.x, e.clientY - pending.y)
      if (dist < DRAG_THRESHOLD_PX) return
      try {
        e.currentTarget.setPointerCapture?.(e.pointerId)
      } catch {
        /* jsdom / older browsers */
      }
      justDraggedRef.current = true
      sourceElRef.current = e.currentTarget
      posRef.current = { dx: e.clientX - pending.x, dy: e.clientY - pending.y }
      applyGhostTransform()
      setDrag({
        source: pending.room,
        startX: pending.x,
        startY: pending.y,
        hoverId: null,
        hoverCell: null,
      })
      return
    }
    if (!drag) return
    // Ghost position: ref + rAF only — no setState, no re-render (perf contract).
    posRef.current = { dx: e.clientX - drag.startX, dy: e.clientY - drag.startY }
    if (rafRef.current == null) {
      rafRef.current = window.requestAnimationFrame?.(applyGhostTransform) ?? null
      // jsdom without rAF: apply synchronously so tests observe the transform.
      if (rafRef.current == null) applyGhostTransform()
    }
    // Hover hit-test stays synchronous (drop correctness depends on it), but
    // only a CHANGE of target commits state. The dragged tile has
    // pointer-events:none while dragging, so elementFromPoint hit-tests the
    // tile UNDER the pointer. Layout mode also resolves the synthetic empty
    // lattice cells ([data-cell]) rendered behind the tiles.
    const under = document.elementFromPoint?.(e.clientX, e.clientY)
    const targetEl = under?.closest?.(
      isLayoutMode ? '[data-room-id], [data-cell]' : '[data-room-id]',
    ) as HTMLElement | null
    const rawHoverId = targetEl?.dataset.roomId != null ? Number(targetEl.dataset.roomId) : null
    const hoverId = rawHoverId === drag.source.id ? null : rawHoverId
    const hoverCell =
      isLayoutMode && targetEl?.dataset.cell != null
        ? { col: Number(targetEl.dataset.col), row: Number(targetEl.dataset.row) }
        : null
    if (
      hoverId !== drag.hoverId ||
      hoverCell?.col !== drag.hoverCell?.col ||
      hoverCell?.row !== drag.hoverCell?.row
    ) {
      setDrag({ ...drag, hoverId, hoverCell })
    }
  }

  const handlePointerUp = () => {
    if (drag) {
      if (isLayoutMode) {
        const source = drag.source
        if (drag.hoverId != null) {
          // Drop on another tile ⇒ SWAP — only between two PLACED tiles
          // (their existing pixel pairs are exchanged verbatim; an unplaced
          // room has no pair to give, so those drops are ignored).
          const target = byId.get(drag.hoverId)
          if (
            target &&
            onLayoutDrop &&
            cellByRoomId.has(source.id) &&
            cellByRoomId.has(target.id)
          ) {
            onLayoutDrop(source, { type: 'swap', room: target })
          }
        } else if (drag.hoverCell) {
          // Drop on an empty lattice cell ⇒ PLACE (also how an unplaced-row
          // tile gets onto the board). Dropping a tile back onto its own
          // cell is a no-op.
          const own = cellByRoomId.get(source.id)
          if (
            onLayoutDrop &&
            !(own && own.col === drag.hoverCell.col && own.row === drag.hoverCell.row)
          ) {
            onLayoutDrop(source, {
              type: 'cell',
              col: drag.hoverCell.col,
              row: drag.hoverCell.row,
            })
          }
        }
      } else {
        const target = drag.hoverId != null ? byId.get(drag.hoverId) : undefined
        if (target && moveEligibility(drag.source, target) !== 'blocked') {
          onMoveRequest(drag.source, target)
        }
      }
    }
    endDrag()
  }

  const handleClick = (room: SpatialRoom) => {
    if (justDraggedRef.current) {
      justDraggedRef.current = false
      return
    }
    // จัดผัง: tap-select is disabled while the mode is active (decision 3).
    if (isLayoutMode) return
    onSelect(room)
  }

  const renderTile = (room: SpatialRoom, pos?: { col: number; row: number }) => {
    const view = roomStatusView(room.status, {
      isClean: room.isClean,
      isMaintenance: room.isMaintenance,
    })
    const isSource = drag?.source.id === room.id
    // Guest-move eligibility affordance is bypassed in layout mode — every
    // placed tile is a valid swap target there.
    const elig: MoveEligibility | null =
      !isLayoutMode && drag && !isSource ? moveEligibility(drag.source, room) : null
    const isHover = drag?.hoverId === room.id

    const dragStyle: React.CSSProperties | undefined = isSource
      ? {
          // Re-renders during a drag (hover changes) must not reset the
          // imperatively-applied position — re-emit the current ref value.
          transform: `translate(${posRef.current.dx}px, ${posRef.current.dy}px) scale(1.04)`,
          transition: 'none', // defeat .transition-transform easing — 1:1 tracking
          willChange: 'transform',
          zIndex: 30,
          position: 'relative',
          pointerEvents: 'none',
          boxShadow: 'var(--v2-shadow-lg)',
          opacity: 0.92,
        }
      : isLayoutMode && drag && isHover
        ? { outline: '2px solid var(--v2-vac)', outlineOffset: 1 }
        : elig === 'ok'
          ? { outline: `2px ${isHover ? 'solid' : 'dashed'} var(--v2-vac)`, outlineOffset: 1 }
          : elig === 'warn'
            ? { outline: `2px ${isHover ? 'solid' : 'dashed'} var(--v2-dirt)`, outlineOffset: 1 }
            : elig === 'blocked'
              ? { opacity: 0.45 }
              : undefined

    return (
      <button
        key={room.id}
        data-room-id={room.id}
        data-col={pos?.col}
        data-row={pos?.row}
        data-elig={elig ?? undefined}
        onClick={() => handleClick(room)}
        onPointerDown={(e) => handlePointerDown(e, room)}
        onPointerMove={handlePointerMove}
        onPointerUp={handlePointerUp}
        onPointerCancel={endDrag}
        className="text-left rounded-[12px] p-2.5 transition-transform active:scale-[.97] hover:shadow-[var(--v2-shadow-md)]"
        style={{
          background: `var(--v2-${view.tone}-bg)`,
          border: '1px solid var(--v2-line)',
          // touch-action:none only on drag sources so the board still scrolls
          // when panned from vacant tiles on tablets. Layout mode makes EVERY
          // tile a drag source, so it widens to all of them.
          touchAction:
            canDrag && (isLayoutMode || room.status === 'occupied') ? 'none' : undefined,
          ...(pos ? { gridColumn: pos.col, gridRow: pos.row } : undefined),
          ...dragStyle,
        }}
        title={`${room.roomNo} · ${view.label}`}
      >
        <div className="flex items-start justify-between">
          <span className="v2-num text-[19px] font-bold leading-none">{room.roomNo}</span>
          <span className={`v2-dot ${view.dot}`} style={{ marginTop: 3 }} />
        </div>
        <div className="text-[10.5px] mt-2 truncate" style={{ color: 'var(--v2-ink-3)' }}>
          {room.roomTypeName || '—'}
        </div>
        <div className="text-[11px] font-semibold mt-0.5" style={{ color: `var(--v2-${view.tone})` }}>
          {view.label}
        </div>
      </button>
    )
  }

  /** จัดผัง: synthetic drop targets for every lattice cell, including ONE
   *  extrapolation band beyond each edge so tiles can extend the board.
   *  Rendered BEFORE the tiles so tiles stack above them in shared cells —
   *  elementFromPoint then only resolves a cell when it is actually empty. */
  const renderLatticeCells = (cols: number, rows: number) => {
    const cells: React.ReactNode[] = []
    for (let row = 1; row <= rows; row += 1) {
      for (let col = 1; col <= cols; col += 1) {
        const isHover = drag?.hoverCell?.col === col && drag?.hoverCell?.row === row
        cells.push(
          <div
            key={`cell-${col}-${row}`}
            data-cell=""
            data-col={col}
            data-row={row}
            className="rounded-[12px]"
            style={{
              gridColumn: col,
              gridRow: row,
              minHeight: 76,
              border: isHover
                ? '2px solid var(--v2-vac)'
                : '1px dashed var(--v2-line)',
              background: isHover ? 'var(--v2-vac-bg)' : undefined,
            }}
          />,
        )
      }
    }
    return cells
  }

  if (layout.placed.length === 0 && layout.unplaced.length === 0) {
    return (
      <div className="v2-card px-5 py-12 text-center text-[14px]" style={{ color: 'var(--v2-ink-3)' }}>
        ไม่พบห้องที่ตรงกับเงื่อนไข
      </div>
    )
  }

  // Layout mode widens the board by one extrapolation band on each axis (and
  // seeds a 1×1 lattice on an empty board so the first placement is possible).
  const boardCols = isLayoutMode ? layout.cols + 1 : layout.cols
  const boardRows = isLayoutMode ? layout.rows + 1 : layout.rows

  return (
    <div className="space-y-6">
      {(layout.placed.length > 0 || (isLayoutMode && layout.unplaced.length > 0)) && (
        <div className="overflow-x-auto pb-1">
          <div
            data-testid="spatial-board"
            className="grid gap-2.5"
            style={{
              gridTemplateColumns: `repeat(${boardCols}, minmax(96px, 1fr))`,
              minWidth: boardCols * 104,
            }}
          >
            {isLayoutMode && renderLatticeCells(boardCols, boardRows)}
            {layout.placed.map(({ room, col, row }) => renderTile(room, { col, row }))}
          </div>
        </div>
      )}

      {layout.unplaced.length > 0 && (
        <div>
          <div className="v2-eyebrow mb-2.5">ยังไม่กำหนดตำแหน่ง</div>
          <div
            data-testid="spatial-unplaced"
            className="grid gap-2.5 grid-cols-3 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8"
          >
            {layout.unplaced.map((room) => renderTile(room))}
          </div>
        </div>
      )}
    </div>
  )
}
