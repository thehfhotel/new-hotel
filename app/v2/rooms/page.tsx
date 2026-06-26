'use client'

import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { Search } from 'lucide-react'
import { useBranch } from '@/contexts/BranchContext'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { roomStatusView } from '@/lib/v2/status'
import { V2Spinner, LiveDot, V2PageHeader, VilleNotice } from '@/components/v2/primitives'
import RoomActionSheet, { type RoomItem, type RoomAction } from '@/components/v2/RoomActionSheet'
import CheckInModal from '@/components/CheckInModal'
import CheckOutModal from '@/components/CheckOutModal'
import ExtendStayModal from '@/components/ExtendStayModal'
import ChangeRoomModal from '@/components/ChangeRoomModal'
import PosSaleModal from '@/components/PosSaleModal'

const ROOM_EVENTS = [
  'RoomMarkedClean',
  'RoomMarkedDirty',
  'CheckInCreated',
  'CheckOutCompleted',
  'CheckInCancelled',
  'BookingCreated',
  'BookingCancelled',
]

type ModalKind = 'checkin' | 'checkout' | 'extend' | 'change' | 'pos'
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
  { value: 'maintenance', label: 'ซ่อมบำรุง', match: (r) => r.status === 'maintenance' || r.isMaintenance },
]

export default function V2Rooms() {
  const { branch, canWrite } = useBranch()
  const branchFetch = useBranchFetch()
  const [rooms, setRooms] = useState<RoomItem[]>([])
  const [loading, setLoading] = useState(true)
  const [live, setLive] = useState(false)
  const [filter, setFilter] = useState('all')
  const [query, setQuery] = useState('')
  const [selected, setSelected] = useState<RoomItem | null>(null)
  const [modal, setModal] = useState<ModalKind | null>(null)
  const [busy, setBusy] = useState(false)

  const fetchRef = useRef<() => void>(() => {})
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null)
  // Latest-wins guard: branch can flip mid-flight (hfhotel default → stored hfville).
  const reqRef = useRef(0)

  const fetchRooms = useCallback(async () => {
    const token = ++reqRef.current
    try {
      const res = await branchFetch('/api/rooms?limit=300')
      if (token !== reqRef.current) return // superseded by a newer branch fetch
      if (res.ok) {
        const data = await res.json()
        setRooms((data.data || data || []) as RoomItem[])
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

  useEffect(() => {
    fetchRef.current = fetchRooms
  }, [fetchRooms])

  useEffect(() => {
    const schedule = () => {
      if (timer.current) clearTimeout(timer.current)
      timer.current = setTimeout(() => fetchRef.current(), 500)
    }
    const es = new EventSource(`/api/events?branch=${encodeURIComponent(branch)}`)
    es.onopen = () => setLive(true)
    es.onerror = () => setLive(false)
    ROOM_EVENTS.forEach((e) => es.addEventListener(e, schedule))
    return () => {
      if (timer.current) clearTimeout(timer.current)
      es.close()
      setLive(false)
    }
  }, [branch])

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
    // Housekeeping / maintenance — mirror the classic housekeeping handler exactly.
    setBusy(true)
    try {
      // Maintenance toggle only — clean/dirty is managed on /housekeeping, not
      // in the room grid (matches iHOTEL; the old 'clean'/'dirty' actions wrote
      // the stale room_status and were no-ops/broken).
      let status = 'available'
      if (a === 'maintenance') status = 'maintenance'
      else if (a === 'ready') status = 'available'
      await branchFetch(`/api/rooms/${selected.id}/status`, {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ status }),
      })
      await fetchRooms()
      setSelected(null)
    } catch {
      /* swallow — list refetch reflects truth */
    } finally {
      setBusy(false)
    }
  }

  if (loading) return <V2Spinner label="กำลังโหลดผังห้องพัก…" />

  return (
    <div className="space-y-5">
      <V2PageHeader eyebrow="ผังห้องพัก" title="ห้องพัก" right={<LiveDot connected={live} />} />

      <VilleNotice branch={branch} />

      {/* Search */}
      <div
        className="flex items-center gap-2 px-3 h-11 rounded-[12px] max-w-md"
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

      {/* Filter chips */}
      <div className="flex gap-2 overflow-x-auto pb-1 -mx-1 px-1">
        {FILTERS.map((f) => (
          <button key={f.value} className="v2-chip" data-active={filter === f.value} onClick={() => setFilter(f.value)}>
            {f.label}
            <span className="v2-num text-[11px] font-bold opacity-70">{counts[f.value] ?? 0}</span>
          </button>
        ))}
      </div>

      {/* Floors */}
      {groups.length === 0 ? (
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
    </div>
  )
}
