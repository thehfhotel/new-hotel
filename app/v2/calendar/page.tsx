'use client'

import { Fragment, useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { ChevronLeft, ChevronRight, CalendarRange } from 'lucide-react'
import { useBranch } from '@/contexts/BranchContext'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useLiveRefresh } from '@/lib/v2/use-live-refresh'
import { V2PageHeader, V2Spinner, V2Empty } from '@/components/v2/primitives'
import {
  buildGrid,
  cellState,
  dayRange,
  toDayKey,
  type CalendarEntry,
  type CellState,
} from '@/lib/v2/calendar-grid'

/**
 * Availability calendar grid (task #53) — rows = rooms, columns = dates.
 * READ-ONLY: fed entirely by GET /api/calendar (both-site bookings + check-ins)
 * and GET /api/rooms (the room roster). Branch-aware via useBranchFetch.
 * Each cell shows occupied (active stay) / booked (reservation) / free.
 */

const WINDOW_DAYS = 14

interface RoomRow {
  roomNo: string
  floor: number | null
}

interface CalendarApiEntry extends CalendarEntry {
  id?: string
  status?: string | null
}

interface CalendarApiResponse {
  success: boolean
  data: { bookings: CalendarApiEntry[]; checkins: CalendarApiEntry[] }
}

const CELL_TONE: Record<CellState, { bg: string; fg: string; label: string }> = {
  free: { bg: 'var(--v2-surface)', fg: 'var(--v2-ink-3)', label: 'ว่าง' },
  booked: { bg: 'var(--v2-arr-bg)', fg: 'var(--v2-arr)', label: 'จองแล้ว' },
  occupied: { bg: 'var(--v2-occ-bg)', fg: 'var(--v2-occ)', label: 'เข้าพัก' },
}

/** Build a Date from a local "YYYY-MM-DD" key without timezone drift. */
function dateFromKey(key: string): Date {
  const [y, m, d] = key.split('-').map(Number)
  return new Date(y, m - 1, d)
}

export default function V2Calendar() {
  const { branch } = useBranch()
  const branchFetch = useBranchFetch()

  // Window anchor: midnight of the first visible day (default = today).
  const [anchor, setAnchor] = useState(() => {
    const d = new Date()
    d.setHours(0, 0, 0, 0)
    return d
  })
  const [rooms, setRooms] = useState<RoomRow[]>([])
  const [bookings, setBookings] = useState<CalendarApiEntry[]>([])
  const [checkins, setCheckins] = useState<CalendarApiEntry[]>([])
  const [loading, setLoading] = useState(true)
  const reqRef = useRef(0)

  const days = useMemo(() => dayRange(anchor, WINDOW_DAYS), [anchor])
  const today = useMemo(() => toDayKey(new Date()), [])

  const load = useCallback(async () => {
    const token = ++reqRef.current
    setLoading(true)
    try {
      const startDate = days[0]
      const endDate = days[days.length - 1]
      const [roomsRes, calRes] = await Promise.all([
        branchFetch('/api/rooms?limit=300'),
        branchFetch(`/api/calendar?startDate=${startDate}&endDate=${endDate}`),
      ])
      if (token !== reqRef.current) return // superseded by a newer fetch

      if (roomsRes.ok) {
        const data = await roomsRes.json()
        const list = (data.data || data || []) as Array<{ roomNo: string; floor: number | null }>
        setRooms(
          list
            .map((r) => ({ roomNo: r.roomNo, floor: r.floor ?? null }))
            .sort(
              (a, b) =>
                (a.floor ?? 0) - (b.floor ?? 0) ||
                a.roomNo.localeCompare(b.roomNo, undefined, { numeric: true }),
            ),
        )
      }
      if (calRes.ok) {
        const data = (await calRes.json()) as CalendarApiResponse
        setBookings(data.data?.bookings || [])
        setCheckins(data.data?.checkins || [])
      }
    } catch {
      /* leave prior state; empty view handles no-data */
    } finally {
      if (token === reqRef.current) setLoading(false)
    }
  }, [branchFetch, days])

  useEffect(() => {
    load()
  }, [load])

  // Live-refresh when a booking/check-in changes in iHOTEL or the other app.
  useLiveRefresh(
    branch,
    ['BookingCreated', 'BookingModified', 'BookingCancelled', 'CheckInCreated', 'CheckOutCompleted', 'CheckInCancelled'],
    load,
  )

  const roomNos = useMemo(() => rooms.map((r) => r.roomNo), [rooms])
  const grid = useMemo(
    () => buildGrid({ rooms: roomNos, days, bookings, checkins }),
    [roomNos, days, bookings, checkins],
  )

  // Occupancy of the first visible column (a quick "today/anchor" glance).
  const anchorOccupancy = useMemo(() => {
    if (roomNos.length === 0) return 0
    const used = roomNos.filter((r) => cellState(grid, r, days[0]) !== 'free').length
    return Math.round((used / roomNos.length) * 100)
  }, [grid, roomNos, days])

  const shift = (deltaDays: number) =>
    setAnchor((prev) => {
      const d = new Date(prev)
      d.setDate(prev.getDate() + deltaDays)
      return d
    })

  const goToday = () => {
    const d = new Date()
    d.setHours(0, 0, 0, 0)
    setAnchor(d)
  }

  return (
    <div className="space-y-5">
      <V2PageHeader
        eyebrow="ปฏิทินห้องว่าง"
        title="ผังห้องว่างตามวันที่"
        right={
          <div className="flex items-center gap-2">
            <button className="v2-btn v2-btn-ghost v2-btn-sm" onClick={() => shift(-WINDOW_DAYS)} aria-label="ช่วงก่อนหน้า">
              <ChevronLeft size={16} />
            </button>
            <button className="v2-btn v2-btn-soft v2-btn-sm" onClick={goToday}>
              วันนี้
            </button>
            <button className="v2-btn v2-btn-ghost v2-btn-sm" onClick={() => shift(WINDOW_DAYS)} aria-label="ช่วงถัดไป">
              <ChevronRight size={16} />
            </button>
          </div>
        }
      />

      {/* Legend + occupancy glance */}
      <div className="flex flex-wrap items-center gap-x-4 gap-y-1.5">
        {(['occupied', 'booked', 'free'] as CellState[]).map((s) => (
          <span key={s} className="inline-flex items-center gap-1.5 text-[12px]" style={{ color: 'var(--v2-ink-3)' }}>
            <span
              className="inline-block w-3 h-3 rounded-[4px]"
              style={{ background: CELL_TONE[s].bg, border: '1px solid var(--v2-line-2)' }}
            />
            {CELL_TONE[s].label}
          </span>
        ))}
        <span className="inline-flex items-center gap-1.5 text-[12px] ml-auto" style={{ color: 'var(--v2-ink-3)' }}>
          <CalendarRange size={13} /> เข้าพัก {anchorOccupancy}% ({days[0]})
        </span>
      </div>

      {loading && rooms.length === 0 ? (
        <V2Spinner label="กำลังโหลดปฏิทิน…" />
      ) : rooms.length === 0 ? (
        <V2Empty title="ไม่มีห้องพัก" hint="ยังไม่มีข้อมูลห้องสำหรับสาขานี้" />
      ) : (
        <div className="v2-card overflow-auto" style={{ maxHeight: '72vh' }}>
          <div
            className="grid text-[12px]"
            style={{
              gridTemplateColumns: `64px repeat(${days.length}, minmax(40px, 1fr))`,
              minWidth: `${64 + days.length * 40}px`,
            }}
          >
            {/* Header: corner + dates */}
            <div
              className="sticky top-0 left-0 z-30 flex items-center justify-center font-semibold"
              style={{ background: 'var(--v2-surface-2)', borderBottom: '1px solid var(--v2-line)', borderRight: '1px solid var(--v2-line)', height: 44, color: 'var(--v2-ink-2)' }}
            >
              ห้อง
            </div>
            {days.map((d) => {
              const date = dateFromKey(d)
              const isToday = d === today
              const dom = date.getDate()
              const wd = date.toLocaleDateString('th-TH', { weekday: 'short' })
              return (
                <div
                  key={d}
                  className="sticky top-0 z-20 flex flex-col items-center justify-center"
                  style={{
                    background: isToday ? 'var(--v2-wine-50)' : 'var(--v2-surface-2)',
                    borderBottom: `2px solid ${isToday ? 'var(--v2-wine-500)' : 'var(--v2-line)'}`,
                    borderRight: '1px solid var(--v2-line)',
                    height: 44,
                  }}
                  title={d}
                >
                  <span className="v2-num font-bold leading-none" style={{ color: isToday ? 'var(--v2-wine-700)' : 'var(--v2-ink)' }}>
                    {dom}
                  </span>
                  <span className="text-[10px] leading-none mt-0.5" style={{ color: 'var(--v2-ink-3)' }}>
                    {wd}
                  </span>
                </div>
              )
            })}

            {/* Body: one row per room */}
            {rooms.map((room) => (
              <Fragment key={room.roomNo}>
                <div
                  className="sticky left-0 z-10 flex items-center justify-center v2-num font-semibold"
                  style={{
                    background: 'var(--v2-surface)',
                    borderBottom: '1px solid var(--v2-line)',
                    borderRight: '1px solid var(--v2-line)',
                    height: 34,
                    color: 'var(--v2-ink)',
                  }}
                  title={room.floor != null ? `ห้อง ${room.roomNo} · ชั้น ${room.floor}` : `ห้อง ${room.roomNo}`}
                >
                  {room.roomNo}
                </div>
                {days.map((d) => {
                  const state = cellState(grid, room.roomNo, d)
                  const tone = CELL_TONE[state]
                  return (
                    <div
                      key={`${room.roomNo}|${d}`}
                      className="flex items-center justify-center"
                      style={{
                        background: tone.bg,
                        borderBottom: '1px solid var(--v2-line)',
                        borderRight: '1px solid var(--v2-line)',
                        height: 34,
                      }}
                      title={`ห้อง ${room.roomNo} · ${d} · ${tone.label}`}
                    >
                      {state !== 'free' && (
                        <span className="w-2 h-2 rounded-full" style={{ background: tone.fg }} />
                      )}
                    </div>
                  )
                })}
              </Fragment>
            ))}
          </div>
        </div>
      )}
    </div>
  )
}
