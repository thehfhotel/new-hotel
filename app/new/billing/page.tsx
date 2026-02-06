'use client'

import { useState, useEffect, useCallback } from 'react'
import Link from 'next/link'
import {
  Receipt,
  Search,
  Filter,
  Eye,
  Loader2,
  AlertCircle,
  Calendar,
  User,
  DoorOpen,
} from 'lucide-react'

interface CheckIn {
  id: number
  cinNo: string
  customerName: string | null
  roomNo: string | null
  roomTypeName: string | null
  checkInTime: string | null
  checkOutTime: string | null
  expectedCheckout: string | null
  status: string
  totalAmount: number | null
  paymentStatus: string | null
}

interface Pagination {
  page: number
  limit: number
  total: number
  totalPages: number
}

export default function BillingPage() {
  const [checkIns, setCheckIns] = useState<CheckIn[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [pagination, setPagination] = useState<Pagination>({
    page: 1,
    limit: 20,
    total: 0,
    totalPages: 0,
  })

  // Filters
  const [searchTerm, setSearchTerm] = useState('')
  const [statusFilter, setStatusFilter] = useState<string>('all')
  const [startDate, setStartDate] = useState('')
  const [endDate, setEndDate] = useState('')
  const [showFilters, setShowFilters] = useState(false)

  const fetchCheckIns = useCallback(async () => {
    setLoading(true)
    setError(null)

    try {
      const params = new URLSearchParams()
      params.set('page', pagination.page.toString())
      params.set('limit', pagination.limit.toString())

      if (statusFilter !== 'all') {
        params.set('status', statusFilter)
      }
      if (startDate) {
        params.set('startDate', startDate)
      }
      if (endDate) {
        params.set('endDate', endDate)
      }

      const response = await fetch(`/api/new/checkins?${params.toString()}`)
      const data = await response.json()

      if (data.success) {
        let filteredData = data.data || []

        // Client-side search filtering
        if (searchTerm) {
          const term = searchTerm.toLowerCase()
          filteredData = filteredData.filter(
            (item: CheckIn) =>
              item.cinNo?.toLowerCase().includes(term) ||
              item.customerName?.toLowerCase().includes(term)
          )
        }

        setCheckIns(filteredData)
        setPagination(data.pagination)
      } else {
        setError(data.error || 'ไม่สามารถโหลดข้อมูลได้')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการโหลดข้อมูล')
    } finally {
      setLoading(false)
    }
  }, [pagination.page, pagination.limit, statusFilter, startDate, endDate, searchTerm])

  useEffect(() => {
    fetchCheckIns()
  }, [fetchCheckIns])

  const formatDate = (dateStr: string | null) => {
    if (!dateStr) return '-'
    const date = new Date(dateStr)
    return date.toLocaleDateString('th-TH', {
      day: 'numeric',
      month: 'short',
      year: '2-digit',
      timeZone: 'UTC',
    })
  }

  const formatCurrency = (amount: number | null) => {
    if (amount === null || amount === undefined) return '-'
    return new Intl.NumberFormat('th-TH', {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    }).format(amount)
  }

  const getStatusBadge = (status: string) => {
    switch (status.toLowerCase()) {
      case 'active':
        return (
          <span className="px-2 py-1 text-xs font-medium rounded-full bg-emerald-500/10 text-emerald-400">
            เข้าพักอยู่
          </span>
        )
      case 'checkedout':
        return (
          <span className="px-2 py-1 text-xs font-medium rounded-full bg-sky-500/10 text-sky-400">
            เช็คเอาท์แล้ว
          </span>
        )
      case 'cancelled':
        return (
          <span className="px-2 py-1 text-xs font-medium rounded-full bg-red-500/10 text-red-400">
            ยกเลิก
          </span>
        )
      default:
        return (
          <span className="px-2 py-1 text-xs font-medium rounded-full bg-zinc-700 text-zinc-300">
            {status}
          </span>
        )
    }
  }

  const handlePageChange = (newPage: number) => {
    setPagination((prev) => ({ ...prev, page: newPage }))
  }

  const clearFilters = () => {
    setSearchTerm('')
    setStatusFilter('all')
    setStartDate('')
    setEndDate('')
    setPagination((prev) => ({ ...prev, page: 1 }))
  }

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Receipt className="w-8 h-8 text-red-400" />
          <div>
            <h1 className="text-2xl font-bold text-zinc-100">ใบแจ้งหนี้</h1>
            <p className="text-zinc-400">Billing / Invoice</p>
          </div>
        </div>
      </div>

      {/* Search and Filters */}
      <div className="bg-zinc-900 rounded-lg border border-zinc-800 p-4">
        <div className="flex flex-col md:flex-row gap-4">
          {/* Search Input */}
          <div className="flex-1">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-5 h-5 text-zinc-500" />
              <input
                type="text"
                placeholder="ค้นหาชื่อผู้เข้าพักหรือเลขที่เช็คอิน..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="w-full pl-10 pr-4 py-2 bg-zinc-900 text-zinc-200 border border-zinc-700 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-transparent"
              />
            </div>
          </div>

          {/* Status Filter */}
          <div className="flex items-center gap-2">
            <select
              value={statusFilter}
              onChange={(e) => {
                setStatusFilter(e.target.value)
                setPagination((prev) => ({ ...prev, page: 1 }))
              }}
              className="px-4 py-2 bg-zinc-900 text-zinc-200 border border-zinc-700 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-transparent"
            >
              <option value="all">ทุกสถานะ</option>
              <option value="active">เข้าพักอยู่</option>
              <option value="checkedout">เช็คเอาท์แล้ว</option>
            </select>

            <button
              onClick={() => setShowFilters(!showFilters)}
              className={`flex items-center gap-2 px-4 py-2 rounded-lg border ${
                showFilters
                  ? 'bg-red-500/10 border-red-500/30 text-red-400'
                  : 'border-zinc-700 text-zinc-300 hover:bg-zinc-800'
              }`}
            >
              <Filter className="w-4 h-4" />
              ตัวกรอง
            </button>
          </div>
        </div>

        {/* Advanced Filters */}
        {showFilters && (
          <div className="mt-4 pt-4 border-t border-zinc-800">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div>
                <label className="block text-sm font-medium text-zinc-300 mb-1">
                  <Calendar className="w-4 h-4 inline mr-1" />
                  จากวันที่
                </label>
                <input
                  type="date"
                  value={startDate}
                  onChange={(e) => {
                    setStartDate(e.target.value)
                    setPagination((prev) => ({ ...prev, page: 1 }))
                  }}
                  className="w-full px-3 py-2 bg-zinc-900 text-zinc-200 border border-zinc-700 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-transparent"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-zinc-300 mb-1">
                  <Calendar className="w-4 h-4 inline mr-1" />
                  ถึงวันที่
                </label>
                <input
                  type="date"
                  value={endDate}
                  onChange={(e) => {
                    setEndDate(e.target.value)
                    setPagination((prev) => ({ ...prev, page: 1 }))
                  }}
                  className="w-full px-3 py-2 bg-zinc-900 text-zinc-200 border border-zinc-700 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-transparent"
                />
              </div>
              <div className="flex items-end">
                <button
                  onClick={clearFilters}
                  className="px-4 py-2 text-sm text-zinc-400 hover:text-zinc-300 hover:bg-zinc-800 rounded-lg"
                >
                  ล้างตัวกรอง
                </button>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Error Message */}
      {error && (
        <div className="flex items-center gap-2 p-4 bg-red-950/50 border border-red-900/50 rounded-lg text-red-400">
          <AlertCircle className="w-5 h-5 flex-shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {/* Table */}
      <div className="bg-zinc-900 rounded-lg border border-zinc-800 overflow-hidden">
        {loading ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="w-8 h-8 animate-spin text-red-500" />
            <span className="ml-2 text-zinc-400">กำลังโหลดข้อมูล...</span>
          </div>
        ) : checkIns.length === 0 ? (
          <div className="text-center py-12 text-zinc-500">
            <Receipt className="w-16 h-16 text-zinc-600 mx-auto mb-4" />
            <p className="text-lg">ไม่พบข้อมูลการเข้าพัก</p>
            <p className="text-sm mt-1">ลองเปลี่ยนเงื่อนไขการค้นหา</p>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-zinc-800">
              <thead className="bg-zinc-800">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-zinc-500 uppercase tracking-wider">
                    เลขที่เช็คอิน
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-zinc-500 uppercase tracking-wider">
                    <DoorOpen className="w-4 h-4 inline mr-1" />
                    ห้อง
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-zinc-500 uppercase tracking-wider">
                    <User className="w-4 h-4 inline mr-1" />
                    ชื่อผู้เข้าพัก
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-zinc-500 uppercase tracking-wider">
                    วันที่เช็คอิน
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-zinc-500 uppercase tracking-wider">
                    วันที่เช็คเอาท์
                  </th>
                  <th className="px-6 py-3 text-right text-xs font-medium text-zinc-500 uppercase tracking-wider">
                    ยอดรวม (บาท)
                  </th>
                  <th className="px-6 py-3 text-center text-xs font-medium text-zinc-500 uppercase tracking-wider">
                    สถานะ
                  </th>
                  <th className="px-6 py-3 text-center text-xs font-medium text-zinc-500 uppercase tracking-wider">
                    ดำเนินการ
                  </th>
                </tr>
              </thead>
              <tbody className="bg-zinc-900 divide-y divide-zinc-800">
                {checkIns.map((checkIn) => (
                  <tr key={checkIn.id} className="hover:bg-zinc-800">
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className="text-sm font-medium text-zinc-100">{checkIn.cinNo}</span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div>
                        <span className="text-sm font-medium text-zinc-100">
                          {checkIn.roomNo || '-'}
                        </span>
                        {checkIn.roomTypeName && (
                          <span className="block text-xs text-zinc-500">{checkIn.roomTypeName}</span>
                        )}
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className="text-sm text-zinc-100">{checkIn.customerName || '-'}</span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className="text-sm text-zinc-400">{formatDate(checkIn.checkInTime)}</span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className="text-sm text-zinc-400">
                        {formatDate(checkIn.checkOutTime || checkIn.expectedCheckout)}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-right">
                      <span className="text-sm font-medium text-zinc-100">
                        {formatCurrency(checkIn.totalAmount)}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-center">
                      {getStatusBadge(checkIn.status)}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-center">
                      <Link
                        href={`/new/billing/${checkIn.id}`}
                        className="inline-flex items-center gap-1 px-3 py-1.5 text-sm font-medium text-red-400 hover:text-red-300 hover:bg-red-500/10 rounded-lg"
                      >
                        <Eye className="w-4 h-4" />
                        ดูใบแจ้งหนี้
                      </Link>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}

        {/* Pagination */}
        {!loading && checkIns.length > 0 && pagination.totalPages > 1 && (
          <div className="px-6 py-4 border-t border-zinc-800 flex items-center justify-between">
            <div className="text-sm text-zinc-400">
              แสดง {(pagination.page - 1) * pagination.limit + 1} -{' '}
              {Math.min(pagination.page * pagination.limit, pagination.total)} จาก {pagination.total}{' '}
              รายการ
            </div>
            <div className="flex items-center gap-2">
              <button
                onClick={() => handlePageChange(pagination.page - 1)}
                disabled={pagination.page === 1}
                className="px-3 py-1 text-sm border border-zinc-700 rounded-lg hover:bg-zinc-800 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                ก่อนหน้า
              </button>
              <span className="px-3 py-1 text-sm">
                หน้า {pagination.page} / {pagination.totalPages}
              </span>
              <button
                onClick={() => handlePageChange(pagination.page + 1)}
                disabled={pagination.page === pagination.totalPages}
                className="px-3 py-1 text-sm border border-zinc-700 rounded-lg hover:bg-zinc-800 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                ถัดไป
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}
