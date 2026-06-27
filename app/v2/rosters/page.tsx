'use client'

import { useCallback, useEffect, useRef, useState, type ReactNode } from 'react'
import { Printer, LogIn, LogOut, Moon, CalendarClock, type LucideIcon } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useBranch, BRANCH_LABELS } from '@/contexts/BranchContext'
import { printRegion } from '@/lib/print'
import { formatStoredDayMonth, formatStoredDateTime, formatStoredDayMonthYear } from '@/lib/format'
import { V2PageHeader, V2Spinner } from '@/components/v2/primitives'

/**
 * Daily guest roster / desk paperwork — task #43. Front-desk "who's in the
 * building today" lists, printable as A4. Consumes the branch-aware
 * GET /api/rosters/* endpoints, which return the FULL lists (the v2 dashboard
 * glances cap at 100 and under-count a busy day). Each roster is its own
 * `.v2-print` region with an A4 print button (lib/print.ts `printRegion`, the
 * v2 print idiom — see app/v2/rounds/page.tsx).
 */

interface StayRow {
  branch: string
  cinId: number
  cinNo: string
  bookingNo: string | null
  customerName: string | null
  phone: string | null
  roomNo: string | null
  roomTypeName: string | null
  checkInTime: string | null
  checkOutTime: string | null
  expectedCheckout: string | null
  adults: number | null
  children: number | null
  nights: number | null
  ratePerNight: number | null
  totalAmount: number | null
  notes: string | null
}

interface ArrivalRow {
  branch: string
  bookId: number
  bookNo: string
  customerName: string | null
  phone: string | null
  checkIn: string | null
  checkOut: string | null
  adults: number | null
  children: number | null
  nights: number | null
  roomCount: number
  roomNos: string | null
  status: string
}

interface Summary {
  date: string
  arrivals: number
  inHouse: number
  departures: number
  continuing: number
  inHouseGuests: number
}

/** YYYY-MM-DD in Bangkok time, `daysAgo` days back from today. */
function bkkDate(daysAgo = 0): string {
  const now = new Date(Date.now() - daysAgo * 86400000)
  return now.toLocaleDateString('en-CA', { timeZone: 'Asia/Bangkok' }) // en-CA → YYYY-MM-DD
}

function guestCount(adults: number | null, children: number | null): string {
  const a = adults ?? 0
  const c = children ?? 0
  return c > 0 ? `${a}+${c}` : String(a)
}

export default function V2Rosters() {
  const branchFetch = useBranchFetch()
  const { branch } = useBranch()
  const [date, setDate] = useState(() => bkkDate(0))
  const [loading, setLoading] = useState(true)
  const [summary, setSummary] = useState<Summary | null>(null)
  const [arrivals, setArrivals] = useState<ArrivalRow[]>([])
  const [inHouse, setInHouse] = useState<StayRow[]>([])
  const [departures, setDepartures] = useState<StayRow[]>([])
  const [continuing, setContinuing] = useState<StayRow[]>([])
  // Latest-wins guard: branch/date can flip mid-flight.
  const reqRef = useRef(0)

  const fetchAll = useCallback(async () => {
    const token = ++reqRef.current
    setLoading(true)
    try {
      const qs = `date=${encodeURIComponent(date)}`
      const [sumRes, arrRes, inRes, depRes, contRes] = await Promise.all([
        branchFetch(`/api/rosters/summary?${qs}`),
        branchFetch(`/api/rosters/arrivals?${qs}`),
        branchFetch(`/api/rosters/in-house?${qs}`),
        branchFetch(`/api/rosters/departures?${qs}`),
        branchFetch(`/api/rosters/continuing?${qs}`),
      ])
      if (token !== reqRef.current) return // superseded by a newer fetch
      if (sumRes.ok) setSummary(await sumRes.json())
      else setSummary(null)
      setArrivals(arrRes.ok ? ((await arrRes.json()).data ?? []) : [])
      setInHouse(inRes.ok ? ((await inRes.json()).data ?? []) : [])
      setDepartures(depRes.ok ? ((await depRes.json()).data ?? []) : [])
      setContinuing(contRes.ok ? ((await contRes.json()).data ?? []) : [])
    } catch {
      if (token === reqRef.current) {
        setSummary(null)
        setArrivals([])
        setInHouse([])
        setDepartures([])
        setContinuing([])
      }
    } finally {
      if (token === reqRef.current) setLoading(false)
    }
  }, [branchFetch, date])

  useEffect(() => {
    fetchAll()
  }, [fetchAll])

  const dateLabel = formatStoredDayMonthYear(summary?.date ?? date)
  const branchLabel = branch === 'all' ? BRANCH_LABELS.all : BRANCH_LABELS[branch]
  const showBranchTag = branch === 'all'

  return (
    <div className="space-y-5">
      <V2PageHeader eyebrow="รายงานหน้าบ้าน · Daily roster" title="รายชื่อผู้เข้าพักประจำวัน" />

      {/* Date picker */}
      <div className="flex flex-wrap items-end gap-3">
        <label className="flex flex-col gap-1">
          <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>
            ประจำวันที่ · Date
          </span>
          <input
            type="date"
            value={date}
            max={bkkDate(-1 /* allow up to tomorrow for arrivals prep */)}
            onChange={(e) => setDate(e.target.value)}
            className="v2-num h-9 px-3 rounded-[10px] text-[14px] outline-none"
            style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
          />
        </label>
        <button
          onClick={() => setDate(bkkDate(0))}
          className="v2-btn v2-btn-ghost v2-btn-sm"
          disabled={date === bkkDate(0)}
        >
          วันนี้
        </button>
      </div>

      {loading ? (
        <V2Spinner label="กำลังโหลดรายชื่อผู้เข้าพัก…" />
      ) : (
        <>
          {/* Summary counts */}
          <div className="grid gap-3 grid-cols-2 lg:grid-cols-5">
            <StatCard label="กำลังจะมาถึง" sub="Arrivals" value={summary?.arrivals ?? arrivals.length} dot="d-arr" />
            <StatCard label="พักอยู่ตอนนี้" sub="In-house" value={summary?.inHouse ?? inHouse.length} dot="d-occ" accent />
            <StatCard label="ออกวันนี้" sub="Departures" value={summary?.departures ?? departures.length} dot="d-dep" />
            <StatCard label="พักต่อ" sub="Continuing" value={summary?.continuing ?? continuing.length} dot="d-occ" />
            <StatCard label="จำนวนแขก (พักอยู่)" sub="Guests in-house" value={summary?.inHouseGuests ?? 0} />
          </div>

          {/* Arrivals */}
          <RosterSection
            id="roster-arrivals"
            icon={LogIn}
            title="กำลังจะมาถึง"
            titleEn="Arrivals"
            count={arrivals.length}
            dateLabel={dateLabel}
            branchLabel={branchLabel}
          >
            {arrivals.length === 0 ? (
              <EmptyRow text="ไม่มีการเข้าพักในวันนี้ · No arrivals" />
            ) : (
              <table className="w-full text-[13px] border-collapse">
                <thead>
                  <Th cols={['การจอง', 'ชื่อแขก', 'โทร', 'ห้อง', 'เข้า → ออก', 'คืน', 'แขก', 'สถานะ']} nums={[5, 6]} />
                </thead>
                <tbody>
                  {arrivals.map((a) => (
                    <tr key={`${a.branch}-${a.bookId}`} style={{ borderBottom: '1px solid var(--v2-line)' }}>
                      <Td>
                        <BranchTag show={showBranchTag} branch={a.branch} />
                        <span className="v2-num">{a.bookNo}</span>
                      </Td>
                      <Td strong>{a.customerName || 'ไม่ระบุชื่อ'}</Td>
                      <Td mut>{a.phone || '—'}</Td>
                      <Td>{a.roomNos || (a.roomCount > 0 ? `${a.roomCount} ห้อง` : 'ยังไม่จัดห้อง')}</Td>
                      <Td mut>
                        {formatStoredDayMonth(a.checkIn)} → {formatStoredDayMonth(a.checkOut)}
                      </Td>
                      <Td num>{a.nights ?? '—'}</Td>
                      <Td num>{guestCount(a.adults, a.children)}</Td>
                      <Td mut>{a.status}</Td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </RosterSection>

          {/* In-house */}
          <StayRosterSection
            id="roster-inhouse"
            icon={Moon}
            title="พักอยู่ตอนนี้"
            titleEn="In-house"
            rows={inHouse}
            dateLabel={dateLabel}
            branchLabel={branchLabel}
            showBranchTag={showBranchTag}
            empty="ยังไม่มีผู้เข้าพัก · No one in-house"
          />

          {/* Departures */}
          <StayRosterSection
            id="roster-departures"
            icon={LogOut}
            title="ออกวันนี้"
            titleEn="Departures"
            rows={departures}
            dateLabel={dateLabel}
            branchLabel={branchLabel}
            showBranchTag={showBranchTag}
            empty="ไม่มีการเช็คเอาท์วันนี้ · No departures"
          />

          {/* Continuing */}
          <StayRosterSection
            id="roster-continuing"
            icon={CalendarClock}
            title="พักต่อ (ค้างคืน)"
            titleEn="Continuing stay"
            rows={continuing}
            dateLabel={dateLabel}
            branchLabel={branchLabel}
            showBranchTag={showBranchTag}
            empty="ไม่มีแขกพักต่อ · No stayovers"
          />
        </>
      )}
    </div>
  )
}

function StatCard({
  label,
  sub,
  value,
  dot,
  accent,
}: {
  label: string
  sub: string
  value: number
  dot?: string
  accent?: boolean
}) {
  return (
    <div className="v2-card p-4">
      <div className="v2-eyebrow mb-1.5 flex items-center gap-1.5">
        {dot && <span className={`v2-dot ${dot}`} />}
        {label}
      </div>
      <div
        className="v2-num text-[24px] font-bold leading-none"
        style={{ color: accent ? 'var(--v2-wine-600)' : 'var(--v2-ink)' }}
      >
        {value}
      </div>
      <div className="text-[11px] mt-1" style={{ color: 'var(--v2-ink-3)' }}>{sub}</div>
    </div>
  )
}

/** A single roster: on-screen header (with A4 print button, never printed) +
 *  the `.v2-print` region holding a print-only header and the table. */
function RosterSection({
  id,
  icon: Icon,
  title,
  titleEn,
  count,
  dateLabel,
  branchLabel,
  children,
}: {
  id: string
  icon: LucideIcon
  title: string
  titleEn: string
  count: number
  dateLabel: string
  branchLabel: string
  children: ReactNode
}) {
  return (
    <section className="space-y-2">
      <div className="flex items-center justify-between gap-3 v2-no-print">
        <div className="v2-eyebrow flex items-center gap-1.5">
          <Icon size={14} /> {title} · {titleEn}
          <span className="v2-num text-[11px] font-bold px-1.5 py-0.5 rounded-full s-mut">{count}</span>
        </div>
        <button
          onClick={() => printRegion(document.getElementById(id))}
          className="v2-btn v2-btn-ghost v2-btn-sm v2-no-print"
          aria-label={`พิมพ์ ${title}`}
        >
          <Printer size={14} /> พิมพ์ A4
        </button>
      </div>
      <div id={id} className="v2-print v2-card overflow-x-auto">
        {/* Paper-only header: title + date + branch + count. */}
        <div className="v2-print-only" style={{ padding: '0 0 8px' }}>
          <div style={{ fontSize: '15px', fontWeight: 700 }}>{title} · {titleEn}</div>
          <div style={{ fontSize: '11px', color: '#555' }}>
            {dateLabel} · {branchLabel} · {count} รายการ
          </div>
        </div>
        {children}
      </div>
    </section>
  )
}

/** Shared section for the three check-in-anchored rosters (same columns). */
function StayRosterSection({
  id,
  icon,
  title,
  titleEn,
  rows,
  dateLabel,
  branchLabel,
  showBranchTag,
  empty,
}: {
  id: string
  icon: LucideIcon
  title: string
  titleEn: string
  rows: StayRow[]
  dateLabel: string
  branchLabel: string
  showBranchTag: boolean
  empty: string
}) {
  return (
    <RosterSection
      id={id}
      icon={icon}
      title={title}
      titleEn={titleEn}
      count={rows.length}
      dateLabel={dateLabel}
      branchLabel={branchLabel}
    >
      {rows.length === 0 ? (
        <EmptyRow text={empty} />
      ) : (
        <table className="w-full text-[13px] border-collapse">
          <thead>
            <Th cols={['ห้อง', 'ชื่อแขก', 'โทร', 'ประเภท', 'เข้า', 'ออก', 'คืน', 'แขก', 'หมายเหตุ']} nums={[6, 7]} />
          </thead>
          <tbody>
            {rows.map((r) => (
              <tr key={`${r.branch}-${r.cinId}`} style={{ borderBottom: '1px solid var(--v2-line)' }}>
                <Td strong>
                  <BranchTag show={showBranchTag} branch={r.branch} />
                  {r.roomNo || r.cinNo}
                </Td>
                <Td strong>{r.customerName || 'ไม่ระบุชื่อ'}</Td>
                <Td mut>{r.phone || '—'}</Td>
                <Td mut>{r.roomTypeName || '—'}</Td>
                <Td mut>{formatStoredDateTime(r.checkInTime)}</Td>
                <Td mut>{formatStoredDayMonth(r.expectedCheckout)}</Td>
                <Td num>{r.nights ?? '—'}</Td>
                <Td num>{guestCount(r.adults, r.children)}</Td>
                <Td mut>{r.notes || '—'}</Td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </RosterSection>
  )
}

function Th({ cols, nums = [] }: { cols: string[]; nums?: number[] }) {
  return (
    <tr style={{ borderBottom: '1px solid var(--v2-line)' }}>
      {cols.map((c, i) => (
        <th
          key={c}
          className={`px-3 py-2.5 font-semibold whitespace-nowrap ${nums.includes(i) ? 'text-right' : 'text-left'}`}
          style={{ color: 'var(--v2-ink-2)' }}
        >
          {c}
        </th>
      ))}
    </tr>
  )
}

function Td({
  children,
  num,
  strong,
  mut,
}: {
  children: ReactNode
  num?: boolean
  strong?: boolean
  mut?: boolean
}) {
  return (
    <td
      className={`px-3 py-2.5 ${num ? 'v2-num text-right whitespace-nowrap' : ''} ${strong ? 'font-semibold' : ''}`}
      style={mut ? { color: 'var(--v2-ink-3)' } : undefined}
    >
      {children}
    </td>
  )
}

function BranchTag({ show, branch }: { show: boolean; branch: string }) {
  if (!show) return null
  return (
    <span
      className="mr-1.5 text-[10px] px-1.5 py-0.5 rounded align-middle"
      style={{ background: 'var(--v2-surface-2)', color: 'var(--v2-ink-3)' }}
    >
      {branch === 'hfville' ? 'Ville' : 'HF'}
    </span>
  )
}

function EmptyRow({ text }: { text: string }) {
  return (
    <div className="px-5 py-9 text-center text-[13px]" style={{ color: 'var(--v2-ink-3)' }}>
      {text}
    </div>
  )
}
