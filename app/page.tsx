'use client'

import { useState, useEffect, useCallback, useRef } from 'react'
import {
  Loader2,
  AlertCircle,
  ArrowRightLeft,
  Clock,
  Coffee,
  LogIn,
  LogOut,
  CalendarPlus,
} from 'lucide-react'
import { useBranch, BRANCH_LABELS } from '@/contexts/BranchContext'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useSkin } from '@/contexts/SkinContext'
import CheckInModal from '@/components/CheckInModal'
import CheckOutModal from '@/components/CheckOutModal'
import ChangeRoomModal from '@/components/ChangeRoomModal'
import ExtendStayModal from '@/components/ExtendStayModal'
import PosSaleModal from '@/components/PosSaleModal'

// Domain-event variants that mutate the dashboard's tile counts or room grid.
// Names match `DomainEvent::type_name()` in
// `hotel-backend/src/outbox/event.rs` — keep in sync. Payment / customer
// events deliberately omitted: they don't shift the room-status grid or
// occupancy counters.
const DASHBOARD_REFRESH_EVENTS = [
  'RoomMarkedClean',
  'RoomMarkedDirty',
  'CheckInCreated',
  'CheckOutCompleted',
  'CheckInCancelled',
  'BookingCreated',
  'BookingModified',
  'BookingCancelled',
] as const

// Debounce window for batched refetches. A full booking aggregate can emit
// 5+ events back-to-back (one per night); 500ms lets the burst collapse to
// one fetch without making the dashboard feel stale.
const REFETCH_DEBOUNCE_MS = 500

interface Stats {
  totalRooms: number
  occupiedRooms: number
  availableRooms: number
  bookedRooms: number
  checkoutRooms: number
  totalCustomers: number
  activeBookings: number
  todayCheckIns: number
  todayCheckOuts: number
}

interface ApiRoom {
  Room_no: string
  Room_Type: string
  Room_Details: string
  Room_Clean: string
  Room_Use: string
  Room_Book: string
  Room_Manternace: string
}

type RoomStatus = 'available' | 'occupied' | 'booked' | 'maintenance' | 'cleaning' | 'checkout'

interface Room {
  roomNumber: string
  type: string
  details: string
  status: RoomStatus
}

interface CheckIn {
  guestName: string
  roomNumber: string
  checkInDate: string
  checkOutDate: string
}

// Each room status maps to a tiny coloured status dot. Tile bodies stay neutral
// (white panel + 1px border) — colour communicates state, not the whole tile.
const statusConfig: Record<RoomStatus, { dot: string; label: string }> = {
  available:   { dot: 'bg-success',     label: 'ว่าง' },
  occupied:    { dot: 'bg-brand-500',   label: 'มีผู้เข้าพัก' },
  booked:      { dot: 'bg-warning',     label: 'จองแล้ว' },
  maintenance: { dot: 'bg-textMuted',   label: 'ซ่อมบำรุง' },
  cleaning:    { dot: 'bg-warning/70',  label: 'ทำความสะอาด' },
  checkout:    { dot: 'bg-info',        label: 'รอเช็คเอาท์' },
}

// HF Hotel room layout matching actual hotel floor plan
const hfHotelRoomLayout: (string | null)[][] = [
  ['509', '510', '511', '512', '513', '514', '515', '516', '517', '518'],
  ['508', '507', null, '506', '505', '504', '503', '502', '501', null, null, 'A4-1', 'A4-2', 'A4-3'],
  ['409', '410', '411', '412', '413', '414', '415', '416', '417', '418', null, 'A3-1', 'A3-2', 'A3-3'],
  ['408', '407', null, '406', '405', '404', '403', '402', '401', null, null, 'V.201', 'A2-1', 'A2-3'],
  ['307', '308', '309', '310', '311', '312', '313'],
  ['306', '305', null, '304', '303', '302', '301'],
]

// HF Ville room layout - 32 rooms, 2 floors (สุราษฎร์ธานี)
const hfVilleRoomLayout: (string | null)[][] = [
  ['101', '103', '105', '107', '109', '111', '113', '115'],
  ['102', '104', '106', '108', '110', '112', '114', '116'],
  ['201', '203', '205', '207', '209', '211', '213', '215', '217', '218'],
  ['202', '204', '206', '208', '210', '212', '214', '216', null, null],
]

function getRoomStatus(room: ApiRoom, isCheckoutToday: boolean): RoomStatus {
  const hour = new Date().getHours()
  if (isCheckoutToday && hour >= 6) return 'checkout'
  if (room.Room_Manternace === 'yes') return 'maintenance'
  if (room.Room_Use === 'yes') return 'occupied'
  if (room.Room_Book && room.Room_Book !== '') return 'booked'
  if (room.Room_Clean === 'yes') return 'cleaning'
  return 'available'
}

export default function NewDashboard() {
  const { branch } = useBranch()
  const branchFetch = useBranchFetch()
  const { skin } = useSkin()
  const isModernSkin = skin === 'modern'
  const [stats, setStats] = useState<Stats>({
    totalRooms: 0, occupiedRooms: 0, availableRooms: 0, bookedRooms: 0,
    checkoutRooms: 0, totalCustomers: 0, activeBookings: 0, todayCheckIns: 0, todayCheckOuts: 0,
  })
  const [rooms, setRooms] = useState<Room[]>([])
  const [checkIns, setCheckIns] = useState<CheckIn[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [selectedRoom, setSelectedRoom] = useState<Room | null>(null)
  // PG room_id by uppercase Room_no — populated once on mount from
  // /api/new/rooms (the new app's room table). The dashboard's main data
  // source (/api/rooms, legacy) only exposes the string Room_no, but
  // POST /api/new/checkins requires the integer room_id. Built once
  // because room inventory rarely changes.
  const [roomIdByNo, setRoomIdByNo] = useState<Map<string, number>>(new Map())
  const [showCheckIn, setShowCheckIn] = useState(false)
  const [showCheckOut, setShowCheckOut] = useState(false)
  // Track G1 / T4 HIGH-2: extend-stay modal (one-more-night flow).
  const [showExtendStay, setShowExtendStay] = useState(false)
  // Track G4 / T4 HIGH-3: change-room modal (move guest A → B).
  const [showChangeRoom, setShowChangeRoom] = useState(false)
  // Track G6: POS / add-product modal (ring up a sale to folio).
  const [showPosSale, setShowPosSale] = useState(false)
  // SSE connection status — drives the tiny header dot. Starts pessimistic
  // so the dot reads "reconnecting" until the EventSource fires `onopen`.
  const [isLiveConnected, setIsLiveConnected] = useState(false)
  // Refs hold the latest fetchData and a debounce timer so the SSE effect
  // can stay stable across renders without re-subscribing on every fetch.
  const fetchDataRef = useRef<() => void>(() => {})
  const refetchTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null)

  // One-shot lookup of PG room ids — needed to drive the check-in/out
  // modals (which require integer room_id, while /api/rooms only exposes
  // the string Room_no). Re-runs only on branch change.
  useEffect(() => {
    let cancelled = false
    const loadIds = async () => {
      try {
        const res = await branchFetch('/api/new/rooms?limit=200')
        if (!res.ok) return
        const data = await res.json()
        if (cancelled) return
        const map = new Map<string, number>()
        for (const r of (data.data || []) as { id: number; roomNo: string }[]) {
          if (r.roomNo) map.set(r.roomNo.toUpperCase(), r.id)
        }
        setRoomIdByNo(map)
      } catch {
        // Silent — buttons will be disabled if id lookup fails
      }
    }
    loadIds()
    return () => { cancelled = true }
  }, [branchFetch])

  const fetchData = useCallback(async () => {
    try {
      const [statsRes, roomsRes, checkoutsRes, checkInsRes] = await Promise.all([
        branchFetch('/api/stats'),
        branchFetch('/api/rooms'),
        branchFetch('/api/rooms/checkouts-today'),
        branchFetch('/api/checkins?limit=10'),
      ])

      if (statsRes.ok) {
        const data = await statsRes.json()
        if (data.success) {
          const d = data.data
          setStats({
            ...d,
            availableRooms: d.totalRooms - d.occupiedRooms - d.checkoutRooms - d.bookedRooms,
          })
        }
      }

      let checkoutSet = new Set<string>()
      if (checkoutsRes.ok) {
        const data = await checkoutsRes.json()
        if (data.success && data.data) checkoutSet = new Set(data.data)
      }

      if (roomsRes.ok) {
        const data = await roomsRes.json()
        if (data.success && data.data) {
          setRooms(data.data.map((r: ApiRoom) => ({
            roomNumber: r.Room_no,
            type: r.Room_Type?.trim() || '',
            details: r.Room_Details?.trim() || '',
            status: getRoomStatus(r, checkoutSet.has(r.Room_no)),
          })))
        }
      }

      if (checkInsRes.ok) {
        const data = await checkInsRes.json()
        if (data.success && data.data) {
          setCheckIns(data.data.map((c: { Cin_cust_name: string; Cin_Room_No: string; Cin_Room_In: string; Cin_Room_Out: string }) => ({
            guestName: c.Cin_cust_name?.trim() || 'ไม่ระบุ',
            roomNumber: c.Cin_Room_No,
            checkInDate: c.Cin_Room_In,
            checkOutDate: c.Cin_Room_Out,
          })))
        }
      }
    } catch (err) {
      setError('ไม่สามารถโหลดข้อมูลได้')
      console.error('Error fetching dashboard data:', err)
    } finally {
      setLoading(false)
    }
  }, [branchFetch])

  useEffect(() => {
    setLoading(true)
    fetchData()
  }, [fetchData])

  // Keep the SSE effect decoupled from `fetchData`'s identity — otherwise
  // every re-render would tear down and rebuild the EventSource.
  useEffect(() => {
    fetchDataRef.current = fetchData
  }, [fetchData])

  // Live updates: subscribe to /api/events and trigger a debounced refetch
  // whenever a dashboard-relevant DomainEvent arrives. Also refetch when the
  // tab regains visibility (covers laptop sleep / dropped SSE / etc.).
  useEffect(() => {
    const scheduleRefetch = () => {
      if (refetchTimerRef.current) clearTimeout(refetchTimerRef.current)
      refetchTimerRef.current = setTimeout(() => {
        fetchDataRef.current()
      }, REFETCH_DEBOUNCE_MS)
    }

    const eventSource = new EventSource(`/api/events?branch=${encodeURIComponent(branch)}`)
    eventSource.onopen = () => setIsLiveConnected(true)
    eventSource.onerror = () => setIsLiveConnected(false)
    for (const eventName of DASHBOARD_REFRESH_EVENTS) {
      eventSource.addEventListener(eventName, scheduleRefetch)
    }

    const handleVisibilityChange = () => {
      if (document.visibilityState === 'visible') {
        fetchDataRef.current()
      }
    }
    document.addEventListener('visibilitychange', handleVisibilityChange)

    return () => {
      document.removeEventListener('visibilitychange', handleVisibilityChange)
      if (refetchTimerRef.current) {
        clearTimeout(refetchTimerRef.current)
        refetchTimerRef.current = null
      }
      eventSource.close()
      setIsLiveConnected(false)
    }
  }, [branch])

  const roomMap = new Map<string, Room>()
  rooms.forEach(r => roomMap.set(r.roomNumber.toUpperCase(), r))

  // Select room layout based on branch
  const roomLayouts = branch === 'all'
    ? [{ label: 'HF Hotel', layout: hfHotelRoomLayout }, { label: 'HF Ville', layout: hfVilleRoomLayout }]
    : [{ label: '', layout: branch === 'hfville' ? hfVilleRoomLayout : hfHotelRoomLayout }]
  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-[50vh]">
        <div className="text-center">
          <Loader2 className="animate-spin h-8 w-8 text-brand-500 mx-auto mb-2" />
          <p className="text-textMuted text-[13px]">กำลังโหลดข้อมูล...</p>
        </div>
      </div>
    )
  }

  return (
    <div className="space-y-3">
      {/* Page Header — 40px-tall bar with bottom border */}
      <div className="flex items-center justify-between bg-panel border border-border h-10 px-3">
        <div className="flex items-center gap-3">
          <h1 className="text-base font-semibold text-text">หน้าหลัก</h1>
          {branch !== 'hfhotel' && (
            <span className="text-[11px] font-medium px-1.5 py-0.5 bg-headerBar border border-border text-textMuted">
              {BRANCH_LABELS[branch]}
            </span>
          )}
        </div>
        <div className="flex items-center gap-2">
          {/* Live SSE indicator — green when connected, gray while
              reconnecting. Tiny on purpose so it stays informational. */}
          <span
            className={`w-1.5 h-1.5 rounded-full ${isLiveConnected ? 'bg-success' : 'bg-textMuted'}`}
            title={isLiveConnected ? 'อัปเดตอัตโนมัติ: เชื่อมต่ออยู่' : 'อัปเดตอัตโนมัติ: กำลังเชื่อมต่อ'}
            aria-label={isLiveConnected ? 'Live updates connected' : 'Live updates reconnecting'}
          />
          <p className="text-textMuted text-[11px]">
            อัปเดตล่าสุด: {new Date().toLocaleDateString('th-TH', {
              year: 'numeric', month: 'long', day: 'numeric', hour: '2-digit', minute: '2-digit',
            })}
          </p>
        </div>
      </div>

      {error && (
        <div className="flex items-center gap-2 p-2 bg-error/10 border border-error/40 text-error text-[12px]">
          <AlertCircle className="w-4 h-4 shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {/* Stats tiles — flat panels, neutral background, single colour accent (the value text) */}
      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-2">
        <div className="bg-panel border border-border px-3 py-2">
          <p className="text-[11px] uppercase tracking-wide text-textMuted">ห้องทั้งหมด</p>
          <p className="text-[20px] font-semibold text-text leading-tight">{stats.totalRooms}</p>
        </div>
        <div className="bg-panel border border-border px-3 py-2">
          <p className="text-[11px] uppercase tracking-wide text-textMuted">ห้องว่าง</p>
          <p className="text-[20px] font-semibold text-success leading-tight">{stats.availableRooms}</p>
        </div>
        <div className="bg-panel border border-border px-3 py-2">
          <p className="text-[11px] uppercase tracking-wide text-textMuted">มีผู้เข้าพัก</p>
          <p className="text-[20px] font-semibold text-brand-500 leading-tight">{stats.occupiedRooms}</p>
        </div>
        <div className="bg-panel border border-border px-3 py-2">
          <p className="text-[11px] uppercase tracking-wide text-textMuted">จองแล้ว</p>
          <p className="text-[20px] font-semibold text-warning leading-tight">{stats.bookedRooms}</p>
        </div>
        <div className="bg-panel border border-border px-3 py-2">
          <p className="text-[11px] uppercase tracking-wide text-textMuted">เช็คอินวันนี้</p>
          <p className="text-[20px] font-semibold text-text leading-tight">{stats.todayCheckIns}</p>
        </div>
        <div className="bg-panel border border-border px-3 py-2">
          <p className="text-[11px] uppercase tracking-wide text-textMuted">เช็คเอาท์วันนี้</p>
          <p className="text-[20px] font-semibold text-info leading-tight">{stats.todayCheckOuts}</p>
        </div>
      </div>

      {/* Room Grid */}
      <div className="bg-panel border border-border">
        <div className="bg-headerBar border-b border-border px-3 py-1.5 text-[12px] font-semibold uppercase tracking-wide text-text">
          สถานะห้องพัก
        </div>
        <div className="p-3">

        {/* Desktop Grid */}
        <div className="hidden md:block space-y-4">
          {roomLayouts.map((section, sectionIdx) => (
            <div key={sectionIdx}>
              {section.label && (
                <p className="text-[12px] font-semibold text-textMuted uppercase tracking-wide mb-1">{section.label}</p>
              )}
              <div className="grid gap-1" style={{ gridTemplateColumns: `repeat(${Math.max(...section.layout.map(r => r.length))}, minmax(0, 1fr))` }}>
                {section.layout.map((row, rowIndex) => (
                  <div key={`row-${sectionIdx}-${rowIndex}`} className="contents">
                    {row.map((roomNumber, colIndex) => {
                      // Modern skin uses a taller cell (tile + label row below) so blank/filler
                      // slots must match for grid row alignment. Missing-room placeholder stays
                      // 60px since it has no label below.
                      const cellHeightClass = isModernSkin ? 'h-[76px]' : 'h-[60px]'
                      if (roomNumber === null) {
                        return <div key={`blank-${sectionIdx}-${rowIndex}-${colIndex}`} className={cellHeightClass} />
                      }
                      const room = roomMap.get(roomNumber.toUpperCase())
                      if (!room) {
                        return (
                          <div key={`missing-${sectionIdx}-${roomNumber}`} className="h-[60px] bg-headerBar border border-border p-1 flex flex-col items-center justify-center">
                            <span className="font-semibold text-[11px] text-textMuted">{roomNumber}</span>
                            <span className="text-[9px] text-textMuted">ไม่พบ</span>
                          </div>
                        )
                      }
                      const config = statusConfig[room.status]
                      if (isModernSkin) {
                        // Legacy-iHOTEL-inspired layout: white header (room #), colour-filled
                        // body (status label), type/details as a small grey label below the tile.
                        return (
                          <div key={`${sectionIdx}-${roomNumber}`} className="flex flex-col">
                            <button
                              onClick={() => setSelectedRoom(room)}
                              className="border border-border hover:border-borderStrong flex flex-col overflow-hidden transition-colors h-[60px]"
                              title={`${room.roomNumber} - ${room.type} ${room.details}`}
                            >
                              <div className="bg-panel border-b border-border px-1.5 py-0.5 text-left">
                                <span className="font-semibold text-[12px] leading-tight text-text">{room.roomNumber}</span>
                              </div>
                              <div className={`${config.dot} flex-1 flex items-center justify-center`}>
                                <span className="text-[11px] font-medium text-white leading-tight">{config.label}</span>
                              </div>
                            </button>
                            <span className="text-[9px] leading-tight text-textMuted mt-0.5 px-0.5 truncate">
                              {room.type}{room.details ? ` ${room.details}` : ''}
                            </span>
                          </div>
                        )
                      }
                      return (
                        <button
                          key={`${sectionIdx}-${roomNumber}`}
                          onClick={() => setSelectedRoom(room)}
                          className="bg-panel border border-border hover:border-borderStrong p-1 flex flex-col items-center justify-center h-[60px] overflow-hidden transition-colors relative"
                          title={`${room.roomNumber} - ${room.type} ${room.details}`}
                        >
                          {/* Status indicator in the top-right corner — colour without flooding the tile.
                              Shape (round vs square) and size are skin-driven via [data-status] in globals.css. */}
                          <span data-status className={`absolute top-1 right-1 w-2 h-2 rounded-full ${config.dot}`} />
                          <span className="font-semibold text-[12px] leading-tight text-text">{room.roomNumber}</span>
                          <span className="text-[9px] leading-tight text-textMuted">{room.type}</span>
                          <span className="text-[9px] leading-tight text-textMuted/80">{room.details}</span>
                        </button>
                      )
                    })}
                    {Array.from({ length: Math.max(...section.layout.map(r => r.length)) - row.length }).map((_, i) => (
                      <div key={`filler-${sectionIdx}-${rowIndex}-${i}`} className={isModernSkin ? 'h-[76px]' : 'h-[60px]'} />
                    ))}
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>

        {/* Mobile List */}
        <div className="md:hidden space-y-1">
          {[...rooms].sort((a, b) => a.roomNumber.localeCompare(b.roomNumber, undefined, { numeric: true })).map(room => {
            const config = statusConfig[room.status]
            return (
              <button
                key={room.roomNumber}
                onClick={() => setSelectedRoom(room)}
                className="bg-panel hover:bg-headerBar w-full flex items-center gap-3 p-2 border border-border"
              >
                <div data-status className={`w-2 h-2 rounded-full ${config.dot}`} />
                <span className="font-semibold text-text">{room.roomNumber}</span>
                <span className="text-textMuted text-[12px]">{room.type}</span>
                <span className="text-textMuted/80 text-[12px]">{room.details}</span>
              </button>
            )
          })}
        </div>

        {/* Legend */}
        <div className="flex flex-wrap gap-3 mt-3 pt-3 border-t border-border">
          {Object.entries(statusConfig).map(([status, config]) => (
            <div key={status} className="flex items-center gap-1.5">
              <div data-status className={`w-2 h-2 rounded-full ${config.dot}`} />
              <span className="text-[11px] text-textMuted">{config.label}</span>
            </div>
          ))}
        </div>
        </div>
      </div>

      {/* Recent Activity */}
      <div className="bg-panel border border-border">
        <div className="bg-headerBar border-b border-border px-3 py-1.5 text-[12px] font-semibold uppercase tracking-wide text-text flex items-center gap-2">
          <Clock size={12} className="text-brand-500" />
          กิจกรรมล่าสุด
        </div>
        <div className="p-3 space-y-1">
          {checkIns.length > 0 ? (
            checkIns.slice(0, 5).map((checkin, i) => (
              <div key={i} className="flex items-center justify-between p-2 hover:bg-headerBar transition-colors">
                <div className="flex items-center gap-2">
                  <div className="w-6 h-6 bg-brand-50 border border-brand-100 flex items-center justify-center">
                    <LogIn size={12} className="text-brand-700" />
                  </div>
                  <div>
                    <p className="font-medium text-text text-[13px]">{checkin.guestName}</p>
                    <p className="text-[11px] text-textMuted">ห้อง {checkin.roomNumber}</p>
                  </div>
                </div>
                <div className="text-right">
                  <p className="text-[11px] text-textMuted">
                    {new Date(checkin.checkInDate).toLocaleDateString('th-TH', { day: 'numeric', month: 'short', timeZone: 'UTC' })}
                    {' - '}
                    {new Date(checkin.checkOutDate).toLocaleDateString('th-TH', { day: 'numeric', month: 'short', timeZone: 'UTC' })}
                  </p>
                </div>
              </div>
            ))
          ) : (
            <p className="text-textMuted text-center py-3 text-[12px]">ไม่มีกิจกรรมล่าสุด</p>
          )}
        </div>
      </div>

      {/* Room Detail Modal — flat panel, no shadow, no rounded corners */}
      {selectedRoom && (
        <div
          className="fixed inset-0 bg-black/40 flex items-center justify-center z-50"
          onClick={() => setSelectedRoom(null)}
        >
          <div className="bg-panel border border-border max-w-sm w-full mx-4" onClick={e => e.stopPropagation()}>
            <div className="bg-headerBar border-b border-border px-3 py-2 flex items-center justify-between">
              <h3 className="text-[14px] font-semibold text-text">
                ห้อง {selectedRoom.roomNumber}
              </h3>
              <span className="inline-flex items-center gap-1 px-1.5 py-0.5 text-[11px] bg-panel border border-border">
                <span data-status className={`w-2 h-2 rounded-full ${statusConfig[selectedRoom.status].dot}`} />
                {statusConfig[selectedRoom.status].label}
              </span>
            </div>
            <div className="p-3 space-y-2">
              <p className="text-textMuted text-[12px]">
                {selectedRoom.type} {selectedRoom.details}
              </p>
              {/* Action buttons — gated on resolved PG room_id. If the
                  /api/new/rooms lookup hasn't landed yet (or this room
                  isn't in PG), buttons stay disabled. */}
              {selectedRoom.status === 'available' && (
                <button
                  onClick={() => setShowCheckIn(true)}
                  disabled={!roomIdByNo.get(selectedRoom.roomNumber.toUpperCase())}
                  className="w-full h-8 flex items-center justify-center gap-1.5 bg-brand-600 text-white hover:bg-brand-700 disabled:opacity-50 transition-colors text-[13px]"
                >
                  <LogIn size={12} />
                  เช็คอิน
                </button>
              )}
              {selectedRoom.status === 'occupied' && (
                <>
                  {/* Track G1 / T4 HIGH-2: extend-stay (one-more-night) — sits
                      next to checkout so receptionists see both options at
                      the same moment a guest asks for either. */}
                  <button
                    onClick={() => setShowExtendStay(true)}
                    disabled={!roomIdByNo.get(selectedRoom.roomNumber.toUpperCase())}
                    className="w-full h-8 flex items-center justify-center gap-1.5 bg-emerald-600 text-white hover:bg-emerald-700 disabled:opacity-50 transition-colors text-[13px]"
                  >
                    <CalendarPlus size={12} />
                    ขยายเวลาเข้าพัก
                  </button>
                  {/* Track G4 / T4 HIGH-3: change-room (mid-stay move). Sits
                      between extend and checkout so receptionists see the
                      full lifecycle action set on an occupied tile. */}
                  <button
                    onClick={() => setShowChangeRoom(true)}
                    disabled={!roomIdByNo.get(selectedRoom.roomNumber.toUpperCase())}
                    className="w-full h-8 flex items-center justify-center gap-1.5 bg-amber-600 text-white hover:bg-amber-700 disabled:opacity-50 transition-colors text-[13px]"
                  >
                    <ArrowRightLeft size={12} />
                    เปลี่ยนห้อง
                  </button>
                  {/* Track G6: POS / charge product to folio. Sits next to
                      change-room because both are "during-stay" actions. */}
                  <button
                    onClick={() => setShowPosSale(true)}
                    disabled={!roomIdByNo.get(selectedRoom.roomNumber.toUpperCase())}
                    className="w-full h-8 flex items-center justify-center gap-1.5 bg-orange-600 text-white hover:bg-orange-700 disabled:opacity-50 transition-colors text-[13px]"
                  >
                    <Coffee size={12} />
                    เพิ่มรายการในบิล
                  </button>
                  <button
                    onClick={() => setShowCheckOut(true)}
                    disabled={!roomIdByNo.get(selectedRoom.roomNumber.toUpperCase())}
                    className="w-full h-8 flex items-center justify-center gap-1.5 bg-info text-white hover:opacity-90 disabled:opacity-50 transition-colors text-[13px]"
                  >
                    <LogOut size={12} />
                    เช็คเอ้าท์
                  </button>
                </>
              )}
              <button
                onClick={() => setSelectedRoom(null)}
                className="w-full h-7 bg-panel border border-borderStrong text-text hover:bg-headerBar transition-colors text-[13px]"
              >
                ปิด
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Check-in / check-out modals */}
      {showCheckIn && selectedRoom && roomIdByNo.get(selectedRoom.roomNumber.toUpperCase()) && (
        <CheckInModal
          room={{
            id: roomIdByNo.get(selectedRoom.roomNumber.toUpperCase())!,
            roomNo: selectedRoom.roomNumber,
            roomTypeName: selectedRoom.type,
          }}
          onClose={() => setShowCheckIn(false)}
          onSuccess={() => {
            fetchData()
            setSelectedRoom(null)
          }}
        />
      )}
      {showCheckOut && selectedRoom && roomIdByNo.get(selectedRoom.roomNumber.toUpperCase()) && (
        <CheckOutModal
          room={{
            id: roomIdByNo.get(selectedRoom.roomNumber.toUpperCase())!,
            roomNo: selectedRoom.roomNumber,
          }}
          onClose={() => setShowCheckOut(false)}
          onSuccess={() => {
            fetchData()
            setSelectedRoom(null)
          }}
        />
      )}
      {showExtendStay && selectedRoom && roomIdByNo.get(selectedRoom.roomNumber.toUpperCase()) && (
        <ExtendStayModal
          room={{
            id: roomIdByNo.get(selectedRoom.roomNumber.toUpperCase())!,
            roomNo: selectedRoom.roomNumber,
          }}
          onClose={() => setShowExtendStay(false)}
          onSuccess={() => {
            fetchData()
            setSelectedRoom(null)
          }}
        />
      )}
      {showChangeRoom && selectedRoom && roomIdByNo.get(selectedRoom.roomNumber.toUpperCase()) && (
        <ChangeRoomModal
          room={{
            id: roomIdByNo.get(selectedRoom.roomNumber.toUpperCase())!,
            roomNo: selectedRoom.roomNumber,
          }}
          onClose={() => setShowChangeRoom(false)}
          onSuccess={() => {
            fetchData()
            setSelectedRoom(null)
          }}
        />
      )}
      {showPosSale && selectedRoom && roomIdByNo.get(selectedRoom.roomNumber.toUpperCase()) && (
        <PosSaleModal
          room={{
            id: roomIdByNo.get(selectedRoom.roomNumber.toUpperCase())!,
            roomNo: selectedRoom.roomNumber,
          }}
          onClose={() => setShowPosSale(false)}
          onSuccess={() => {
            // Don't reset selectedRoom — cashiers often ring up several
            // lines back-to-back, and the modal handles its own refresh.
            fetchData()
          }}
        />
      )}
    </div>
  )
}
