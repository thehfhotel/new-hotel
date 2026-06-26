'use client'

import { useCallback, useEffect, useRef, useState } from 'react'
import {
  Search,
  ChevronLeft,
  ChevronRight,
  X,
  Phone,
  CreditCard,
  MapPin,
  CalendarClock,
  Star,
  BedDouble,
} from 'lucide-react'
import { useBranch } from '@/contexts/BranchContext'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { formatStoredDate, formatStoredDayMonth, formatCurrency } from '@/lib/format'
import { V2Spinner, V2PageHeader } from '@/components/v2/primitives'

interface Guest {
  id: string
  name: string
  type: string
  phone: string
  idCard: string
  address: string
  lastVisit: string | null
}

interface GuestStats {
  totalBookings: number
  totalStays: number
  firstVisit: string | null
  lastVisit: string | null
  favoriteRoomType: string | null
  avgStayDays: number | null
}

interface GuestBooking {
  id: number
  roomNumber: string
  roomType: string
  checkInDate: string
  checkOutDate: string
  status: string
  totalAmount: number
}

export default function V2Guests() {
  const { branch } = useBranch()
  const branchFetch = useBranchFetch()
  const [guests, setGuests] = useState<Guest[]>([])
  const [loading, setLoading] = useState(true)
  const [search, setSearch] = useState('')
  const [debounced, setDebounced] = useState('')
  const [page, setPage] = useState(1)
  const [totalPages, setTotalPages] = useState(1)
  const [selected, setSelected] = useState<Guest | null>(null)
  // Latest-wins guard: branch can flip mid-flight (hfhotel default → stored hfville).
  const reqRef = useRef(0)

  useEffect(() => {
    const t = setTimeout(() => {
      setDebounced(search)
      setPage(1)
    }, 300)
    return () => clearTimeout(t)
  }, [search])

  const fetchGuests = useCallback(async () => {
    const token = ++reqRef.current
    setLoading(true)
    try {
      const params = new URLSearchParams({ page: String(page), limit: '20' })
      if (debounced.trim()) params.append('search', debounced.trim())
      const res = await branchFetch(`/api/customers?${params}`)
      if (token !== reqRef.current) return // superseded by a newer branch fetch
      if (res.ok) {
        const data = await res.json()
        setGuests((data.customers || data.data || []) as Guest[])
        // /api/customers returns pagination fields at the TOP LEVEL
        // ({total, page, totalPages}), unlike /api/* which nest them under
        // `pagination`. Read top-level first so paging past page 1 works.
        setTotalPages(data.totalPages || data.pagination?.totalPages || 1)
      }
    } catch {
      /* empty state */
    } finally {
      setLoading(false)
    }
  }, [branchFetch, page, debounced])

  useEffect(() => {
    fetchGuests()
  }, [fetchGuests])

  return (
    <div className="space-y-5">
      <V2PageHeader eyebrow="ลูกค้า" title="ทะเบียนลูกค้า" />

      <div
        className="flex items-center gap-2 px-3 h-11 rounded-[12px] max-w-md"
        style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line)' }}
      >
        <Search size={17} style={{ color: 'var(--v2-ink-3)' }} />
        <input
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="ค้นหาชื่อ เบอร์โทร หรือเลขบัตร…"
          className="flex-1 bg-transparent outline-none text-[14px]"
        />
      </div>

      {loading ? (
        <V2Spinner label="กำลังโหลดลูกค้า…" />
      ) : guests.length === 0 ? (
        <div className="v2-card px-5 py-12 text-center text-[14px]" style={{ color: 'var(--v2-ink-3)' }}>
          ไม่พบลูกค้า
        </div>
      ) : (
        <div className="v2-card overflow-hidden divide-y" style={{ borderColor: 'var(--v2-line)' }}>
          {guests.map((g) => (
            <button
              key={g.id}
              onClick={() => setSelected(g)}
              className="w-full flex items-center gap-3 px-4 lg:px-5 py-3 text-left transition-colors hover:bg-[var(--v2-surface-2)]"
            >
              <div
                className="w-10 h-10 rounded-full grid place-items-center text-[13px] font-bold v2-num shrink-0"
                style={{ background: 'var(--v2-wine-50)', color: 'var(--v2-wine-700)' }}
              >
                {(g.name || '?').slice(0, 2).toUpperCase()}
              </div>
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  <span className="text-[14.5px] font-semibold truncate">{g.name || 'ไม่ระบุชื่อ'}</span>
                  {g.type && g.type !== 'Regular' && (
                    <span className="v2-pill s-arr text-[11px]">
                      <Star size={11} /> {g.type}
                    </span>
                  )}
                </div>
                <div className="text-[12.5px]" style={{ color: 'var(--v2-ink-3)' }}>
                  {g.phone || 'ไม่มีเบอร์โทร'}
                </div>
              </div>
              {/* The list endpoint doesn't return last-visit, so we don't claim
                  "never stayed" (it would be wrong for returning guests). Show
                  it only when present; the profile panel has full history. */}
              <div className="hidden sm:block text-right text-[12px]" style={{ color: 'var(--v2-ink-3)' }}>
                {g.lastVisit ? `ล่าสุด ${formatStoredDate(g.lastVisit)}` : '—'}
              </div>
              <ChevronRight size={16} style={{ color: 'var(--v2-ink-3)' }} />
            </button>
          ))}
        </div>
      )}

      {totalPages > 1 && (
        <div className="flex items-center justify-center gap-2">
          <button className="v2-btn v2-btn-ghost v2-btn-sm" disabled={page <= 1} onClick={() => setPage((p) => p - 1)}>
            <ChevronLeft size={16} /> ก่อนหน้า
          </button>
          <span className="v2-num text-[13px]" style={{ color: 'var(--v2-ink-3)' }}>{page} / {totalPages}</span>
          <button className="v2-btn v2-btn-ghost v2-btn-sm" disabled={page >= totalPages} onClick={() => setPage((p) => p + 1)}>
            ถัดไป <ChevronRight size={16} />
          </button>
        </div>
      )}

      {selected && <GuestPanel guest={selected} onClose={() => setSelected(null)} branchFetch={branchFetch} />}
    </div>
  )
}

function GuestPanel({
  guest,
  onClose,
  branchFetch,
}: {
  guest: Guest
  onClose: () => void
  branchFetch: (input: string, init?: RequestInit) => Promise<Response>
}) {
  const [stats, setStats] = useState<GuestStats | null>(null)
  const [history, setHistory] = useState<GuestBooking[]>([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    let cancelled = false
    const load = async () => {
      setLoading(true)
      try {
        const [statsRes, bookRes] = await Promise.all([
          branchFetch(`/api/customers/${guest.id}/stats`).catch(() => null),
          branchFetch(`/api/customers/${guest.id}/bookings`).catch(() => null),
        ])
        if (cancelled) return
        if (statsRes && statsRes.ok) {
          const d = await statsRes.json()
          setStats(d.stats || null)
        }
        if (bookRes && bookRes.ok) {
          const d = await bookRes.json()
          setHistory((d.bookings || d.data || []) as GuestBooking[])
        }
      } finally {
        if (!cancelled) setLoading(false)
      }
    }
    load()
    return () => {
      cancelled = true
    }
  }, [guest.id, branchFetch])

  return (
    <div className="v2-sheet-backdrop" onClick={onClose} role="dialog" aria-modal="true">
      <div
        className="mt-auto lg:mt-0 lg:ml-auto w-full lg:w-[460px] lg:h-full v2-sheet-up lg:[animation:v2-panel-in_.24s_cubic-bezier(.2,.8,.2,1)] flex flex-col"
        style={{ background: 'var(--v2-surface)' }}
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-start gap-3 px-5 py-4" style={{ borderBottom: '1px solid var(--v2-line)' }}>
          <div
            className="w-12 h-12 rounded-full grid place-items-center text-[15px] font-bold v2-num shrink-0"
            style={{ background: 'var(--v2-wine-50)', color: 'var(--v2-wine-700)' }}
          >
            {(guest.name || '?').slice(0, 2).toUpperCase()}
          </div>
          <div className="flex-1 min-w-0">
            <div className="text-[18px] font-semibold leading-tight">{guest.name || 'ไม่ระบุชื่อ'}</div>
            {guest.type && <div className="text-[12.5px] mt-0.5" style={{ color: 'var(--v2-ink-3)' }}>{guest.type}</div>}
          </div>
          <button onClick={onClose} className="p-2 rounded-full" style={{ color: 'var(--v2-ink-3)' }} aria-label="ปิด">
            <X size={20} />
          </button>
        </div>

        <div className="px-5 py-4 space-y-5 overflow-y-auto">
          {/* Contact */}
          <div className="space-y-2">
            <InfoRow icon={<Phone size={15} />} value={guest.phone || '-'} />
            <InfoRow icon={<CreditCard size={15} />} value={guest.idCard || '-'} mono />
            <InfoRow icon={<MapPin size={15} />} value={guest.address || '-'} />
          </div>

          {/* Stats */}
          <div className="grid grid-cols-2 gap-2.5">
            <StatBox label="เข้าพักทั้งหมด" value={loading ? '…' : String(stats?.totalStays ?? 0)} icon={<BedDouble size={14} />} />
            <StatBox label="การจอง" value={loading ? '…' : String(stats?.totalBookings ?? 0)} icon={<CalendarClock size={14} />} />
            <StatBox label="ห้องที่ชอบ" value={loading ? '…' : stats?.favoriteRoomType || '-'} icon={<Star size={14} />} />
            <StatBox
              label="พักเฉลี่ย"
              value={loading ? '…' : stats?.avgStayDays != null ? `${stats.avgStayDays} คืน` : '-'}
              icon={<CalendarClock size={14} />}
            />
          </div>

          {/* History */}
          <div>
            <div className="v2-eyebrow mb-2">ประวัติการเข้าพัก</div>
            {loading ? (
              <div className="text-[13px]" style={{ color: 'var(--v2-ink-3)' }}>กำลังโหลด…</div>
            ) : history.length === 0 ? (
              <div className="text-[13px]" style={{ color: 'var(--v2-ink-3)' }}>ยังไม่มีประวัติ</div>
            ) : (
              <div className="v2-inset divide-y" style={{ borderColor: 'var(--v2-line)' }}>
                {history.slice(0, 12).map((h) => (
                  <div key={h.id} className="flex items-center gap-3 px-3 py-2.5" style={{ borderColor: 'var(--v2-line)' }}>
                    <div className="flex-1 min-w-0">
                      <div className="text-[13.5px] font-medium">
                        ห้อง {h.roomNumber} <span className="font-normal" style={{ color: 'var(--v2-ink-3)' }}>· {h.roomType}</span>
                      </div>
                      <div className="text-[12px]" style={{ color: 'var(--v2-ink-3)' }}>
                        {formatStoredDayMonth(h.checkInDate)} – {formatStoredDayMonth(h.checkOutDate)}
                      </div>
                    </div>
                    <span className="v2-num text-[13px] font-semibold">{formatCurrency(h.totalAmount)}</span>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  )
}

function InfoRow({ icon, value, mono }: { icon: React.ReactNode; value: string; mono?: boolean }) {
  return (
    <div className="flex items-center gap-3 text-[14px]">
      <span style={{ color: 'var(--v2-ink-3)' }}>{icon}</span>
      <span className={mono ? 'v2-num' : ''} style={{ color: 'var(--v2-ink)' }}>{value}</span>
    </div>
  )
}

function StatBox({ label, value, icon }: { label: string; value: string; icon: React.ReactNode }) {
  return (
    <div className="v2-inset px-3 py-2.5">
      <div className="flex items-center gap-1.5 text-[11px]" style={{ color: 'var(--v2-ink-3)' }}>
        {icon} {label}
      </div>
      <div className="v2-num text-[18px] font-bold mt-1 truncate">{value}</div>
    </div>
  )
}
