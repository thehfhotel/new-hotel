'use client'

import { useState, useEffect, useCallback } from 'react'
import Link from 'next/link'
import {
  X,
  Calendar,
  User,
  Phone,
  CreditCard,
  MapPin,
  BedDouble,
  FileText,
  Plus,
  Trash2,
  Loader2,
  AlertCircle,
  ExternalLink,
} from 'lucide-react'

interface Room {
  roomNo: string
  roomType: string
  total: number
}

interface Note {
  id: number
  text: string
  createdAt: string
  updatedAt: string
}

interface Customer {
  id: string
  fullName?: string
  firstName?: string
  lastName?: string
  phone?: string
  idCard?: string
  address?: string
  type?: string
}

interface BookingDetail {
  bookNo: string
  /** Legacy MSSQL identifier (R\d{6}). Populated by writeback worker. */
  legacyBookId?: string
  bookDate: string
  checkIn: string
  checkOut: string
  status: string
  statusCode: string | number
  customer: Customer
  rooms: Room[]
  roomCount: number
  totalAmount: number
  notes: Note[]
}

interface BookingDetailDrawerProps {
  bookNo: string | null
  onClose: () => void
  inline?: boolean
}

const statusColors: Record<string, string> = {
  'จอง': 'bg-yellow-500/10 text-yellow-400',
  'เข้าพัก': 'bg-green-500/10 text-green-400',
  'เสร็จสิ้น': 'bg-gray-100 text-gray-500',
  'ยกเลิก': 'bg-red-500/10 text-red-600',
}

function formatDate(dateStr: string): string {
  if (!dateStr) return '-'
  const date = new Date(dateStr)
  return date.toLocaleDateString('th-TH', {
    day: '2-digit',
    month: '2-digit',
    year: '2-digit',
    timeZone: 'UTC',
  })
}

function formatDateTime(dateStr: string): string {
  if (!dateStr) return '-'
  const date = new Date(dateStr)
  return date.toLocaleString('th-TH', {
    day: '2-digit',
    month: '2-digit',
    year: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    timeZone: 'UTC',
  })
}

function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('th-TH', {
    style: 'currency',
    currency: 'THB',
    minimumFractionDigits: 0,
  }).format(amount)
}

export default function BookingDetailDrawer({ bookNo, onClose, inline = false }: BookingDetailDrawerProps) {
  const [booking, setBooking] = useState<BookingDetail | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [newNote, setNewNote] = useState('')
  const [addingNote, setAddingNote] = useState(false)
  const [deletingNoteId, setDeletingNoteId] = useState<number | null>(null)
  const [noteError, setNoteError] = useState<string | null>(null)

  const fetchBooking = useCallback(async () => {
    if (!bookNo) return

    setLoading(true)
    setError(null)

    try {
      const res = await fetch(`/api/bookings/${encodeURIComponent(bookNo)}`)
      const data = await res.json()

      if (!data.success) {
        throw new Error(data.error || 'Failed to fetch booking')
      }

      setBooking(data.booking)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch booking')
    } finally {
      setLoading(false)
    }
  }, [bookNo])

  useEffect(() => {
    if (bookNo) {
      fetchBooking()
    }
  }, [bookNo, fetchBooking])

  async function handleAddNote() {
    if (!bookNo || !newNote.trim()) return

    setAddingNote(true)
    setNoteError(null)

    try {
      const res = await fetch(`/api/bookings/${encodeURIComponent(bookNo)}/notes`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text: newNote.trim() }),
      })

      const data = await res.json()

      if (!data.success) {
        throw new Error(data.error || 'Failed to add note')
      }

      // Add the new note to the list
      setBooking(prev => prev ? {
        ...prev,
        notes: [data.note, ...prev.notes],
      } : null)
      setNewNote('')
    } catch (err) {
      setNoteError(err instanceof Error ? err.message : 'Failed to add note')
    } finally {
      setAddingNote(false)
    }
  }

  async function handleDeleteNote(noteId: number) {
    if (!bookNo) return

    setDeletingNoteId(noteId)
    setNoteError(null)

    try {
      const res = await fetch(`/api/bookings/${encodeURIComponent(bookNo)}/notes?noteId=${noteId}`, {
        method: 'DELETE',
      })

      const data = await res.json()

      if (!data.success) {
        throw new Error(data.error || 'Failed to delete note')
      }

      // Remove the note from the list
      setBooking(prev => prev ? {
        ...prev,
        notes: prev.notes.filter(n => n.id !== noteId),
      } : null)
    } catch (err) {
      setNoteError(err instanceof Error ? err.message : 'Failed to delete note')
    } finally {
      setDeletingNoteId(null)
    }
  }

  if (!bookNo) return null

  // Inline mode - rendered as a static sidebar
  if (inline) {
    return (
      <div className="bg-white rounded-lg shadow-lg border border-gray-200 overflow-hidden">
        <div className="p-4 max-h-[calc(100vh-2rem)] overflow-y-auto">
          {/* Header */}
          <div className="flex items-center justify-between mb-4 pb-4 border-b border-gray-200">
            <div className="flex flex-col">
              <h2 className="text-lg font-bold text-gray-900">
                #{bookNo}
              </h2>
              {detail?.legacyBookId && (
                <span className="text-xs text-gray-500 font-mono" title="Legacy system reference">
                  Legacy: {detail.legacyBookId}
                </span>
              )}
            </div>
            <button
              onClick={onClose}
              className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
              title="ปิด"
            >
              <X size={20} />
            </button>
          </div>

          {/* Loading */}
          {loading && (
            <div className="flex items-center justify-center py-12">
              <Loader2 className="h-8 w-8 animate-spin text-red-600" />
            </div>
          )}

          {/* Error */}
          {error && (
            <div className="flex items-center justify-center py-12 text-red-600">
              <AlertCircle className="h-6 w-6 mr-2" />
              <span>{error}</span>
            </div>
          )}

          {/* Content */}
          {booking && !loading && (
            <div className="space-y-6">
              {/* Booking Info */}
              <section>
                <h3 className="flex items-center gap-2 text-sm font-semibold text-gray-500 uppercase mb-3">
                  <Calendar size={16} />
                  ข้อมูลการจอง
                </h3>
                <div className="bg-gray-100 rounded-lg p-4 space-y-3">
                  <div className="flex justify-between items-center">
                    <span className="text-gray-500">สถานะ</span>
                    <span className={`px-2.5 py-0.5 rounded-full text-xs font-medium ${statusColors[booking.status] || 'bg-gray-100 text-gray-500'}`}>
                      {booking.status}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-500">วันที่จอง</span>
                    <span className="font-medium text-gray-800">{formatDate(booking.bookDate)}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-500">เช็คอิน</span>
                    <span className="font-medium text-gray-800">{formatDate(booking.checkIn)}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-500">เช็คเอาท์</span>
                    <span className="font-medium text-gray-800">{formatDate(booking.checkOut)}</span>
                  </div>
                </div>
              </section>

              {/* Rooms */}
              <section>
                <h3 className="flex items-center gap-2 text-sm font-semibold text-gray-500 uppercase mb-3">
                  <BedDouble size={16} />
                  ห้องพัก ({booking.roomCount})
                </h3>
                <div className="space-y-2">
                  {booking.rooms.map((room, idx) => (
                    <div key={idx} className="bg-gray-100 rounded-lg p-3 flex justify-between items-center">
                      <div>
                        <span className="font-medium text-gray-800">ห้อง {room.roomNo}</span>
                        <span className="text-gray-500 ml-2">- {room.roomType}</span>
                      </div>
                      <span className="font-medium text-red-600">
                        {formatCurrency(room.total)}
                      </span>
                    </div>
                  ))}
                  <div className="flex justify-between items-center pt-2 border-t border-gray-200 mt-2">
                    <span className="font-semibold text-gray-800">รวมทั้งหมด</span>
                    <span className="font-bold text-lg text-red-600">
                      {formatCurrency(booking.totalAmount)}
                    </span>
                  </div>
                </div>
              </section>

              {/* Customer */}
              <section>
                <h3 className="flex items-center gap-2 text-sm font-semibold text-gray-500 uppercase mb-3">
                  <User size={16} />
                  ข้อมูลลูกค้า
                </h3>
                <div className="bg-gray-100 rounded-lg p-4 space-y-3">
                  <div className="font-medium text-lg text-gray-800">
                    {booking.customer.fullName || `${booking.customer.firstName || ''} ${booking.customer.lastName || ''}`.trim() || '-'}
                  </div>
                  {booking.customer.phone && (
                    <div className="flex items-center gap-2 text-gray-500">
                      <Phone size={16} />
                      <span>{booking.customer.phone}</span>
                    </div>
                  )}
                  {booking.customer.idCard && (
                    <div className="flex items-center gap-2 text-gray-500">
                      <CreditCard size={16} />
                      <span>{booking.customer.idCard}</span>
                    </div>
                  )}
                  {booking.customer.address && (
                    <div className="flex items-center gap-2 text-gray-500">
                      <MapPin size={16} />
                      <span className="text-sm">{booking.customer.address}</span>
                    </div>
                  )}
                  {booking.customer.id && (
                    <Link
                      href={`/customers?search=${encodeURIComponent(booking.customer.id)}`}
                      className="inline-flex items-center gap-1 text-red-600 hover:text-red-300 text-sm mt-2"
                    >
                      ดูข้อมูลลูกค้าเพิ่มเติม
                      <ExternalLink size={14} />
                    </Link>
                  )}
                </div>
              </section>

              {/* Notes */}
              <section>
                <h3 className="flex items-center gap-2 text-sm font-semibold text-gray-500 uppercase mb-3">
                  <FileText size={16} />
                  บันทึก ({booking.notes.length})
                </h3>

                {noteError && (
                  <div className="mb-3 p-2 bg-red-50 border border-red-200 rounded-lg text-red-600 text-sm">
                    {noteError}
                  </div>
                )}

                {/* Add Note Form */}
                <div className="mb-4">
                  <textarea
                    value={newNote}
                    onChange={(e) => setNewNote(e.target.value)}
                    placeholder="เพิ่มบันทึก..."
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 text-sm resize-none"
                    rows={2}
                  />
                  <button
                    onClick={handleAddNote}
                    disabled={!newNote.trim() || addingNote}
                    className="mt-2 w-full flex items-center justify-center gap-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                  >
                    {addingNote ? (
                      <Loader2 className="h-4 w-4 animate-spin" />
                    ) : (
                      <Plus size={16} />
                    )}
                    เพิ่มบันทึก
                  </button>
                </div>

                {/* Notes List */}
                <div className="space-y-2">
                  {booking.notes.length === 0 ? (
                    <p className="text-gray-500 text-sm text-center py-4">
                      ยังไม่มีบันทึก
                    </p>
                  ) : (
                    booking.notes.map((note) => (
                      <div key={note.id} className="bg-gray-100 rounded-lg p-3">
                        <div className="flex justify-between items-start gap-2">
                          <p className="text-sm text-gray-700 flex-1 whitespace-pre-wrap">
                            {note.text}
                          </p>
                          <button
                            onClick={() => handleDeleteNote(note.id)}
                            disabled={deletingNoteId === note.id}
                            className="p-2 text-gray-500 hover:text-red-600 transition-colors"
                            aria-label="ลบบันทึก"
                          >
                            {deletingNoteId === note.id ? (
                              <Loader2 className="h-4 w-4 animate-spin" />
                            ) : (
                              <Trash2 size={16} />
                            )}
                          </button>
                        </div>
                        <p className="text-xs text-gray-500 mt-2">
                          {formatDateTime(note.createdAt)}
                        </p>
                      </div>
                    ))
                  )}
                </div>
              </section>
            </div>
          )}
        </div>
      </div>
    )
  }

  // Modal mode - rendered as overlay drawer
  return (
    <>
      {/* Backdrop - click to dismiss */}
      <div
        className="fixed inset-0 bg-black/20 z-40"
        onClick={onClose}
      />

      {/* Drawer */}
      <div
        role="dialog"
        aria-modal="true"
        tabIndex={-1}
        onKeyDown={(e) => { if (e.key === 'Escape') onClose() }}
        className="fixed right-0 top-0 h-full w-full sm:w-96 bg-white shadow-xl border-l border-gray-200 overflow-y-auto z-50"
      >
        <div className="p-4">
          {/* Header */}
          <div className="flex items-center justify-between mb-4 pb-4 border-b border-gray-200">
            <div className="flex flex-col">
              <h2 className="text-lg font-bold text-gray-900">
                Booking #{bookNo}
              </h2>
              {detail?.legacyBookId && (
                <span className="text-xs text-gray-500 font-mono" title="Legacy system reference">
                  Legacy: {detail.legacyBookId}
                </span>
              )}
            </div>
            <button
              onClick={onClose}
              className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
            >
              <X size={20} />
            </button>
          </div>

          {/* Loading */}
          {loading && (
            <div className="flex items-center justify-center py-12">
              <Loader2 className="h-8 w-8 animate-spin text-blue-600" />
            </div>
          )}

          {/* Error */}
          {error && (
            <div className="flex items-center justify-center py-12 text-red-600">
              <AlertCircle className="h-6 w-6 mr-2" />
              <span>{error}</span>
            </div>
          )}

          {/* Content */}
          {booking && !loading && (
            <div className="space-y-6">
              {/* Booking Info */}
              <section>
                <h3 className="flex items-center gap-2 text-sm font-semibold text-gray-500 uppercase mb-3">
                  <Calendar size={16} />
                  ข้อมูลการจอง
                </h3>
                <div className="bg-gray-100 rounded-lg p-4 space-y-3">
                  <div className="flex justify-between items-center">
                    <span className="text-gray-500">สถานะ</span>
                    <span className={`px-2.5 py-0.5 rounded-full text-xs font-medium ${statusColors[booking.status] || 'bg-gray-100 text-gray-500'}`}>
                      {booking.status}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-500">วันที่จอง</span>
                    <span className="font-medium text-gray-800">{formatDate(booking.bookDate)}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-500">เช็คอิน</span>
                    <span className="font-medium text-gray-800">{formatDate(booking.checkIn)}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-500">เช็คเอาท์</span>
                    <span className="font-medium text-gray-800">{formatDate(booking.checkOut)}</span>
                  </div>
                </div>
              </section>

              {/* Rooms */}
              <section>
                <h3 className="flex items-center gap-2 text-sm font-semibold text-gray-500 uppercase mb-3">
                  <BedDouble size={16} />
                  ห้องพัก ({booking.roomCount})
                </h3>
                <div className="space-y-2">
                  {booking.rooms.map((room, idx) => (
                    <div key={idx} className="bg-gray-100 rounded-lg p-3 flex justify-between items-center">
                      <div>
                        <span className="font-medium text-gray-800">ห้อง {room.roomNo}</span>
                        <span className="text-gray-500 ml-2">- {room.roomType}</span>
                      </div>
                      <span className="font-medium text-red-600">
                        {formatCurrency(room.total)}
                      </span>
                    </div>
                  ))}
                  <div className="flex justify-between items-center pt-2 border-t border-gray-200 mt-2">
                    <span className="font-semibold text-gray-800">รวมทั้งหมด</span>
                    <span className="font-bold text-lg text-red-600">
                      {formatCurrency(booking.totalAmount)}
                    </span>
                  </div>
                </div>
              </section>

              {/* Customer */}
              <section>
                <h3 className="flex items-center gap-2 text-sm font-semibold text-gray-500 uppercase mb-3">
                  <User size={16} />
                  ข้อมูลลูกค้า
                </h3>
                <div className="bg-gray-100 rounded-lg p-4 space-y-3">
                  <div className="font-medium text-lg text-gray-800">
                    {booking.customer.fullName || `${booking.customer.firstName || ''} ${booking.customer.lastName || ''}`.trim() || '-'}
                  </div>
                  {booking.customer.phone && (
                    <div className="flex items-center gap-2 text-gray-500">
                      <Phone size={16} />
                      <span>{booking.customer.phone}</span>
                    </div>
                  )}
                  {booking.customer.idCard && (
                    <div className="flex items-center gap-2 text-gray-500">
                      <CreditCard size={16} />
                      <span>{booking.customer.idCard}</span>
                    </div>
                  )}
                  {booking.customer.address && (
                    <div className="flex items-center gap-2 text-gray-500">
                      <MapPin size={16} />
                      <span className="text-sm">{booking.customer.address}</span>
                    </div>
                  )}
                  {booking.customer.id && (
                    <Link
                      href={`/customers?search=${encodeURIComponent(booking.customer.id)}`}
                      className="inline-flex items-center gap-1 text-red-600 hover:text-red-300 text-sm mt-2"
                    >
                      ดูข้อมูลลูกค้าเพิ่มเติม
                      <ExternalLink size={14} />
                    </Link>
                  )}
                </div>
              </section>

              {/* Notes */}
              <section>
                <h3 className="flex items-center gap-2 text-sm font-semibold text-gray-500 uppercase mb-3">
                  <FileText size={16} />
                  บันทึก ({booking.notes.length})
                </h3>

                {noteError && (
                  <div className="mb-3 p-2 bg-red-50 border border-red-200 rounded-lg text-red-600 text-sm">
                    {noteError}
                  </div>
                )}

                {/* Add Note Form */}
                <div className="mb-4">
                  <textarea
                    value={newNote}
                    onChange={(e) => setNewNote(e.target.value)}
                    placeholder="เพิ่มบันทึก..."
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 text-sm resize-none"
                    rows={2}
                  />
                  <button
                    onClick={handleAddNote}
                    disabled={!newNote.trim() || addingNote}
                    className="mt-2 w-full flex items-center justify-center gap-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                  >
                    {addingNote ? (
                      <Loader2 className="h-4 w-4 animate-spin" />
                    ) : (
                      <Plus size={16} />
                    )}
                    เพิ่มบันทึก
                  </button>
                </div>

                {/* Notes List */}
                <div className="space-y-2">
                  {booking.notes.length === 0 ? (
                    <p className="text-gray-500 text-sm text-center py-4">
                      ยังไม่มีบันทึก
                    </p>
                  ) : (
                    booking.notes.map((note) => (
                      <div key={note.id} className="bg-gray-100 rounded-lg p-3">
                        <div className="flex justify-between items-start gap-2">
                          <p className="text-sm text-gray-700 flex-1 whitespace-pre-wrap">
                            {note.text}
                          </p>
                          <button
                            onClick={() => handleDeleteNote(note.id)}
                            disabled={deletingNoteId === note.id}
                            className="p-2 text-gray-500 hover:text-red-600 transition-colors"
                            aria-label="ลบบันทึก"
                          >
                            {deletingNoteId === note.id ? (
                              <Loader2 className="h-4 w-4 animate-spin" />
                            ) : (
                              <Trash2 size={16} />
                            )}
                          </button>
                        </div>
                        <p className="text-xs text-gray-500 mt-2">
                          {formatDateTime(note.createdAt)}
                        </p>
                      </div>
                    ))
                  )}
                </div>
              </section>
            </div>
          )}
        </div>
      </div>
    </>
  )
}
