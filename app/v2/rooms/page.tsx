'use client'

import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { AlertCircle, Search } from 'lucide-react'
import { useBranch } from '@/contexts/BranchContext'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useLiveRefresh } from '@/lib/v2/use-live-refresh'
import { roomStatusView } from '@/lib/v2/status'
import { buildLayoutMoves, type SpatialRoom } from '@/lib/v2/spatial-grid'
import { V2Spinner, LiveDot, V2PageHeader, VilleNotice } from '@/components/v2/primitives'
import RoomActionSheet, { type RoomItem, type RoomAction } from '@/components/v2/RoomActionSheet'
import SpatialRoomGrid, { type LayoutDropTarget } from '@/components/v2/SpatialRoomGrid'
import GuestMoveConfirmModal from '@/components/v2/GuestMoveConfirmModal'
import CheckInModal from '@/components/CheckInModal'
import CheckOutModal from '@/components/CheckOutModal'
import ExtendStayModal from '@/components/ExtendStayModal'
import ChangeRoomModal from '@/components/ChangeRoomModal'
import PosSaleModal from '@/components/PosSaleModal'
import WalkupPosModal from '@/components/WalkupPosModal'

const ROOM_EVENTS = [
  'RoomMarkedClean',
  'RoomMarkedDirty',
  'CheckInCreated',
  'CheckOutCompleted',
  'CheckInCancelled',
  'BookingCreated',
  'BookingModified',
  'BookingCancelled',
  // #236: a จัดผัง drop on ANY terminal — the board is shared, so the other
  // tabs must pick up the rearrangement instead of sitting on a stale board.
  'RoomLayoutChanged',
]

type ModalKind = 'checkin' | 'checkout' | 'extend' | 'change' | 'pos' | 'walkup'
const MODAL_ACTIONS: RoomAction[] = ['checkin', 'checkout', 'extend', 'change', 'pos']

interface FilterOption {
  value: string
  label: string
  match: (r: RoomItem) => boolean
}

const FILTERS: FilterOption[] = [
  { value: 'all', label: 'ทั้งหมด', match: () => true },
  { value: 'available', label: 'ว่าง', match: (r) => r.status === 'available' },
  { value: 'occupied', label: 'เข้าพัก', match: (r) => r.status === 'occupied' },
  { value: 'booked', label: 'จองแล้ว', match: (r) => r.status === 'booked' },
  { value: 'checkout_pending', label: 'รอเช็คเอาท์', match: (r) => r.status === 'checkout_pending' },
  { value: 'dirty', label: 'รอทำความสะอาด', match: (r) => r.status === 'available' && r.isClean === false },
  { value: 'maintenance', label: 'ซ่อมบำรุง', match: (r) => r.status === 'maintenance' || r.isMaintenance },
]

/** Spatial (iHOTEL board positions) is the DEFAULT view — ADR 0003 U1. The
 *  floor-grouped list stays as a toggle, persisted per device. */
type RoomsViewMode = 'spatial' | 'floors'
const VIEW_STORAGE_KEY = 'v2.roomsView'

export default function V2Rooms() {
  const { branch, canWrite, layoutWritebackEnabled } = useBranch()
  const branchFetch = useBranchFetch()
  const [rooms, setRooms] = useState<SpatialRoom[]>([])
  const [loading, setLoading] = useState(true)
  const [filter, setFilter] = useState('all')
  const [query, setQuery] = useState('')
  const [selected, setSelected] = useState<RoomItem | null>(null)
  const [modal, setModal] = useState<ModalKind | null>(null)
  const [busy, setBusy] = useState(false)
  // Spatial is the default (ADR 0003 U1); read the persisted choice after
  // mount to avoid an SSR/client hydration mismatch.
  const [viewMode, setViewMode] = useState<RoomsViewMode>('spatial')
  // Guest-move drag (#225): drop on an eligible target opens the confirm.
  const [moveReq, setMoveReq] = useState<{ from: SpatialRoom; to: SpatialRoom } | null>(null)
  // Layout-edit mode (#236 จัดผัง): only reachable while the ship-dark
  // LAYOUT_WRITEBACK_ENABLED flag is on — the board is SHARED with iHOTEL and
  // a canonical-only rearrange would fork the two boards.
  const [layoutMode, setLayoutMode] = useState(false)
  const [layoutError, setLayoutError] = useState<string | null>(null)

  useEffect(() => {
    try {
      const stored = window.localStorage.getItem(VIEW_STORAGE_KEY)
      if (stored === 'floors' || stored === 'spatial') setViewMode(stored)
    } catch {
      /* private mode — keep default */
    }
  }, [])

  const changeViewMode = (mode: RoomsViewMode) => {
    setViewMode(mode)
    // จัดผัง only exists on the spatial board — leaving it exits the mode.
    if (mode !== 'spatial') setLayoutMode(false)
    try {
      window.localStorage.setItem(VIEW_STORAGE_KEY, mode)
    } catch {
      /* private mode — non-persistent */
    }
  }

  // Latest-wins guard: branch can flip mid-flight (hfhotel default → stored hfville).
  const reqRef = useRef(0)
  // How many จัดผัง drops this terminal currently has in flight (#236).
  const layoutInFlightRef = useRef(0)

  const fetchRooms = useCallback(async () => {
    const token = ++reqRef.current
    try {
      const res = await branchFetch('/api/rooms?limit=300')
      if (token !== reqRef.current) return // superseded by a newer branch fetch
      if (res.ok) {
        const data = await res.json()
        setRooms((data.data || data || []) as SpatialRoom[])
      }
    } catch {
      /* empty state */
    } finally {
      setLoading(false)
    }
  }, [branchFetch])

  // `loading` starts true; refetches (branch switch / SSE) update in place
  // without flashing the spinner.
  useEffect(() => {
    fetchRooms()
  }, [fetchRooms])

  // Live-refresh on iHOTEL/other-app changes mirrored through the event stream.
  //
  // `RoomLayoutChanged` also comes back to the terminal that caused it (the
  // stream has no origin filter). Refetching then would race this tab's own
  // optimistic coords during a rapid จัดผัง session, so a drop in flight
  // suppresses the refetch — the optimistic state already IS what the server
  // just persisted, and a failed drop resyncs explicitly in its catch.
  const liveRefresh = useCallback(() => {
    if (layoutInFlightRef.current > 0) return
    fetchRooms()
  }, [fetchRooms])
  const live = useLiveRefresh(branch, ROOM_EVENTS, liveRefresh)

  const filtered = useMemo(() => {
    const f = FILTERS.find((x) => x.value === filter) || FILTERS[0]
    const q = query.trim().toLowerCase()
    return rooms.filter((r) => f.match(r) && (!q || r.roomNo.toLowerCase().includes(q)))
  }, [rooms, filter, query])

  // Group by floor, numeric sort within.
  const groups = useMemo(() => {
    const byFloor = new Map<number, RoomItem[]>()
    for (const r of filtered) {
      const fl = r.floor ?? 0
      if (!byFloor.has(fl)) byFloor.set(fl, [])
      byFloor.get(fl)!.push(r)
    }
    return [...byFloor.entries()]
      .sort((a, b) => a[0] - b[0])
      .map(([floor, list]) => ({
        floor,
        list: list.sort((a, b) => a.roomNo.localeCompare(b.roomNo, undefined, { numeric: true })),
      }))
  }, [filtered])

  const counts = useMemo(() => {
    const c: Record<string, number> = {}
    for (const f of FILTERS) c[f.value] = rooms.filter(f.match).length
    return c
  }, [rooms])

  const refreshAfterModal = () => {
    fetchRooms()
    setModal(null)
    setSelected(null)
  }

  const handleAction = async (a: RoomAction) => {
    if (!selected) return
    if (MODAL_ACTIONS.includes(a)) {
      setModal(a as ModalKind)
      return // keep `selected`; sheet is replaced by the modal overlay
    }
    // Housekeeping actions (modal actions returned above). All route through the
    // /api/housekeeping/* endpoints, which flip the canonical flag AND mirror to
    // legacy HT_Rooms (Room_Manternace / Room_Clean) so iHOTEL stays in step —
    // the old PATCH /status path only set the canonical string and never reached
    // iHOTEL. `a` is 'maintenance' | 'ready' | 'clean' | 'dirty' here.
    setBusy(true)
    try {
      if (a === 'clean' || a === 'dirty') {
        await branchFetch(`/api/housekeeping/rooms/${selected.id}/${a}`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({}),
        })
      } else {
        const maintenance = a === 'maintenance'
        await branchFetch(`/api/housekeeping/rooms/${selected.id}/maintenance`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ maintenance }),
        })
      }
      await fetchRooms()
      setSelected(null)
    } catch {
      /* swallow — list refetch reflects truth */
    } finally {
      setBusy(false)
    }
  }

  /** จัดผัง drop (#236) — immediate per-drop write, no confirm (decision 4).
   *  Optimistic apply + revert-and-inline-error on failure. Pixels for a
   *  place are neighbor-derived from the UNFILTERED room list so they
   *  round-trip through computeSpatialLayout into the intended cell; a swap
   *  exchanges the two rooms' existing pairs verbatim (decision 2). */
  const handleLayoutDrop = async (source: SpatialRoom, target: LayoutDropTarget) => {
    // `rooms` (unfiltered) is deliberate — see buildLayoutMoves/deriveBoardPixels.
    const moves = buildLayoutMoves(rooms, source, target)
    if (!moves) return // swap partner has no coords, or self-drop — no-op

    // Optimistic apply; remember only the touched rooms' prior coords so a
    // revert can't clobber an interleaved drop on other tiles.
    const prev = new Map(
      moves.map((m) => {
        const r = rooms.find((rr) => rr.id === m.id)
        return [m.id, { roomX: r?.roomX ?? null, roomY: r?.roomY ?? null }] as const
      }),
    )
    setRooms((rs) =>
      rs.map((r) => {
        const m = moves.find((mm) => mm.id === r.id)
        return m ? { ...r, roomX: m.roomX, roomY: m.roomY } : r
      }),
    )
    setLayoutError(null)

    layoutInFlightRef.current += 1
    try {
      const res = await branchFetch('/api/rooms/layout', {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ moves }),
      })
      const data = await res.json().catch(() => ({}))
      if (!res.ok || !data.ok) {
        throw new Error(data.error || data.message || 'บันทึกผังห้องไม่สำเร็จ')
      }
      // Success: the optimistic coords ARE what the server persisted — no
      // refetch needed (and skipping it keeps rapid drops from racing).
    } catch (err) {
      setRooms((rs) =>
        rs.map((r) => {
          const p = prev.get(r.id)
          return p ? { ...r, roomX: p.roomX, roomY: p.roomY } : r
        }),
      )
      setLayoutError(err instanceof Error ? err.message : 'บันทึกผังห้องไม่สำเร็จ')
      fetchRooms() // resync with server truth
    } finally {
      layoutInFlightRef.current -= 1
    }
  }

  if (loading) return <V2Spinner label="กำลังโหลดผังห้องพัก…" />

  return (
    <div className="space-y-5">
      <V2PageHeader
        eyebrow="ผังห้องพัก"
        title="ห้องพัก"
        right={
          <div className="flex items-center gap-3">
            {canWrite && (
              <button
                type="button"
                onClick={() => setModal('walkup')}
                className="rounded-md bg-amber-600 px-3 py-1.5 text-[13px] font-medium text-white hover:bg-amber-700"
              >
                ขายสินค้า / Walk-up sale
              </button>
            )}
            <LiveDot connected={live} />
          </div>
        }
      />

      <VilleNotice branch={branch} />

      {/* Search + view toggle */}
      <div className="flex flex-wrap items-center gap-3">
        <div
          className="flex items-center gap-2 px-3 h-11 rounded-[12px] max-w-md flex-1 min-w-[220px]"
          style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line)' }}
        >
          <Search size={17} style={{ color: 'var(--v2-ink-3)' }} />
          <input
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="ค้นหาหมายเลขห้อง…"
            inputMode="numeric"
            className="flex-1 bg-transparent outline-none text-[14px]"
          />
        </div>
        {/* Spatial board (iHOTEL-arranged positions) is the default; the
            floor-grouped list is the secondary toggle — ADR 0003 U1. */}
        <div className="flex gap-2" role="group" aria-label="รูปแบบการแสดงผล">
          <button
            type="button"
            className="v2-chip"
            data-active={viewMode === 'spatial'}
            onClick={() => changeViewMode('spatial')}
          >
            ผังห้อง
          </button>
          <button
            type="button"
            className="v2-chip"
            data-active={viewMode === 'floors'}
            onClick={() => changeViewMode('floors')}
          >
            แยกตามชั้น
          </button>
          {/* จัดผัง (#236) — visible ONLY while LAYOUT_WRITEBACK_ENABLED is on
              (the board is shared with iHOTEL; a canonical-only rearrange is
              never allowed). Ungated by role, like iHOTEL FormRoomMain. */}
          {layoutWritebackEnabled && viewMode === 'spatial' && (
            <button
              type="button"
              className="v2-chip"
              data-active={layoutMode}
              onClick={() => {
                setLayoutMode((v) => !v)
                setLayoutError(null)
              }}
            >
              จัดผัง
            </button>
          )}
        </div>
      </div>

      {/* จัดผัง status / error strip */}
      {layoutMode && (
        <div className="space-y-2">
          <div className="text-[12.5px]" style={{ color: 'var(--v2-ink-3)' }}>
            โหมดจัดผัง: ลากห้องไปยังช่องว่างเพื่อย้าย / วางทับห้องอื่นเพื่อสลับตำแหน่ง —
            บันทึกทันทีและมีผลกับ iHOTEL ด้วย
          </div>
          {layoutError && (
            <div className="flex items-start gap-2 rounded-lg border border-red-200 bg-red-50 px-3 py-2 text-[13px] text-red-700">
              <AlertCircle size={16} className="mt-0.5 shrink-0" />
              <span>{layoutError}</span>
            </div>
          )}
        </div>
      )}

      {/* Filter chips */}
      <div className="flex gap-2 overflow-x-auto pb-1 -mx-1 px-1">
        {FILTERS.map((f) => (
          <button key={f.value} className="v2-chip" data-active={filter === f.value} onClick={() => setFilter(f.value)}>
            {f.label}
            <span className="v2-num text-[11px] font-bold opacity-70">{counts[f.value] ?? 0}</span>
          </button>
        ))}
      </div>

      {/* Board — spatial (default) or floor-grouped */}
      {viewMode === 'spatial' ? (
        <SpatialRoomGrid
          // จัดผัง always works on the UNFILTERED set: bands computed from a
          // filtered subset can differ, and pixels derived from them may land
          // in the wrong cell once the filter clears (round-trip hazard).
          rooms={layoutMode ? rooms : filtered}
          onSelect={(room) => setSelected(room)}
          onMoveRequest={(from, to) => setMoveReq({ from, to })}
          canDrag={canWrite}
          mode={layoutMode ? 'layout' : 'guest'}
          onLayoutDrop={handleLayoutDrop}
        />
      ) : groups.length === 0 ? (
        <div className="v2-card px-5 py-12 text-center text-[14px]" style={{ color: 'var(--v2-ink-3)' }}>
          ไม่พบห้องที่ตรงกับเงื่อนไข
        </div>
      ) : (
        <div className="space-y-6">
          {groups.map(({ floor, list }) => (
            <div key={floor}>
              <div className="v2-eyebrow mb-2.5">{floor === 0 ? 'อื่น ๆ' : `ชั้น ${floor}`}</div>
              <div className="grid gap-2.5 grid-cols-3 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8">
                {list.map((room) => {
                  const view = roomStatusView(room.status, { isClean: room.isClean, isMaintenance: room.isMaintenance })
                  return (
                    <button
                      key={room.id}
                      onClick={() => setSelected(room)}
                      className="text-left rounded-[12px] p-2.5 transition-transform active:scale-[.97] hover:shadow-[var(--v2-shadow-md)]"
                      style={{ background: `var(--v2-${view.tone}-bg)`, border: '1px solid var(--v2-line)' }}
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
                })}
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Legend */}
      <div className="flex flex-wrap gap-x-4 gap-y-1.5 pt-2">
        {FILTERS.slice(1).map((f) => {
          const view = roomStatusView(
            f.value === 'dirty' ? 'available' : f.value,
            { isClean: f.value !== 'dirty', isMaintenance: f.value === 'maintenance' },
          )
          return (
            <span key={f.value} className="inline-flex items-center gap-1.5 text-[12px]" style={{ color: 'var(--v2-ink-3)' }}>
              <span className={`v2-dot ${view.dot}`} /> {f.label}
            </span>
          )
        })}
      </div>

      {/* Action sheet — hidden while a transactional modal is open */}
      {selected && !modal && (
        <RoomActionSheet room={selected} onClose={() => setSelected(null)} onAction={handleAction} busy={busy} readOnly={!canWrite} />
      )}

      {/* Transactional modals (reused from the classic app for contract safety) */}
      {modal === 'checkin' && selected && (
        <CheckInModal
          room={{ id: selected.id, roomNo: selected.roomNo, roomTypeName: selected.roomTypeName }}
          onClose={() => setModal(null)}
          onSuccess={refreshAfterModal}
        />
      )}
      {modal === 'checkout' && selected && (
        <CheckOutModal room={{ id: selected.id, roomNo: selected.roomNo }} onClose={() => setModal(null)} onSuccess={refreshAfterModal} />
      )}
      {modal === 'extend' && selected && (
        <ExtendStayModal room={{ id: selected.id, roomNo: selected.roomNo }} onClose={() => setModal(null)} onSuccess={refreshAfterModal} />
      )}
      {modal === 'change' && selected && (
        <ChangeRoomModal room={{ id: selected.id, roomNo: selected.roomNo }} onClose={() => setModal(null)} onSuccess={refreshAfterModal} />
      )}
      {modal === 'pos' && selected && (
        <PosSaleModal room={{ id: selected.id, roomNo: selected.roomNo }} onClose={() => setModal(null)} onSuccess={() => fetchRooms()} />
      )}
      {/* Walk-up sale — not room-bound, so it opens without a `selected` room. */}
      {modal === 'walkup' && (
        <WalkupPosModal onClose={() => setModal(null)} onSuccess={() => fetchRooms()} />
      )}

      {/* Guest-move drag confirm (#225) — additive alternate to the
          RoomActionSheet → ChangeRoomModal path; same backend contract.
          onSuccess refetches the grid; the modal stays open for slip print. */}
      {moveReq && (
        <GuestMoveConfirmModal
          fromRoom={moveReq.from}
          toRoom={moveReq.to}
          onClose={() => setMoveReq(null)}
          onSuccess={() => fetchRooms()}
        />
      )}
    </div>
  )
}
