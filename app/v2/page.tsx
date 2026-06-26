'use client'

import { useCallback, useEffect, useRef, useState } from 'react'
import Link from 'next/link'
import {
  CalendarPlus,
  LogIn,
  LogOut,
  ArrowRight,
  AlertTriangle,
  ChevronRight,
  Moon,
} from 'lucide-react'
import { useBranch } from '@/contexts/BranchContext'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { formatStoredDayMonth, formatStoredDateTime } from '@/lib/format'
import type { Booking } from '@/types/booking'
import { isSameStoredDay } from '@/lib/v2/status'
import { V2Spinner, LiveDot, VilleNotice } from '@/components/v2/primitives'

// Mirrors the /api/stats payload. NOTE: there is no `availableRooms` field —
// it is derived client-side (totalRooms - occupied - checkout - booked), the
// same way the classic dashboard does it (app/page.tsx).
interface Stats {
  totalRooms: number
  occupiedRooms: number
  bookedRooms: number
  checkoutRooms: number
  totalCustomers: number
  activeBookings: number
  todayCheckIns: number
  todayCheckOuts: number
}

interface CheckInRow {
  id: number
  cinNo: string
  customerName: string | null
  roomNo: string | null
  roomTypeName: string | null
  checkInTime: string | null
  checkOutTime: string | null
  expectedCheckout: string | null
  status: string
}

const REFRESH_EVENTS = [
  'CheckInCreated',
  'CheckOutCompleted',
  'CheckInCancelled',
  'BookingCreated',
  'BookingModified',
  'BookingCancelled',
  'RoomMarkedClean',
  'RoomMarkedDirty',
]

function flowTone(n: number, tone: string) {
  return n > 0 ? tone : 's-mut'
}

export default function V2Today() {
  const { branch } = useBranch()
  const branchFetch = useBranchFetch()
  const [stats, setStats] = useState<Stats | null>(null)
  const [checkins, setCheckins] = useState<CheckInRow[]>([])
  const [bookings, setBookings] = useState<Booking[]>([])
  const [shiftOpen, setShiftOpen] = useState<boolean | null>(null)
  const [loading, setLoading] = useState(true)
  const [live, setLive] = useState(false)

  const fetchRef = useRef<() => void>(() => {})
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null)

  const fetchData = useCallback(async () => {
    try {
      const [statsRes, cinRes, bookRes, shiftRes] = await Promise.all([
        branchFetch('/api/stats'),
        branchFetch('/api/checkins?limit=100'),
        branchFetch('/api/bookings?limit=100'),
        branchFetch('/api/shifts/current').catch(() => null),
      ])

      if (statsRes.ok) {
        const d = await statsRes.json()
        if (d.success) setStats(d.data)
      }
      if (cinRes.ok) {
        const d = await cinRes.json()
        setCheckins((d.data || []) as CheckInRow[])
      }
      if (bookRes.ok) {
        const d = await bookRes.json()
        setBookings((d.data || []) as Booking[])
      }
      if (shiftRes && shiftRes.ok) {
        const d = await shiftRes.json()
        setShiftOpen(Boolean(d?.data || d?.shift))
      } else if (shiftRes && shiftRes.status === 404) {
        // /api/shifts/current returns 404 when no shift is open — that's
        // the "round not open" state the banner exists to warn about.
        setShiftOpen(false)
      } else {
        setShiftOpen(null)
      }
    } catch {
      /* surfaces as empty states */
    } finally {
      setLoading(false)
    }
  }, [branchFetch])

  // `loading` starts true and is cleared in fetchData's finally. We deliberately
  // do NOT re-raise it on refetch (branch switch / SSE) so live updates don't
  // flash the full-page spinner — the screen updates in place.
  useEffect(() => {
    fetchData()
  }, [fetchData])

  useEffect(() => {
    fetchRef.current = fetchData
  }, [fetchData])

  useEffect(() => {
    const schedule = () => {
      if (timer.current) clearTimeout(timer.current)
      timer.current = setTimeout(() => fetchRef.current(), 500)
    }
    const es = new EventSource(`/api/events?branch=${encodeURIComponent(branch)}`)
    es.onopen = () => setLive(true)
    es.onerror = () => setLive(false)
    REFRESH_EVENTS.forEach((e) => es.addEventListener(e, schedule))
    const onVis = () => document.visibilityState === 'visible' && fetchRef.current()
    document.addEventListener('visibilitychange', onVis)
    return () => {
      document.removeEventListener('visibilitychange', onVis)
      if (timer.current) clearTimeout(timer.current)
      es.close()
      setLive(false)
    }
  }, [branch])

  if (loading) return <V2Spinner label="กำลังโหลดข้อมูลหน้าบ้าน…" />

  const inHouse = checkins.filter((c) => c.status === 'active' || c.status === 'checkedin')
  const departures = inHouse.filter((c) => isSameStoredDay(c.expectedCheckout))
  const arrivals = bookings.filter(
    (b) => isSameStoredDay(b.checkIn) && ['pending', 'confirmed'].includes(b.status),
  )

  const total = stats?.totalRooms ?? 0
  const occupied = stats?.occupiedRooms ?? 0
  const pct = total > 0 ? Math.round((occupied / total) * 100) : 0
  // /api/stats has no availableRooms field — derive it exactly like the classic
  // dashboard (app/page.tsx) so the two UIs agree.
  const available = Math.max(
    0,
    total - occupied - (stats?.checkoutRooms ?? 0) - (stats?.bookedRooms ?? 0),
  )

  const today = new Date()
  const dateLabel = today.toLocaleDateString('th-TH', {
    weekday: 'long',
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  })

  return (
    <div className="space-y-5">
      {/* Header */}
      <div className="flex flex-wrap items-end justify-between gap-3">
        <div>
          <div className="v2-eyebrow mb-1 flex items-center gap-2">
            <span>หน้าหลัก</span>
            <LiveDot connected={live} />
          </div>
          <h1 className="v2-display text-[26px] lg:text-[32px] leading-none">{dateLabel}</h1>
        </div>
        {/* Write actions only for HF Hotel; Ville/all are view-only (writes
            would otherwise misroute to the HF Hotel pool). */}
        {branch === 'hfhotel' && (
          <div className="flex items-center gap-2">
            <Link href="/v2/reservations?new=1" className="v2-btn v2-btn-primary">
              <CalendarPlus size={17} /> จองใหม่
            </Link>
            <Link href="/v2/rooms" className="v2-btn v2-btn-ghost">
              <LogIn size={17} /> เช็คอิน
            </Link>
          </div>
        )}
      </div>

      <VilleNotice branch={branch} />

      {/* Shift gate banner */}
      {shiftOpen === false && (
        <div
          className="v2-card flex items-center gap-3 px-4 py-3"
          style={{ borderColor: 'var(--v2-arr)', background: 'var(--v2-arr-bg)' }}
        >
          <AlertTriangle size={18} style={{ color: 'var(--v2-arr)' }} />
          <p className="text-[13.5px] flex-1" style={{ color: 'var(--v2-ink)' }}>
            ยังไม่ได้เปิดรอบขาย — การเช็คอิน/เช็คเอาท์จะถูกระงับจนกว่าจะเปิดรอบ
          </p>
          <Link href="/billing" className="v2-btn v2-btn-soft v2-btn-sm">เปิดรอบ</Link>
        </div>
      )}

      {/* Hero: occupancy pulse + day flow */}
      <div className="grid lg:grid-cols-[360px_1fr] gap-4">
        {/* Occupancy gauge */}
        <div className="v2-card v2-rise v2-rise-1 p-5 lg:p-6 flex items-center gap-5">
          <div
            className="v2-gauge"
            style={{ ['--pct' as string]: pct } as React.CSSProperties}
          >
            <div className="text-center">
              <div className="v2-num text-[34px] font-bold leading-none">{pct}%</div>
              <div className="text-[11px] mt-0.5" style={{ color: 'var(--v2-ink-3)' }}>เข้าพัก</div>
            </div>
          </div>
          <div className="flex-1 space-y-3">
            <div className="v2-eyebrow">อัตราการเข้าพัก</div>
            <div className="flex items-baseline gap-1.5">
              <span className="v2-num text-[22px] font-bold">{occupied}</span>
              <span className="text-[13px]" style={{ color: 'var(--v2-ink-3)' }}>/ {total} ห้อง</span>
            </div>
            <div className="grid grid-cols-2 gap-2 pt-1">
              <MiniStat label="ว่าง" value={available} dot="d-ok" />
              <MiniStat label="จองแล้ว" value={stats?.bookedRooms ?? 0} dot="d-arr" />
            </div>
          </div>
        </div>

        {/* Day flow: arrivals → in-house → departures */}
        <div className="v2-card v2-rise v2-rise-2 p-5 lg:p-6">
          <div className="v2-eyebrow mb-4">จังหวะของวันนี้</div>
          <div className="flex items-stretch">
            <FlowStep
              icon={<LogIn size={18} />}
              label="กำลังจะมาถึง"
              value={arrivals.length}
              tone={flowTone(arrivals.length, 's-arr')}
              href="/v2/reservations"
            />
            <FlowConnector />
            <FlowStep
              icon={<Moon size={18} />}
              label="พักอยู่ตอนนี้"
              value={inHouse.length}
              tone={flowTone(inHouse.length, 's-occ')}
              href="/v2/rooms"
            />
            <FlowConnector />
            <FlowStep
              icon={<LogOut size={18} />}
              label="ออกวันนี้"
              value={departures.length}
              tone={flowTone(departures.length, 's-dep')}
              href="/v2/rooms"
            />
          </div>
        </div>
      </div>

      {/* Lists */}
      <div className="grid lg:grid-cols-3 gap-4">
        <ListCard
          title="กำลังจะมาถึง"
          count={arrivals.length}
          tone="s-arr"
          href="/v2/reservations"
          empty="ไม่มีการเข้าพักวันนี้"
          className="v2-rise v2-rise-2"
        >
          {arrivals.slice(0, 6).map((b) => (
            <GuestRow
              key={b.id}
              href="/v2/reservations"
              name={b.customerName || 'ไม่ระบุชื่อ'}
              room={b.roomCount > 0 ? `${b.roomCount} ห้อง` : b.bookNo}
              sub={`${formatStoredDayMonth(b.checkIn)} – ${formatStoredDayMonth(b.checkOut)}`}
              dot="d-arr"
            />
          ))}
        </ListCard>

        <ListCard
          title="พักอยู่ตอนนี้"
          count={inHouse.length}
          tone="s-occ"
          href="/v2/rooms"
          empty="ยังไม่มีผู้เข้าพัก"
          className="v2-rise v2-rise-3"
        >
          {inHouse.slice(0, 6).map((c) => (
            <GuestRow
              key={c.id}
              href="/v2/rooms"
              name={c.customerName || 'ไม่ระบุชื่อ'}
              room={c.roomNo ? `ห้อง ${c.roomNo}` : c.cinNo}
              sub={`ถึง ${formatStoredDayMonth(c.expectedCheckout)}`}
              dot="d-occ"
            />
          ))}
        </ListCard>

        <ListCard
          title="ออกวันนี้"
          count={departures.length}
          tone="s-dep"
          href="/v2/rooms"
          empty="ไม่มีการเช็คเอาท์วันนี้"
          className="v2-rise v2-rise-4"
        >
          {departures.slice(0, 6).map((c) => (
            <GuestRow
              key={c.id}
              href="/v2/rooms"
              name={c.customerName || 'ไม่ระบุชื่อ'}
              room={c.roomNo ? `ห้อง ${c.roomNo}` : c.cinNo}
              sub={`เข้า ${formatStoredDateTime(c.checkInTime)}`}
              dot="d-dep"
            />
          ))}
        </ListCard>
      </div>
    </div>
  )
}

function MiniStat({ label, value, dot }: { label: string; value: number; dot: string }) {
  return (
    <div className="v2-inset px-3 py-2 flex items-center gap-2">
      <span className={`v2-dot ${dot}`} />
      <span className="v2-num text-[17px] font-bold">{value}</span>
      <span className="text-[12px]" style={{ color: 'var(--v2-ink-3)' }}>{label}</span>
    </div>
  )
}

function FlowStep({
  icon,
  label,
  value,
  tone,
  href,
}: {
  icon: React.ReactNode
  label: string
  value: number
  tone: string
  href: string
}) {
  return (
    <Link href={href} className="flex-1 flex flex-col items-center text-center gap-2 group">
      <span className={`v2-pill ${tone}`} style={{ width: 44, height: 44, borderRadius: 999, justifyContent: 'center' }}>
        {icon}
      </span>
      <span className="v2-num text-[30px] font-bold leading-none">{value}</span>
      <span className="text-[12.5px] group-hover:underline" style={{ color: 'var(--v2-ink-2)' }}>{label}</span>
    </Link>
  )
}

function FlowConnector() {
  return (
    <div className="flex items-center px-1 lg:px-3 pt-3.5">
      <ArrowRight size={18} style={{ color: 'var(--v2-line-2)' }} />
    </div>
  )
}

function ListCard({
  title,
  count,
  tone,
  href,
  empty,
  className,
  children,
}: {
  title: string
  count: number
  tone: string
  href: string
  empty: string
  className?: string
  children: React.ReactNode
}) {
  return (
    <section className={`v2-card overflow-hidden ${className || ''}`}>
      <div className="flex items-center gap-2 px-4 py-3" style={{ borderBottom: '1px solid var(--v2-line)' }}>
        <h2 className="text-[15px] font-semibold">{title}</h2>
        <span className={`v2-num text-[12px] font-bold px-2 py-0.5 rounded-full ${tone}`}>{count}</span>
        <div className="flex-1" />
        <Link href={href} className="text-[12px] flex items-center gap-0.5" style={{ color: 'var(--v2-ink-3)' }}>
          ดูทั้งหมด <ChevronRight size={13} />
        </Link>
      </div>
      <div className="divide-y" style={{ borderColor: 'var(--v2-line)' }}>
        {count === 0 ? (
          <div className="px-5 py-9 text-center text-[13px]" style={{ color: 'var(--v2-ink-3)' }}>{empty}</div>
        ) : (
          children
        )}
      </div>
    </section>
  )
}

function GuestRow({
  href,
  name,
  room,
  sub,
  dot,
}: {
  href: string
  name: string
  room: string
  sub: string
  dot: string
}) {
  return (
    <Link
      href={href}
      className="flex items-center gap-3 px-4 py-2.5 transition-colors"
      style={{ borderColor: 'var(--v2-line)' }}
    >
      <span className={`v2-dot ${dot}`} style={{ width: 9, height: 9 }} />
      <div className="flex-1 min-w-0">
        <div className="text-[14px] font-medium truncate">{name}</div>
        <div className="text-[12px]" style={{ color: 'var(--v2-ink-3)' }}>{sub}</div>
      </div>
      <span className="v2-num text-[13px] font-semibold" style={{ color: 'var(--v2-ink-2)' }}>{room}</span>
    </Link>
  )
}
