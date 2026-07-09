'use client'

import { useCallback, useEffect, useRef, useState } from 'react'
import { CalendarPlus, Search, ChevronLeft, ChevronRight, Moon, BedDouble, Wallet } from 'lucide-react'
import { useBranch } from '@/contexts/BranchContext'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useLiveRefresh } from '@/lib/v2/use-live-refresh'
import { formatStoredDayMonth, formatCurrency } from '@/lib/format'
import type { Booking, BookingDetail } from '@/types/booking'
import { bookingStatusView } from '@/lib/v2/status'
import { V2Spinner, V2PageHeader, StatusPill, VilleNotice } from '@/components/v2/primitives'
import BookingForm, { type BookingFormState } from '@/components/forms/BookingForm'

const STATUS_FILTERS = [
  { value: '', label: 'ทั้งหมด' },
  { value: 'pending', label: 'รอยืนยัน' },
  { value: 'confirmed', label: 'ยืนยันแล้ว' },
  { value: 'checkedin', label: 'เช็คอินแล้ว' },
  { value: 'completed', label: 'เสร็จสิ้น' },
  { value: 'cancelled', label: 'ยกเลิก' },
]

export default function V2Reservations() {
  const { branch, canWrite } = useBranch()
  const branchFetch = useBranchFetch()
  const [bookings, setBookings] = useState<Booking[]>([])
  const [loading, setLoading] = useState(true)
  const [status, setStatus] = useState('')
  const [search, setSearch] = useState('')
  // "New OTA bookings" queue — `book_source` filter (backend NewBookingsQuery.source).
  // The OTA chip sets source='ota' + status='pending'; a plain status chip clears it.
  const [source, setSource] = useState('')
  // Always-visible count of pending OTA bookings the front desk hasn't actioned
  // yet, so they're aware of incoming OTA reservations even from another tab.
  const [otaPending, setOtaPending] = useState(0)
  // Task #53 — balance-due filter (also the notification-bell deep-link target,
  // /v2/reservations?balanceDue=1). Initial value is read from the URL below.
  const [balanceDue, setBalanceDue] = useState(false)
  const [page, setPage] = useState(1)
  const [totalPages, setTotalPages] = useState(1)

  const [showForm, setShowForm] = useState(false)
  const [mode, setMode] = useState<'create' | 'edit'>('create')
  const [editing, setEditing] = useState<BookingFormState | null>(null)
  // Latest-wins guard: branch can flip mid-flight (hfhotel default → stored hfville).
  const reqRef = useRef(0)

  const fetchBookings = useCallback(async () => {
    const token = ++reqRef.current
    setLoading(true)
    try {
      const params = new URLSearchParams({ page: String(page), limit: '20' })
      if (status) params.set('status', status)
      if (source) params.set('source', source)
      if (search.trim()) params.set('search', search.trim())
      if (balanceDue) params.set('balanceDue', 'true')
      const res = await branchFetch(`/api/bookings?${params.toString()}`)
      if (token !== reqRef.current) return // superseded by a newer branch fetch
      if (res.ok) {
        const data = await res.json()
        setBookings((data.data || []) as Booking[])
        setTotalPages(data.pagination?.totalPages || 1)
      }
    } catch {
      /* empty state */
    } finally {
      setLoading(false)
    }
  }, [branchFetch, page, status, source, search, balanceDue])

  // Cheap standalone count for the "OTA ใหม่" badge (source=ota + status=pending).
  // `limit=1` — we only read pagination.total.
  const fetchOtaPending = useCallback(async () => {
    try {
      const res = await branchFetch('/api/bookings?source=ota&status=pending&limit=1')
      if (res.ok) {
        const data = await res.json()
        setOtaPending(data.pagination?.total || 0)
      }
    } catch {
      /* ignore — badge just stays at its last value */
    }
  }, [branchFetch])

  useEffect(() => {
    fetchBookings()
  }, [fetchBookings])

  useEffect(() => {
    fetchOtaPending()
  }, [fetchOtaPending])

  // Live-refresh when a booking/check-in changes in iHOTEL or the other app —
  // refresh both the list and the OTA-pending badge so a newly-arrived OTA
  // booking shows up live without a manual reload.
  const refreshAll = useCallback(() => {
    fetchBookings()
    fetchOtaPending()
  }, [fetchBookings, fetchOtaPending])
  useLiveRefresh(
    branch,
    ['BookingCreated', 'BookingModified', 'BookingCancelled', 'CheckInCreated', 'CheckOutCompleted', 'CheckInCancelled'],
    refreshAll,
  )

  // Open the create form when arriving via "จองใหม่" (?new=1) or ⌘K command,
  // and pre-apply the balance-due filter when deep-linked from the bell.
  useEffect(() => {
    if (typeof window === 'undefined') return
    const qs = new URLSearchParams(window.location.search)
    if (qs.get('new') === '1') {
      setMode('create')
      setEditing(null)
      setShowForm(true)
    }
    if (qs.get('balanceDue') === '1' || qs.get('balanceDue') === 'true') {
      setBalanceDue(true)
    }
  }, [])

  const openCreate = () => {
    setMode('create')
    setEditing(null)
    setShowForm(true)
  }

  const openEdit = async (b: Booking) => {
    try {
      const res = await branchFetch(`/api/bookings/${b.id}`)
      if (!res.ok) return
      const data = await res.json()
      if (!data.success) return
      const d: BookingDetail = data.booking
      setEditing({
        id: d.id,
        bookNo: d.bookNo,
        customerId: d.customerId,
        customerName: d.customerName || undefined,
        checkIn: d.checkIn ? d.checkIn.split('T')[0] : '',
        checkOut: d.checkOut ? d.checkOut.split('T')[0] : '',
        adults: d.adults || 1,
        children: d.children || 0,
        status: d.status,
        source: d.source,
        depositAmount: d.depositAmount,
        notes: d.notes,
        rooms: d.rooms.map((r) => ({ roomId: r.roomId, pricePerNight: r.pricePerNight })),
      })
      setMode('edit')
      setShowForm(true)
    } catch {
      /* ignore */
    }
  }

  const handleSave = async (data: BookingFormState) => {
    const endpoint = data.id ? `/api/bookings/${data.id}` : '/api/bookings'
    const res = await branchFetch(endpoint, {
      method: data.id ? 'PUT' : 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        customerId: data.customerId,
        checkIn: data.checkIn,
        checkOut: data.checkOut,
        adults: data.adults,
        children: data.children,
        status: data.status,
        source: data.source,
        totalAmount: null,
        depositAmount: data.depositAmount,
        notes: data.notes,
        rooms: data.rooms,
      }),
    })
    const result = await res.json()
    if (!res.ok || !result.success) throw new Error(result.message || 'บันทึกการจองไม่สำเร็จ')
    fetchBookings()
  }

  const handleCancel = async (id: number) => {
    const res = await branchFetch(`/api/bookings/${id}/cancel`, { method: 'PUT' })
    const result = await res.json()
    if (!res.ok || !result.success) throw new Error(result.message || 'ยกเลิกการจองไม่สำเร็จ')
    fetchBookings()
  }

  return (
    <div className="space-y-5">
      <V2PageHeader
        eyebrow="การจอง"
        title="รายการจอง"
        right={
          canWrite ? (
            <button onClick={openCreate} className="v2-btn v2-btn-primary">
              <CalendarPlus size={17} /> จองใหม่
            </button>
          ) : undefined
        }
      />

      <VilleNotice branch={branch} />

      {/* Search */}
      <div
        className="flex items-center gap-2 px-3 h-11 rounded-[12px] max-w-md"
        style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line)' }}
      >
        <Search size={17} style={{ color: 'var(--v2-ink-3)' }} />
        <input
          value={search}
          onChange={(e) => {
            setPage(1)
            setSearch(e.target.value)
          }}
          placeholder="ค้นหาเลขที่จอง หรือชื่อลูกค้า…"
          className="flex-1 bg-transparent outline-none text-[14px]"
        />
      </div>

      {/* Status chips + "New OTA" queue + balance-due toggle */}
      <div className="flex gap-2 overflow-x-auto pb-1 -mx-1 px-1">
        {STATUS_FILTERS.map((f) => (
          <button
            key={f.value || 'all'}
            className="v2-chip"
            // A plain status chip is a single-status, ALL-source view — not
            // highlighted while the combined OTA filter is active.
            data-active={status === f.value && !source}
            onClick={() => {
              setPage(1)
              setSource('')
              setStatus(f.value)
            }}
          >
            {f.label}
          </button>
        ))}
        {/* "New OTA bookings" queue = source=ota + status=pending. Toggles the
            combined filter; the badge shows how many are awaiting the front desk. */}
        <button
          className="v2-chip whitespace-nowrap"
          data-active={source === 'ota' && status === 'pending'}
          onClick={() => {
            setPage(1)
            if (source === 'ota' && status === 'pending') {
              setSource('')
              setStatus('')
            } else {
              setSource('ota')
              setStatus('pending')
            }
          }}
        >
          OTA ใหม่{otaPending > 0 ? ` · ${otaPending}` : ''}
        </button>
        <button
          className="v2-chip whitespace-nowrap"
          data-active={balanceDue}
          onClick={() => {
            setPage(1)
            setBalanceDue((v) => !v)
          }}
        >
          <Wallet size={13} /> ค้างชำระ
        </button>
      </div>

      {/* List */}
      {loading ? (
        <V2Spinner label="กำลังโหลดการจอง…" />
      ) : bookings.length === 0 ? (
        <div className="v2-card px-5 py-12 text-center text-[14px]" style={{ color: 'var(--v2-ink-3)' }}>
          ไม่พบการจอง
        </div>
      ) : (
        <div className="v2-card overflow-hidden divide-y" style={{ borderColor: 'var(--v2-line)' }}>
          {bookings.map((b) => (
            <button
              key={b.id}
              onClick={() => { if (canWrite) openEdit(b) }}
              className="w-full flex items-center gap-4 px-4 lg:px-5 py-3.5 text-left transition-colors hover:bg-[var(--v2-surface-2)]"
              style={{ borderColor: 'var(--v2-line)' }}
            >
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2 flex-wrap">
                  <span className="text-[15px] font-semibold truncate">{b.customerName || 'ไม่ระบุชื่อ'}</span>
                  <StatusPill view={bookingStatusView(b.status)} />
                  {(() => {
                    const bal = (b.totalAmount ?? 0) - (b.depositAmount ?? 0)
                    return bal > 0 ? (
                      <span
                        className="inline-flex items-center gap-1 text-[11px] px-1.5 py-0.5 rounded"
                        style={{ background: 'var(--v2-occ-bg)', color: 'var(--v2-occ)' }}
                      >
                        <Wallet size={11} /> ค้าง {formatCurrency(bal)}
                      </span>
                    ) : null
                  })()}
                </div>
                <div className="text-[12.5px] mt-0.5 v2-num" style={{ color: 'var(--v2-ink-3)' }}>
                  {b.bookNo}
                </div>
              </div>
              <div className="hidden sm:flex items-center gap-1.5 text-[13px]" style={{ color: 'var(--v2-ink-2)' }}>
                {formatStoredDayMonth(b.checkIn)} <ChevronRight size={13} style={{ color: 'var(--v2-ink-3)' }} /> {formatStoredDayMonth(b.checkOut)}
              </div>
              <div className="flex items-center gap-3 text-[12.5px]" style={{ color: 'var(--v2-ink-3)' }}>
                {b.nights != null && (
                  <span className="inline-flex items-center gap-1"><Moon size={13} /> {b.nights}</span>
                )}
                <span className="inline-flex items-center gap-1"><BedDouble size={13} /> {b.roomCount}</span>
              </div>
            </button>
          ))}
        </div>
      )}

      {/* Pagination */}
      {totalPages > 1 && (
        <div className="flex items-center justify-center gap-2">
          <button className="v2-btn v2-btn-ghost v2-btn-sm" disabled={page <= 1} onClick={() => setPage((p) => p - 1)}>
            <ChevronLeft size={16} /> ก่อนหน้า
          </button>
          <span className="v2-num text-[13px]" style={{ color: 'var(--v2-ink-3)' }}>
            {page} / {totalPages}
          </span>
          <button className="v2-btn v2-btn-ghost v2-btn-sm" disabled={page >= totalPages} onClick={() => setPage((p) => p + 1)}>
            ถัดไป <ChevronRight size={16} />
          </button>
        </div>
      )}

      {showForm && (
        <BookingForm
          isOpen={showForm}
          mode={mode}
          initialData={editing}
          onClose={() => setShowForm(false)}
          onSave={handleSave}
          onCancel={handleCancel}
        />
      )}
    </div>
  )
}
