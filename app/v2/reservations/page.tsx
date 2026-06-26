'use client'

import { useCallback, useEffect, useRef, useState } from 'react'
import { CalendarPlus, Search, ChevronLeft, ChevronRight, Moon, BedDouble } from 'lucide-react'
import { useBranch } from '@/contexts/BranchContext'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { formatStoredDayMonth } from '@/lib/format'
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
  const { branch } = useBranch()
  const branchFetch = useBranchFetch()
  const [bookings, setBookings] = useState<Booking[]>([])
  const [loading, setLoading] = useState(true)
  const [status, setStatus] = useState('')
  const [search, setSearch] = useState('')
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
      if (search.trim()) params.set('search', search.trim())
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
  }, [branchFetch, page, status, search])

  useEffect(() => {
    fetchBookings()
  }, [fetchBookings])

  // Open the create form when arriving via "จองใหม่" (?new=1) or ⌘K command.
  useEffect(() => {
    if (typeof window !== 'undefined' && new URLSearchParams(window.location.search).get('new') === '1') {
      setMode('create')
      setEditing(null)
      setShowForm(true)
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
          branch === 'hfhotel' ? (
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

      {/* Status chips */}
      <div className="flex gap-2 overflow-x-auto pb-1 -mx-1 px-1">
        {STATUS_FILTERS.map((f) => (
          <button
            key={f.value || 'all'}
            className="v2-chip"
            data-active={status === f.value}
            onClick={() => {
              setPage(1)
              setStatus(f.value)
            }}
          >
            {f.label}
          </button>
        ))}
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
              onClick={() => { if (branch === 'hfhotel') openEdit(b) }}
              className="w-full flex items-center gap-4 px-4 lg:px-5 py-3.5 text-left transition-colors hover:bg-[var(--v2-surface-2)]"
              style={{ borderColor: 'var(--v2-line)' }}
            >
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  <span className="text-[15px] font-semibold truncate">{b.customerName || 'ไม่ระบุชื่อ'}</span>
                  <StatusPill view={bookingStatusView(b.status)} />
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
