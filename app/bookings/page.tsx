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
  BedDouble,
  ArrowUpDown,
  ArrowUp,
  ArrowDown,
} from 'lucide-react'
import BookingDetailDrawer from '@/components/BookingDetailDrawer'

// Types
interface Room {
  roomNo: string
  roomType: string
}

interface GroupedBooking {
  bookNo: string
  bookDate: string
  checkIn: string
  checkOut: string
  customer: {
    name: string
  }
  status: string
  rooms: Room[]
  roomCount: number
}

interface BookingsResponse {
  success: boolean
  data: GroupedBooking[]
  pagination: {
    page: number
    limit: number
    total: number
    totalPages: number
  }
}

const statusOptions = [
  { value: '', label: 'สถานะทั้งหมด' },
  { value: 'จอง', label: 'จอง' },
  { value: 'เข้าพัก', label: 'เข้าพัก' },
  { value: 'เสร็จสิ้น', label: 'เสร็จสิ้น' },
  { value: 'ยกเลิก', label: 'ยกเลิก' },
]

// Format date helper
function formatDate(dateString: string): string {
  if (!dateString) return '-'
  const date = new Date(dateString)
  return date.toLocaleDateString('th-TH', {
    day: '2-digit',
    month: '2-digit',
    year: '2-digit',
    timeZone: 'UTC',
  })
}

export default function BookingsPage() {
  // State
  const [bookings, setBookings] = useState<GroupedBooking[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  // Pagination
  const [currentPage, setCurrentPage] = useState(1)
  const [totalPages, setTotalPages] = useState(1)
  const [totalItems, setTotalItems] = useState(0)
  const itemsPerPage = 15

  // Filters
  const [statusFilter, setStatusFilter] = useState('')
  const [startDate, setStartDate] = useState('')
  const [endDate, setEndDate] = useState('')
  const [searchTerm, setSearchTerm] = useState('')

  // Drawer
  const [selectedBookNo, setSelectedBookNo] = useState<string | null>(null)

  // Sorting
  const [sortBy, setSortBy] = useState('bookDate')
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('desc')

  // Fetch bookings
  const fetchBookings = useCallback(async () => {
    setLoading(true)
    setError(null)

    try {
      const params = new URLSearchParams({
        page: currentPage.toString(),
        limit: itemsPerPage.toString(),
        sortBy,
        sortOrder,
      })

      if (statusFilter) params.append('status', statusFilter)
      if (startDate) params.append('startDate', startDate)
      if (endDate) params.append('endDate', endDate)
      if (searchTerm) params.append('search', searchTerm)

      const response = await fetch(`/api/bookings?${params.toString()}`)

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
  }, [currentPage, statusFilter, startDate, endDate, searchTerm, sortBy, sortOrder])

  useEffect(() => {
    fetchBookings()
  }, [fetchBookings])

  // Handle filter reset
  const handleResetFilters = () => {
    setStatusFilter('')
    setStartDate('')
    setEndDate('')
    setSearchTerm('')
    setCurrentPage(1)
  }

  // Pagination handlers
  const goToPage = (page: number) => {
    if (page >= 1 && page <= totalPages) {
      setCurrentPage(page)
    }
  }

  // Handle column sort
  const handleSort = (column: string) => {
    if (sortBy === column) {
      setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc')
    } else {
      setSortBy(column)
      setSortOrder('asc')
    }
    setCurrentPage(1)
  }

  // Get sort icon for column
  const getSortIcon = (column: string) => {
    if (sortBy !== column) {
      return <ArrowUpDown size={14} className="text-gray-400" />
    }
    return sortOrder === 'asc'
      ? <ArrowUp size={14} className="text-blue-600" />
      : <ArrowDown size={14} className="text-blue-600" />
  }

  // Get rooms display - show room types
  const getRoomsDisplay = (rooms: Room[]) => {
    const types = rooms.map(r => r.roomType).filter(r => r !== '-')
    if (types.length === 0) return '-'
    // Count unique types
    const typeCounts = types.reduce((acc, t) => {
      acc[t] = (acc[t] || 0) + 1
      return acc
    }, {} as Record<string, number>)
    return Object.entries(typeCounts).map(([t, c]) => c > 1 ? `${t} x${c}` : t).join(', ')
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">การจองทั้งหมด</h1>
          <p className="text-sm text-gray-500 mt-1">
            {totalItems} รายการ (จัดกลุ่มตามเลขที่จอง)
          </p>
        </div>
        <button
          onClick={fetchBookings}
          disabled={loading}
          className="flex items-center space-x-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors disabled:opacity-50"
        >
          <RefreshCw className={`h-4 w-4 ${loading ? 'animate-spin' : ''}`} />
          <span>รีเฟรช</span>
        </button>
      </div>

      {/* Filters */}
      <div className="bg-white rounded-lg shadow p-4">
        <div className="flex items-center space-x-2 mb-4">
          <Filter className="h-5 w-5 text-gray-500" />
          <span className="font-medium text-gray-700">ตัวกรอง</span>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {/* Search */}
          <div className="relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
            <input
              type="text"
              placeholder="ค้นหาเลขจอง/ชื่อลูกค้า..."
              value={searchTerm}
              onChange={(e) => {
                setSearchTerm(e.target.value)
                setCurrentPage(1)
              }}
              className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            />
          </div>

          {/* Status Filter */}
          <div>
            <select
              value={statusFilter}
              onChange={(e) => {
                setStatusFilter(e.target.value)
                setCurrentPage(1)
              }}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            >
              {statusOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </div>

          {/* Date Range Selector */}
          <div className="flex items-center border border-gray-300 rounded-lg overflow-hidden focus-within:ring-2 focus-within:ring-blue-500 focus-within:border-blue-500">
            <div className="flex items-center px-3 bg-gray-50 border-r border-gray-300 h-full">
              <Calendar className="h-4 w-4 text-gray-400" />
            </div>
            <input
              type="date"
              value={startDate}
              onChange={(e) => {
                setStartDate(e.target.value)
                setCurrentPage(1)
              }}
              className="flex-1 px-2 py-2 border-0 focus:ring-0 text-sm"
              title="เช็คอิน"
            />
            <span className="text-gray-400 px-1">→</span>
            <input
              type="date"
              value={endDate}
              onChange={(e) => {
                setEndDate(e.target.value)
                setCurrentPage(1)
              }}
              className="flex-1 px-2 py-2 border-0 focus:ring-0 text-sm"
              title="เช็คเอาท์"
            />
          </div>

          {/* Reset Button */}
          <div>
            <button
              onClick={handleResetFilters}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50 transition-colors"
            >
              ล้างตัวกรอง
            </button>
          </div>
        </div>
      </div>

      {/* Table */}
      <div className="bg-white rounded-lg shadow overflow-hidden">
        {loading ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="h-8 w-8 animate-spin text-blue-600" />
            <span className="ml-2 text-gray-600">กำลังโหลด...</span>
          </div>
        ) : error ? (
          <div className="flex items-center justify-center py-12 text-red-600">
            <AlertCircle className="h-6 w-6 mr-2" />
            <span>{error}</span>
          </div>
        ) : bookings.length === 0 ? (
          <div className="text-center py-12 text-gray-500">
            ไม่พบข้อมูลการจอง
          </div>
        ) : (
          <>
            <div className="overflow-x-auto">
              <table className="min-w-full divide-y divide-gray-200">
                <thead className="bg-gray-50">
                  <tr>
                    <th
                      onClick={() => handleSort('bookNo')}
                      className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:bg-gray-100 select-none"
                    >
                      <div className="flex items-center gap-1">
                        เลขที่จอง
                        {getSortIcon('bookNo')}
                      </div>
                    </th>
                    <th
                      onClick={() => handleSort('status')}
                      className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:bg-gray-100 select-none"
                    >
                      <div className="flex items-center gap-1">
                        สถานะ
                        {getSortIcon('status')}
                      </div>
                    </th>
                    <th
                      onClick={() => handleSort('customer')}
                      className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:bg-gray-100 select-none"
                    >
                      <div className="flex items-center gap-1">
                        ลูกค้า
                        {getSortIcon('customer')}
                      </div>
                    </th>
                    <th
                      onClick={() => handleSort('checkIn')}
                      className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:bg-gray-100 select-none"
                    >
                      <div className="flex items-center gap-1">
                        เช็คอิน
                        {getSortIcon('checkIn')}
                      </div>
                    </th>
                    <th
                      onClick={() => handleSort('checkOut')}
                      className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:bg-gray-100 select-none"
                    >
                      <div className="flex items-center gap-1">
                        เช็คเอาท์
                        {getSortIcon('checkOut')}
                      </div>
                    </th>
                    <th
                      onClick={() => handleSort('roomCount')}
                      className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:bg-gray-100 select-none"
                    >
                      <div className="flex items-center gap-1">
                        ห้องพัก
                        {getSortIcon('roomCount')}
                      </div>
                    </th>
                  </tr>
                </thead>
                <tbody className="bg-white divide-y divide-gray-200">
                  {bookings.map((booking) => (
                    <tr
                      key={booking.bookNo}
                      onClick={() => setSelectedBookNo(booking.bookNo)}
                      className="hover:bg-blue-50 cursor-pointer transition-colors"
                    >
                      <td className="px-4 py-4 whitespace-nowrap">
                        <span className="text-sm font-medium text-blue-600">
                          {booking.bookNo}
                        </span>
                      </td>
                      <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900">
                        {booking.status}
                      </td>
                      <td className="px-4 py-4 whitespace-nowrap">
                        <span className="text-sm text-gray-900">
                          {booking.customer.name || '-'}
                        </span>
                      </td>
                      <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900">
                        {formatDate(booking.checkIn)}
                      </td>
                      <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900">
                        {formatDate(booking.checkOut)}
                      </td>
                      <td className="px-4 py-4 whitespace-nowrap">
                        <div className="flex items-center gap-2">
                          <BedDouble size={16} className="text-gray-400" />
                          <div className="flex flex-col">
                            <span className="text-sm text-gray-900">
                              {getRoomsDisplay(booking.rooms)}
                            </span>
                            <span className="text-xs text-gray-500">
                              {booking.roomCount} ห้อง
                            </span>
                          </div>
                        </div>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>

            {/* Pagination */}
            <div className="bg-white px-4 py-3 flex items-center justify-between border-t border-gray-200 sm:px-6">
              <div className="flex-1 flex justify-between sm:hidden">
                <button
                  onClick={() => goToPage(currentPage - 1)}
                  disabled={currentPage === 1}
                  className="relative inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  ก่อนหน้า
                </button>
                <button
                  onClick={() => goToPage(currentPage + 1)}
                  disabled={currentPage === totalPages}
                  className="ml-3 relative inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  ถัดไป
                </button>
              </div>
              <div className="hidden sm:flex-1 sm:flex sm:items-center sm:justify-between">
                <div>
                  <p className="text-sm text-gray-700">
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
                  <nav className="relative z-0 inline-flex rounded-md shadow-sm -space-x-px" aria-label="Pagination">
                    <button
                      onClick={() => goToPage(currentPage - 1)}
                      disabled={currentPage === 1}
                      className="relative inline-flex items-center px-2 py-2 rounded-l-md border border-gray-300 bg-white text-sm font-medium text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
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
                              ? 'z-10 bg-blue-50 border-blue-500 text-blue-600'
                              : 'bg-white border-gray-300 text-gray-500 hover:bg-gray-50'
                          }`}
                        >
                          {pageNum}
                        </button>
                      )
                    })}

                    <button
                      onClick={() => goToPage(currentPage + 1)}
                      disabled={currentPage === totalPages}
                      className="relative inline-flex items-center px-2 py-2 rounded-r-md border border-gray-300 bg-white text-sm font-medium text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
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

      {/* Detail Drawer */}
      <BookingDetailDrawer
        bookNo={selectedBookNo}
        onClose={() => setSelectedBookNo(null)}
      />
    </div>
  )
}
