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
 * (Layout-edit drag — rearranging the board itself — is a separate, explicit
 * mode per the #225 decision comment; not part of this gesture.)
 */

const DRAG_THRESHOLD_PX = 8

interface DragState {
  source: SpatialRoom
  startX: number
  startY: number
  dx: number
  dy: number
  hoverId: number | null
}

export default function SpatialRoomGrid({
  rooms,
  onSelect,
  onMoveRequest,
  canDrag,
}: {
  rooms: SpatialRoom[]
  onSelect: (room: SpatialRoom) => void
  /** Fired when an occupied tile is dropped on an eligible target. */
  onMoveRequest: (from: SpatialRoom, to: SpatialRoom) => void
  /** False on read-only branches — tiles stay tappable, never draggable. */
  canDrag: boolean
}) {
  const layout = useMemo(() => computeSpatialLayout(rooms), [rooms])
  const [drag, setDrag] = useState<DragState | null>(null)
  // Pre-drag tracking (pointer down, threshold not yet crossed).
  const pendingRef = useRef<{ room: SpatialRoom; x: number; y: number } | null>(null)
  // Suppresses the click that follows a completed drag.
  const justDraggedRef = useRef(false)

  const byId = useMemo(() => {
    const m = new Map<number, SpatialRoom>()
    for (const r of rooms) m.set(r.id, r)
    return m
  }, [rooms])

  const endDrag = () => {
    pendingRef.current = null
    setDrag(null)
    // The click that follows pointerup fires synchronously; clear the
    // suppression flag on the next tick so a later genuine tap still selects.
    window.setTimeout(() => {
      justDraggedRef.current = false
    }, 0)
  }

  const handlePointerDown = (e: React.PointerEvent<HTMLElement>, room: SpatialRoom) => {
    if (!canDrag || room.status !== 'occupied') return
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
      setDrag({
        source: pending.room,
        startX: pending.x,
        startY: pending.y,
        dx: e.clientX - pending.x,
        dy: e.clientY - pending.y,
        hoverId: null,
      })
      return
    }
    if (!drag) return
    // The dragged tile has pointer-events:none while dragging, so
    // elementFromPoint hit-tests the tile UNDER the pointer.
    const under = document.elementFromPoint?.(e.clientX, e.clientY)
    const targetEl = under?.closest?.('[data-room-id]') as HTMLElement | null
    const hoverId = targetEl ? Number(targetEl.dataset.roomId) : null
    setDrag({
      ...drag,
      dx: e.clientX - drag.startX,
      dy: e.clientY - drag.startY,
      hoverId: hoverId === drag.source.id ? null : hoverId,
    })
  }

  const handlePointerUp = () => {
    if (drag) {
      const target = drag.hoverId != null ? byId.get(drag.hoverId) : undefined
      if (target && moveEligibility(drag.source, target) !== 'blocked') {
        onMoveRequest(drag.source, target)
      }
    }
    endDrag()
  }

  const handleClick = (room: SpatialRoom) => {
    if (justDraggedRef.current) {
      justDraggedRef.current = false
      return
    }
    onSelect(room)
  }

  const renderTile = (room: SpatialRoom, pos?: { col: number; row: number }) => {
    const view = roomStatusView(room.status, {
      isClean: room.isClean,
      isMaintenance: room.isMaintenance,
    })
    const isSource = drag?.source.id === room.id
    const elig: MoveEligibility | null =
      drag && !isSource ? moveEligibility(drag.source, room) : null
    const isHover = drag?.hoverId === room.id

    const dragStyle: React.CSSProperties | undefined = isSource
      ? {
          transform: `translate(${drag!.dx}px, ${drag!.dy}px) scale(1.04)`,
          zIndex: 30,
          position: 'relative',
          pointerEvents: 'none',
          boxShadow: 'var(--v2-shadow-lg)',
          opacity: 0.92,
        }
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
          // when panned from vacant tiles on tablets.
          touchAction: canDrag && room.status === 'occupied' ? 'none' : undefined,
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

  if (layout.placed.length === 0 && layout.unplaced.length === 0) {
    return (
      <div className="v2-card px-5 py-12 text-center text-[14px]" style={{ color: 'var(--v2-ink-3)' }}>
        ไม่พบห้องที่ตรงกับเงื่อนไข
      </div>
    )
  }

  return (
    <div className="space-y-6">
      {layout.placed.length > 0 && (
        <div className="overflow-x-auto pb-1">
          <div
            data-testid="spatial-board"
            className="grid gap-2.5"
            style={{
              gridTemplateColumns: `repeat(${layout.cols}, minmax(96px, 1fr))`,
              minWidth: layout.cols * 104,
            }}
          >
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
