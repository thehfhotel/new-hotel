'use client'

import { useState, useEffect, useCallback } from 'react'
import { format, parseISO } from 'date-fns'
import { th } from 'date-fns/locale'
import {
  Search,
  Filter,
  ChevronLeft,
  ChevronRight,
  Loader2,
  AlertCircle,
  Calendar,
  RefreshCw,
} from 'lucide-react'

// Types
interface Booking {
  Book_No: string
  Book_Date: string
  Book_Date_in: string
  Book_Date_out: string
  Book_Cust_Name: string
  Book_Status: string
  Book_Room_Type: string
}

interface BookingsResponse {
  data: Booking[]
  pagination: {
    page: number
    limit: number
    total: number
    totalPages: number
  }
}

// Status configuration
const statusConfig: Record<string, { label: string; bgColor: string; textColor: string }> = {
  'จอง': { label: 'จอง', bgColor: 'bg-yellow-100', textColor: 'text-yellow-800' },
  'เข้าพัก': { label: 'เข้าพัก', bgColor: 'bg-green-100', textColor: 'text-green-800' },
  'ยกเลิก': { label: 'ยกเลิก', bgColor: 'bg-red-100', textColor: 'text-red-800' },
  'เสร็จสิ้น': { label: 'เสร็จสิ้น', bgColor: 'bg-gray-100', textColor: 'text-gray-800' },
}

const statusOptions = [
  { value: '', label: 'ทั้งหมด' },
  { value: 'จอง', label: 'จอง' },
  { value: 'เข้าพัก', label: 'เข้าพัก' },
  { value: 'ยกเลิก', label: 'ยกเลิก' },
]

export default function BookingsPage() {
  // State
  const [bookings, setBookings] = useState<Booking[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  // Pagination
  const [currentPage, setCurrentPage] = useState(1)
  const [totalPages, setTotalPages] = useState(1)
  const [totalItems, setTotalItems] = useState(0)
  const itemsPerPage = 10

  // Filters
  const [statusFilter, setStatusFilter] = useState('')
  const [startDate, setStartDate] = useState('')
  const [endDate, setEndDate] = useState('')
  const [searchTerm, setSearchTerm] = useState('')

  // Fetch bookings
  const fetchBookings = useCallback(async () => {
    setLoading(true)
    setError(null)

    try {
      const params = new URLSearchParams({
        page: currentPage.toString(),
        limit: itemsPerPage.toString(),
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
      setBookings(data.data)
      setTotalPages(data.pagination.totalPages)
      setTotalItems(data.pagination.total)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setLoading(false)
    }
  }, [currentPage, statusFilter, startDate, endDate, searchTerm])

  useEffect(() => {
    fetchBookings()
  }, [fetchBookings])

  // Format date helper
  const formatDate = (dateString: string) => {
    try {
      const date = parseISO(dateString)
      return format(date, 'd MMM yyyy', { locale: th })
    } catch {
      return dateString
    }
  }

  // Get status badge
  const getStatusBadge = (status: string) => {
    const config = statusConfig[status] || {
      label: status,
      bgColor: 'bg-gray-100',
      textColor: 'text-gray-800'
    }
    return (
      <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${config.bgColor} ${config.textColor}`}>
        {config.label}
      </span>
    )
  }

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

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold text-gray-900">รายการจอง</h1>
        <button
          onClick={fetchBookings}
          className="flex items-center space-x-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
        >
          <RefreshCw className="h-4 w-4" />
          <span>รีเฟรช</span>
        </button>
      </div>

      {/* Filters */}
      <div className="bg-white rounded-lg shadow p-4">
        <div className="flex items-center space-x-2 mb-4">
          <Filter className="h-5 w-5 text-gray-500" />
          <span className="font-medium text-gray-700">ตัวกรอง</span>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
          {/* Search */}
          <div className="relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
            <input
              type="text"
              placeholder="ค้นหาชื่อลูกค้า..."
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

          {/* Start Date */}
          <div className="relative">
            <Calendar className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
            <input
              type="date"
              value={startDate}
              onChange={(e) => {
                setStartDate(e.target.value)
                setCurrentPage(1)
              }}
              className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              placeholder="วันเริ่มต้น"
            />
          </div>

          {/* End Date */}
          <div className="relative">
            <Calendar className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
            <input
              type="date"
              value={endDate}
              onChange={(e) => {
                setEndDate(e.target.value)
                setCurrentPage(1)
              }}
              className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              placeholder="วันสิ้นสุด"
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
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      เลขที่จอง
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      วันที่จอง
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      วันเข้าพัก
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      วันออก
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      ชื่อลูกค้า
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      สถานะ
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      ประเภทห้อง
                    </th>
                  </tr>
                </thead>
                <tbody className="bg-white divide-y divide-gray-200">
                  {bookings.map((booking) => (
                    <tr key={booking.Book_No} className="hover:bg-gray-50 transition-colors">
                      <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-blue-600">
                        {booking.Book_No}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                        {formatDate(booking.Book_Date)}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                        {formatDate(booking.Book_Date_in)}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                        {formatDate(booking.Book_Date_out)}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                        {booking.Book_Cust_Name}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        {getStatusBadge(booking.Book_Status)}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                        {booking.Book_Room_Type}
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
    </div>
  )
}
