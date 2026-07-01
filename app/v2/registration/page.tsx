'use client'

import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import Link from 'next/link'
import { Search, Printer, ClipboardList } from 'lucide-react'
import { useBranch, BRANCH_LABELS } from '@/contexts/BranchContext'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useLiveRefresh } from '@/lib/v2/use-live-refresh'
import { formatStoredDayMonth } from '@/lib/format'
import { checkinStatusView } from '@/lib/v2/status'
import { V2PageHeader, V2Spinner, StatusPill } from '@/components/v2/primitives'

/**
 * Registration REVIEW hub (ใบลงทะเบียน) — a searchable index of every check-in
 * so reception can find & reprint any registration slip. Each row deep-links to
 * the existing reprint page (`/v2/registration/{id}`, the #29 reprint path).
 *
 * Reuses the SAME canonical source as the v2 dashboard's check-in list:
 * branch-aware `GET /api/checkins?limit=…` → `{ data: NewCheckIn[] }` (camelCase:
 * id, cinNo, bookingNo, customerName, roomNo, roomTypeName, checkInTime,
 * expectedCheckout, status). The `id` is the check-in id the reprint page keys
 * on. No new endpoint invented.
 */

interface CheckInRow {
  id: number
  cinNo: string
  bookingNo: string | null
  customerName: string | null
  roomNo: string | null
  roomTypeName: string | null
  checkInTime: string | null
  checkOutTime: string | null
  expectedCheckout: string | null
  status: string
}

export default function V2RegistrationHub() {
  const { branch } = useBranch()
  const branchFetch = useBranchFetch()
  const [rows, setRows] = useState<CheckInRow[]>([])
  const [loading, setLoading] = useState(true)
  const [search, setSearch] = useState('')
  // Latest-wins guard: branch can flip mid-flight (hfhotel default → stored hfville).
  const reqRef = useRef(0)

  const fetchCheckins = useCallback(async () => {
    const token = ++reqRef.current
    setLoading(true)
    try {
      const res = await branchFetch('/api/checkins?limit=200')
      if (token !== reqRef.current) return // superseded by a newer branch fetch
      if (res.ok) {
        const data = await res.json()
        setRows((data.data || []) as CheckInRow[])
      } else {
        setRows([])
      }
    } catch {
      if (token === reqRef.current) setRows([])
    } finally {
      if (token === reqRef.current) setLoading(false)
    }
  }, [branchFetch])

  useEffect(() => {
    fetchCheckins()
  }, [fetchCheckins])

  // Live-refresh when a check-in changes in iHOTEL or the other app.
  useLiveRefresh(
    branch,
    ['CheckInCreated', 'CheckOutCompleted', 'CheckInCancelled'],
    fetchCheckins,
  )

  const filtered = useMemo(() => {
    const q = search.trim().toLowerCase()
    if (!q) return rows
    return rows.filter((r) => {
      const hay = [r.customerName, r.roomNo, r.cinNo, r.bookingNo]
        .filter(Boolean)
        .join(' ')
        .toLowerCase()
      return hay.includes(q)
    })
  }, [rows, search])

  const branchLabel = branch === 'all' ? BRANCH_LABELS.all : BRANCH_LABELS[branch]

  return (
    <div className="space-y-5">
      <V2PageHeader eyebrow="เอกสารหน้าบ้าน · Registration" title="ใบลงทะเบียน" />

      {/* Search */}
      <div
        className="flex items-center gap-2 px-3 h-11 rounded-[12px] max-w-md"
        style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line)' }}
      >
        <Search size={17} style={{ color: 'var(--v2-ink-3)' }} />
        <input
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="ค้นหาชื่อแขก ห้อง หรือเลขที่…"
          className="flex-1 bg-transparent outline-none text-[14px]"
        />
      </div>

      {loading ? (
        <V2Spinner label="กำลังโหลดใบลงทะเบียน…" />
      ) : filtered.length === 0 ? (
        <div className="v2-card px-5 py-12 text-center" style={{ color: 'var(--v2-ink-3)' }}>
          <ClipboardList size={26} className="mx-auto mb-2 opacity-60" />
          <p className="text-[14px] font-medium" style={{ color: 'var(--v2-ink-2)' }}>
            {search.trim() ? 'ไม่พบใบลงทะเบียนที่ค้นหา' : 'ยังไม่มีใบลงทะเบียน'}
          </p>
          <p className="text-[12.5px] mt-1">
            {search.trim() ? `สำหรับ "${search.trim()}"` : `${branchLabel} · ไม่มีการเช็คอิน`}
          </p>
        </div>
      ) : (
        <div className="v2-card overflow-hidden divide-y" style={{ borderColor: 'var(--v2-line)' }}>
          {filtered.map((r) => (
            <Link
              key={r.id}
              href={`/v2/registration/${r.id}`}
              className="w-full flex items-center gap-3 px-4 lg:px-5 py-3 text-left transition-colors hover:bg-[var(--v2-surface-2)]"
            >
              {/* Room chip */}
              <div
                className="w-11 h-11 rounded-[10px] grid place-items-center text-[13px] font-bold v2-num shrink-0"
                style={{ background: 'var(--v2-wine-50)', color: 'var(--v2-wine-700)' }}
              >
                {r.roomNo || '—'}
              </div>

              {/* Name + meta */}
              <div className="flex-1 min-w-0">
                <div className="font-semibold text-[14.5px] truncate">
                  {r.customerName || 'ไม่ระบุชื่อ'}
                </div>
                <div className="text-[12px] mt-0.5 flex flex-wrap items-center gap-x-2 gap-y-0.5" style={{ color: 'var(--v2-ink-3)' }}>
                  <span className="v2-num">เลขที่ {r.cinNo}</span>
                  {r.bookingNo && <span className="v2-num">· จอง {r.bookingNo}</span>}
                  {r.roomTypeName && <span>· {r.roomTypeName}</span>}
                  {r.checkInTime && (
                    <span>· เข้า {formatStoredDayMonth(r.checkInTime)}</span>
                  )}
                </div>
              </div>

              {/* Status + reprint affordance */}
              <div className="flex items-center gap-3 shrink-0">
                <span className="hidden sm:inline-flex">
                  <StatusPill view={checkinStatusView(r.status)} />
                </span>
                <span
                  className="inline-flex items-center gap-1 text-[12.5px] font-medium"
                  style={{ color: 'var(--v2-wine-600)' }}
                  aria-label="พิมพ์ใบลงทะเบียน"
                >
                  <Printer size={15} />
                  <span className="hidden md:inline">พิมพ์</span>
                </span>
              </div>
            </Link>
          ))}
        </div>
      )}
    </div>
  )
}
