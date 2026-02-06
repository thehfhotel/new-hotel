'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  Search,
  Filter,
  ChevronLeft,
  ChevronRight,
  Loader2,
  AlertCircle,
  Calendar,
  RefreshCw,
  Plus,
  X,
  BedDouble,
  Edit3,
} from 'lucide-react'
import DatePicker from 'react-datepicker'
import 'react-datepicker/dist/react-datepicker.css'
import BookingForm, { BookingFormData } from '@/components/forms/BookingForm'
import { RoomOption } from '@/components/pickers/RoomPicker'

// Types
interface BookingRoom {
  id: number
  roomId: number
  roomNo: string | null
  roomTypeName: string | null
  pricePerNight: number | null
}

interface Booking {
  id: number
  bookNo: string
  customerId: number
  customerName: string | null
  checkIn: string | null
  checkOut: string | null
  nights: number | null
  adults: number | null
  children: number | null
  status: string
  source: string | null
  totalAmount: number | null
  depositAmount: number | null
  notes: string | null
  roomCount: number
  createdAt: string | null
}

interface BookingDetail extends Booking {
  rooms: BookingRoom[]
}

interface BookingsResponse {
  success: boolean
  data: Booking[]
  pagination: {
    page: number
    limit: number
    total: number
    totalPages: number
  }
}

const statusOptions = [
  { value: '', label: 'สถานะทั้งหมด' },
  { value: 'pending', label: 'รอยืนยัน' },
  { value: 'confirmed', label: 'ยืนยันแล้ว' },
  { value: 'checkedin', label: 'เช็คอินแล้ว' },
  { value: 'completed', label: 'เสร็จสิ้น' },
  { value: 'cancelled', label: 'ยกเลิก' },
  { value: 'noshow', label: 'ไม่มาตามนัด' },
]

const statusColors: Record<string, string> = {
  pending: 'bg-yellow-500/10 text-yellow-400',
  confirmed: 'bg-sky-500/10 text-sky-400',
  checkedin: 'bg-emerald-500/10 text-emerald-400',
  completed: 'bg-zinc-700 text-zinc-300',
  cancelled: 'bg-red-500/10 text-red-400',
  noshow: 'bg-violet-500/10 text-violet-400',
}

const statusLabels: Record<string, string> = {
  pending: 'รอยืนยัน',
  confirmed: 'ยืนยันแล้ว',
  checkedin: 'เช็คอินแล้ว',
  completed: 'เสร็จสิ้น',
  cancelled: 'ยกเลิก',
  noshow: 'ไม่มาตามนัด',
}

// Convert Gregorian year to Buddhist Era display
function formatDateBE(dateString: string | null): string {
  if (!dateString) return '-'
  const date = new Date(dateString)
  const year = date.getFullYear() + 543
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${day}/${month}/${year.toString().slice(-2)}`
}

// Format date for API (YYYY-MM-DD)
function formatDateForApi(date: Date): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

export default function NewBookingsPage() {
  // State
  const [bookings, setBookings] = useState<Booking[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  // Pagination
  const [currentPage, setCurrentPage] = useState(1)
  const [totalPages, setTotalPages] = useState(1)
  const [totalItems, setTotalItems] = useState(0)
  const itemsPerPage = 20

  // Filters
  const [statusFilter, setStatusFilter] = useState('')
  const [dateRange, setDateRange] = useState<[Date | null, Date | null]>([null, null])
  const [startDate, endDate] = dateRange
  const [searchTerm, setSearchTerm] = useState('')
  const [debouncedSearch, setDebouncedSearch] = useState('')

  // Form modal
  const [showForm, setShowForm] = useState(false)
  const [formMode, setFormMode] = useState<'create' | 'edit'>('create')
  const [editingBooking, setEditingBooking] = useState<BookingFormData | null>(null)
  const [loadingDetail, setLoadingDetail] = useState(false)

  // Debounce search
  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedSearch(searchTerm)
      setCurrentPage(1)
    }, 300)
    return () => clearTimeout(timer)
  }, [searchTerm])

  // Fetch bookings
  const fetchBookings = useCallback(async () => {
    setLoading(true)
    setError(null)

    try {
      const params = new URLSearchParams({
        page: currentPage.toString(),
        limit: itemsPerPage.toString(),
        sortBy: 'createdAt',
        sortOrder: 'desc',
      })

      if (statusFilter) params.append('status', statusFilter)
      if (startDate) params.append('startDate', formatDateForApi(startDate))
      if (endDate) params.append('endDate', formatDateForApi(endDate))
      if (debouncedSearch) params.append('search', debouncedSearch)

      const response = await fetch(`/api/new/bookings?${params.toString()}`)

      if (!response.ok) {
        throw new Error('ไม่สามารถดึงข้อมูลการจองได้')
      }

      const data: BookingsResponse = await response.json()

      if (!data.success) {
        throw new Error('ไม่สามารถดึงข้อมูลการจองได้')
      }

      setBookings(data.data)
      setTotalPages(data.pagination.totalPages)
      setTotalItems(data.pagination.total)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setLoading(false)
    }
  }, [currentPage, statusFilter, startDate, endDate, debouncedSearch])

  useEffect(() => {
    fetchBookings()
  }, [fetchBookings])

  // Handle filter reset
  const handleResetFilters = () => {
    setStatusFilter('')
    setDateRange([null, null])
    setSearchTerm('')
    setCurrentPage(1)
  }

  // Pagination handlers
  const goToPage = (page: number) => {
    if (page >= 1 && page <= totalPages) {
      setCurrentPage(page)
    }
  }

  // Handle add new booking
  const handleAddBooking = () => {
    setEditingBooking(null)
    setFormMode('create')
    setShowForm(true)
  }

  // Handle edit booking
  const handleEditBooking = async (booking: Booking) => {
    setLoadingDetail(true)
    try {
      const response = await fetch(`/api/new/bookings/${booking.id}`)
      if (!response.ok) {
        throw new Error('ไม่สามารถดึงข้อมูลการจองได้')
      }
      const data = await response.json()
      if (!data.success) {
        throw new Error('ไม่สามารถดึงข้อมูลการจองได้')
      }

      const detail: BookingDetail = data.booking

      setEditingBooking({
        id: detail.id,
        bookNo: detail.bookNo,
        customerId: detail.customerId,
        customerName: detail.customerName || undefined,
        checkIn: detail.checkIn ? detail.checkIn.split('T')[0] : '',
        checkOut: detail.checkOut ? detail.checkOut.split('T')[0] : '',
        adults: detail.adults || 1,
        children: detail.children || 0,
        status: detail.status,
        source: detail.source,
        depositAmount: detail.depositAmount,
        notes: detail.notes,
        rooms: detail.rooms.map((r) => ({
          roomId: r.roomId,
          pricePerNight: r.pricePerNight,
        })),
      })
      setFormMode('edit')
      setShowForm(true)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setLoadingDetail(false)
    }
  }

  // Handle save booking
  const handleSaveBooking = async (data: BookingFormData) => {
    const endpoint = data.id
      ? `/api/new/bookings/${data.id}`
      : '/api/new/bookings'
    const method = data.id ? 'PUT' : 'POST'

    const response = await fetch(endpoint, {
      method,
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        customerId: data.customerId,
        checkIn: data.checkIn,
        checkOut: data.checkOut,
        adults: data.adults,
        children: data.children,
        status: data.status,
        source: data.source,
        totalAmount: null, // Calculate on backend or later
        depositAmount: data.depositAmount,
        notes: data.notes,
        rooms: data.rooms,
      }),
    })

    const result = await response.json()

    if (!response.ok || !result.success) {
      throw new Error(result.message || 'Failed to save booking')
    }

    // Refresh the list
    fetchBookings()
  }

  // Handle cancel booking
  const handleCancelBooking = async (id: number) => {
    const response = await fetch(`/api/new/bookings/${id}/cancel`, {
      method: 'PUT',
    })

    const result = await response.json()

    if (!response.ok || !result.success) {
      throw new Error(result.message || 'Failed to cancel booking')
    }

    // Refresh the list
    fetchBookings()
  }

  // Close form
  const handleCloseForm = () => {
    setShowForm(false)
    setEditingBooking(null)
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-zinc-100">การจอง (New Mode)</h1>
          <p className="text-sm text-zinc-500 mt-1">
            {totalItems.toLocaleString()} รายการ
          </p>
        </div>
        <div className="flex items-center gap-3">
          <button
            onClick={fetchBookings}
            disabled={loading}
            className="flex items-center space-x-2 px-4 py-2 border border-zinc-700 text-zinc-300 rounded-lg hover:bg-zinc-800 disabled:opacity-50"
          >
            <RefreshCw className={`h-4 w-4 ${loading ? 'animate-spin' : ''}`} />
            <span className="hidden sm:inline">รีเฟรช</span>
          </button>
          <button
            onClick={handleAddBooking}
            className="flex items-center space-x-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700"
          >
            <Plus className="h-4 w-4" />
            <span>เพิ่มการจอง</span>
          </button>
        </div>
      </div>

      {/* Filters */}
      <div className="bg-zinc-900 rounded-lg border border-zinc-800 p-4">
        <div className="flex items-center space-x-2 mb-4">
          <Filter className="h-5 w-5 text-zinc-500" />
          <span className="font-medium text-zinc-300">ตัวกรอง</span>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {/* Search */}
          <div className="relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-zinc-500" />
            <input
              type="text"
              placeholder="ค้นหาเลขจอง/ชื่อลูกค้า..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-full pl-10 pr-4 py-2 bg-zinc-900 text-zinc-200 border border-zinc-700 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500"
            />
            {searchTerm && (
              <button
                onClick={() => setSearchTerm('')}
                className="absolute right-3 top-1/2 transform -translate-y-1/2 text-zinc-500 hover:text-zinc-400"
              >
                <X className="h-4 w-4" />
              </button>
            )}
          </div>

          {/* Status Filter */}
          <div>
            <select
              value={statusFilter}
              onChange={(e) => {
                setStatusFilter(e.target.value)
                setCurrentPage(1)
              }}
              className="w-full px-4 py-2 bg-zinc-900 text-zinc-200 border border-zinc-700 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500"
            >
              {statusOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </div>

          {/* Date Range Selector */}
          <div className="relative">
            <div className="flex items-center border border-zinc-700 rounded-lg overflow-hidden focus-within:ring-2 focus-within:ring-red-500 focus-within:border-red-500">
              <div className="flex items-center px-3 bg-zinc-800 border-r border-zinc-700 py-2">
                <Calendar className="h-4 w-4 text-zinc-500" />
              </div>
              <DatePicker
                selectsRange
                startDate={startDate}
                endDate={endDate}
                onChange={(update) => {
                  setDateRange(update)
                  setCurrentPage(1)
                }}
                placeholderText="เลือกช่วงวันที่"
                className="w-full px-3 py-2 bg-zinc-900 text-zinc-200 border-0 focus:ring-0 text-sm focus:outline-none"
                dateFormat="dd/MM/yy"
                isClearable
              />
            </div>
          </div>

          {/* Reset Button */}
          <div>
            <button
              onClick={handleResetFilters}
              className="w-full px-4 py-2 border border-zinc-700 rounded-lg text-zinc-300 hover:bg-zinc-800"
            >
              ล้างตัวกรอง
            </button>
          </div>
        </div>
      </div>

      {/* Table */}
      <div className="bg-zinc-900 rounded-lg border border-zinc-800 overflow-hidden">
        {loading ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="h-8 w-8 animate-spin text-red-600" />
            <span className="ml-2 text-zinc-400">กำลังโหลด...</span>
          </div>
        ) : error ? (
          <div className="flex items-center justify-center py-12 text-red-600">
            <AlertCircle className="h-6 w-6 mr-2" />
            <span>{error}</span>
          </div>
        ) : bookings.length === 0 ? (
          <div className="text-center py-12 text-zinc-500">
            <Calendar className="h-12 w-12 mx-auto text-zinc-600 mb-3" />
            <p>ไม่พบข้อมูลการจอง</p>
          </div>
        ) : (
          <>
            <div className="overflow-x-auto">
              <table className="w-full table-fixed divide-y divide-zinc-800">
                <thead className="bg-zinc-800">
                  <tr>
                    <th className="w-28 px-3 py-3 text-left text-xs font-medium text-zinc-500 uppercase tracking-wider">
                      เลขที่จอง
                    </th>
                    <th className="w-24 px-3 py-3 text-left text-xs font-medium text-zinc-500 uppercase tracking-wider">
                      วันที่จอง
                    </th>
                    <th className="w-24 px-3 py-3 text-left text-xs font-medium text-zinc-500 uppercase tracking-wider">
                      สถานะ
                    </th>
                    <th className="px-3 py-3 text-left text-xs font-medium text-zinc-500 uppercase tracking-wider">
                      ลูกค้า
                    </th>
                    <th className="w-24 px-3 py-3 text-left text-xs font-medium text-zinc-500 uppercase tracking-wider">
                      เช็คอิน
                    </th>
                    <th className="w-24 px-3 py-3 text-left text-xs font-medium text-zinc-500 uppercase tracking-wider">
                      เช็คเอาท์
                    </th>
                    <th className="w-20 px-3 py-3 text-left text-xs font-medium text-zinc-500 uppercase tracking-wider">
                      ห้อง
                    </th>
                    <th className="w-16 px-3 py-3 text-left text-xs font-medium text-zinc-500 uppercase tracking-wider">

                    </th>
                  </tr>
                </thead>
                <tbody className="bg-zinc-900 divide-y divide-zinc-800">
                  {bookings.map((booking) => (
                    <tr
                      key={booking.id}
                      onClick={() => handleEditBooking(booking)}
                      className="hover:bg-red-500/10 cursor-pointer"
                    >
                      <td className="px-3 py-3 truncate">
                        <span className="text-sm font-medium text-red-400">
                          {booking.bookNo}
                        </span>
                      </td>
                      <td className="px-3 py-3 text-sm text-zinc-100">
                        {formatDateBE(booking.createdAt)}
                      </td>
                      <td className="px-3 py-3">
                        <span
                          className={`inline-block px-2 py-1 text-xs rounded-full ${
                            statusColors[booking.status] || 'bg-zinc-700 text-zinc-300'
                          }`}
                        >
                          {statusLabels[booking.status] || booking.status}
                        </span>
                      </td>
                      <td className="px-3 py-3 truncate">
                        <span className="text-sm text-zinc-100">
                          {booking.customerName || '-'}
                        </span>
                      </td>
                      <td className="px-3 py-3 text-sm text-zinc-100">
                        {formatDateBE(booking.checkIn)}
                      </td>
                      <td className="px-3 py-3 text-sm text-zinc-100">
                        {formatDateBE(booking.checkOut)}
                      </td>
                      <td className="px-3 py-3">
                        <div className="flex items-center gap-1">
                          <BedDouble size={14} className="text-zinc-500" />
                          <span className="text-sm text-zinc-100">
                            {booking.roomCount}
                          </span>
                        </div>
                      </td>
                      <td className="px-3 py-3">
                        <button
                          onClick={(e) => {
                            e.stopPropagation()
                            handleEditBooking(booking)
                          }}
                          className="p-1.5 text-zinc-500 hover:text-red-400 hover:bg-red-500/10 rounded-lg"
                          title="แก้ไข"
                        >
                          <Edit3 className="w-4 h-4" />
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>

            {/* Pagination */}
            <div className="bg-zinc-900 px-4 py-3 flex items-center justify-between border-t border-zinc-800 sm:px-6">
              <div className="flex-1 flex justify-between sm:hidden">
                <button
                  onClick={() => goToPage(currentPage - 1)}
                  disabled={currentPage === 1}
                  className="relative inline-flex items-center px-4 py-2 border border-zinc-700 text-sm font-medium rounded-md text-zinc-300 bg-zinc-900 hover:bg-zinc-800 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  ก่อนหน้า
                </button>
                <button
                  onClick={() => goToPage(currentPage + 1)}
                  disabled={currentPage === totalPages}
                  className="ml-3 relative inline-flex items-center px-4 py-2 border border-zinc-700 text-sm font-medium rounded-md text-zinc-300 bg-zinc-900 hover:bg-zinc-800 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  ถัดไป
                </button>
              </div>
              <div className="hidden sm:flex-1 sm:flex sm:items-center sm:justify-between">
                <div>
                  <p className="text-sm text-zinc-300">
                    แสดง{' '}
                    <span className="font-medium">
                      {(currentPage - 1) * itemsPerPage + 1}
                    </span>{' '}
                    ถึง{' '}
                    <span className="font-medium">
                      {Math.min(currentPage * itemsPerPage, totalItems)}
                    </span>{' '}
                    จาก{' '}
                    <span className="font-medium">{totalItems}</span>{' '}
                    รายการ
                  </p>
                </div>
                <div>
                  <nav
                    className="relative z-0 inline-flex rounded-md -space-x-px"
                    aria-label="Pagination"
                  >
                    <button
                      onClick={() => goToPage(currentPage - 1)}
                      disabled={currentPage === 1}
                      className="relative inline-flex items-center px-2 py-2 rounded-l-md border border-zinc-700 bg-zinc-900 text-sm font-medium text-zinc-500 hover:bg-zinc-800 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                      <ChevronLeft className="h-5 w-5" />
                    </button>

                    {/* Page numbers */}
                    {Array.from({ length: Math.min(5, totalPages) }, (_, i) => {
                      let pageNum: number
                      if (totalPages <= 5) {
                        pageNum = i + 1
                      } else if (currentPage <= 3) {
                        pageNum = i + 1
                      } else if (currentPage >= totalPages - 2) {
                        pageNum = totalPages - 4 + i
                      } else {
                        pageNum = currentPage - 2 + i
                      }

                      return (
                        <button
                          key={pageNum}
                          onClick={() => goToPage(pageNum)}
                          className={`relative inline-flex items-center px-4 py-2 border text-sm font-medium ${
                            currentPage === pageNum
                              ? 'z-10 bg-red-500/10 border-red-500 text-red-400'
                              : 'bg-zinc-900 border-zinc-700 text-zinc-500 hover:bg-zinc-800'
                          }`}
                        >
                          {pageNum}
                        </button>
                      )
                    })}

                    <button
                      onClick={() => goToPage(currentPage + 1)}
                      disabled={currentPage === totalPages}
                      className="relative inline-flex items-center px-2 py-2 rounded-r-md border border-zinc-700 bg-zinc-900 text-sm font-medium text-zinc-500 hover:bg-zinc-800 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                      <ChevronRight className="h-5 w-5" />
                    </button>
                  </nav>
                </div>
              </div>
            </div>
          </>
        )}
      </div>

      {/* Loading overlay for detail fetch */}
      {loadingDetail && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-zinc-900 border border-zinc-800 rounded-lg p-6 flex items-center gap-3">
            <Loader2 className="w-6 h-6 animate-spin text-red-600" />
            <span className="text-zinc-300">กำลังโหลดข้อมูล...</span>
          </div>
        </div>
      )}

      {/* Booking Form Modal */}
      <BookingForm
        isOpen={showForm}
        onClose={handleCloseForm}
        onSave={handleSaveBooking}
        onCancel={handleCancelBooking}
        initialData={editingBooking}
        mode={formMode}
      />
    </div>
  )
}
