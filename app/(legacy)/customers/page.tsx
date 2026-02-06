'use client'

import { useState, useEffect, useCallback } from 'react'
import { format } from 'date-fns'
import {
  X,
  Users,
  Search,
  User,
  Phone,
  CreditCard,
  MapPin,
  Calendar,
  Home,
  CalendarDays,
  TrendingUp,
  Clock,
  Star,
  Plus,
  Edit3,
} from 'lucide-react'
import DataTable, { Column, PaginationInfo } from '@/components/DataTable'
import CustomerForm, { CustomerFormData } from '@/components/forms/CustomerForm'
import { useMode } from '@/contexts/ModeContext'

interface Customer {
  id: string
  name: string
  type: string
  phone: string
  idCard: string
  address: string
  lastVisit: string | null
}

interface NewCustomer {
  id: number
  firstName: string
  lastName: string
  phone: string
  email: string
  idCard: string
  address: string
  notes: string
  customerType: string
  active: boolean
  createdAt: string | null
  updatedAt: string | null
}

interface BookingHistory {
  id: number
  roomNumber: string
  roomType: string
  checkInDate: string
  checkOutDate: string
  status: string
  totalAmount: number
}

interface CustomerStats {
  totalBookings: number
  totalStays: number
  firstVisit: string | null
  lastVisit: string | null
  favoriteRoomType: string | null
  avgStayDays: number | null
}

const ITEMS_PER_PAGE = 20

export default function CustomersPage() {
  const { isNew } = useMode()

  const [customers, setCustomers] = useState<Customer[]>([])
  const [newCustomers, setNewCustomers] = useState<NewCustomer[]>([])
  const [loading, setLoading] = useState(true)
  const [searchQuery, setSearchQuery] = useState('')
  const [debouncedSearch, setDebouncedSearch] = useState('')
  const [pagination, setPagination] = useState<PaginationInfo>({
    currentPage: 1,
    totalPages: 1,
    totalItems: 0,
    itemsPerPage: ITEMS_PER_PAGE,
  })
  const [sortBy, setSortBy] = useState<string | null>(null)
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc' | null>(null)

  // Selected customer for modal
  const [selectedCustomer, setSelectedCustomer] = useState<Customer | null>(null)
  const [selectedNewCustomer, setSelectedNewCustomer] = useState<NewCustomer | null>(null)
  const [bookingHistory, setBookingHistory] = useState<BookingHistory[]>([])
  const [loadingHistory, setLoadingHistory] = useState(false)
  const [customerStats, setCustomerStats] = useState<CustomerStats | null>(null)
  const [loadingStats, setLoadingStats] = useState(false)
  const [showModal, setShowModal] = useState(false)

  // Customer form modal state (for New mode)
  const [showCustomerForm, setShowCustomerForm] = useState(false)
  const [customerFormMode, setCustomerFormMode] = useState<'create' | 'edit'>('create')
  const [customerToEdit, setCustomerToEdit] = useState<CustomerFormData | null>(null)

  // Debounce search input
  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedSearch(searchQuery)
      setPagination((prev) => ({ ...prev, currentPage: 1 }))
    }, 300)

    return () => clearTimeout(timer)
  }, [searchQuery])

  // Fetch customers for legacy mode
  const fetchLegacyCustomers = useCallback(async () => {
    setLoading(true)
    try {
      const params = new URLSearchParams({
        page: pagination.currentPage.toString(),
        limit: ITEMS_PER_PAGE.toString(),
        includeLastVisit: 'true',
      })

      if (debouncedSearch) {
        params.append('search', debouncedSearch)
      }

      if (sortBy && sortOrder) {
        params.append('sortBy', sortBy)
        params.append('sortOrder', sortOrder)
      }

      const response = await fetch(`/api/customers?${params}`)
      if (response.ok) {
        const data = await response.json()
        setCustomers(data.customers || data.data || [])
        setPagination((prev) => ({
          ...prev,
          totalItems: data.total || data.totalItems || 0,
          totalPages: Math.ceil((data.total || data.totalItems || 0) / ITEMS_PER_PAGE),
        }))
      }
    } catch (error) {
      console.error('Error fetching customers:', error)
    } finally {
      setLoading(false)
    }
  }, [pagination.currentPage, debouncedSearch, sortBy, sortOrder])

  // Fetch customers for new mode
  const fetchNewCustomers = useCallback(async () => {
    setLoading(true)
    try {
      const params = new URLSearchParams({
        page: pagination.currentPage.toString(),
        limit: ITEMS_PER_PAGE.toString(),
        activeOnly: 'true',
      })

      if (debouncedSearch) {
        params.append('search', debouncedSearch)
      }

      if (sortBy && sortOrder) {
        params.append('sortBy', sortBy)
        params.append('sortOrder', sortOrder)
      }

      const response = await fetch(`/api/new/customers?${params}`)
      if (response.ok) {
        const data = await response.json()
        setNewCustomers(data.data || [])
        setPagination((prev) => ({
          ...prev,
          totalItems: data.pagination?.total || 0,
          totalPages: data.pagination?.totalPages || 1,
        }))
      }
    } catch (error) {
      console.error('Error fetching new customers:', error)
    } finally {
      setLoading(false)
    }
  }, [pagination.currentPage, debouncedSearch, sortBy, sortOrder])

  // Fetch customers based on mode
  useEffect(() => {
    if (isNew) {
      fetchNewCustomers()
    } else {
      fetchLegacyCustomers()
    }
  }, [isNew, fetchLegacyCustomers, fetchNewCustomers])

  // Fetch booking history for selected customer (legacy mode)
  const fetchBookingHistory = async (customerId: string) => {
    setLoadingHistory(true)
    try {
      const response = await fetch(`/api/customers/${customerId}/bookings`)
      if (response.ok) {
        const data = await response.json()
        setBookingHistory(data.bookings || data || [])
      }
    } catch (error) {
      console.error('Error fetching booking history:', error)
      setBookingHistory([])
    } finally {
      setLoadingHistory(false)
    }
  }

  // Fetch customer stats (legacy mode)
  const fetchCustomerStats = async (customerId: string) => {
    setLoadingStats(true)
    try {
      const response = await fetch(`/api/customers/${customerId}/stats`)
      if (response.ok) {
        const data = await response.json()
        setCustomerStats(data.stats || null)
      }
    } catch (error) {
      console.error('Error fetching customer stats:', error)
      setCustomerStats(null)
    } finally {
      setLoadingStats(false)
    }
  }

  const handleLegacyRowClick = (customer: Customer) => {
    setSelectedCustomer(customer)
    setShowModal(true)
    fetchBookingHistory(customer.id)
    fetchCustomerStats(customer.id)
  }

  const handleNewRowClick = (customer: NewCustomer) => {
    setSelectedNewCustomer(customer)
    setCustomerToEdit({
      id: customer.id,
      firstName: customer.firstName || '',
      lastName: customer.lastName || '',
      phone: customer.phone || '',
      email: customer.email || '',
      idCard: customer.idCard || '',
      address: customer.address || '',
      notes: customer.notes || '',
    })
    setCustomerFormMode('edit')
    setShowCustomerForm(true)
  }

  const closeModal = () => {
    setShowModal(false)
    setSelectedCustomer(null)
    setBookingHistory([])
    setCustomerStats(null)
  }

  const handlePageChange = (page: number) => {
    setPagination((prev) => ({ ...prev, currentPage: page }))
  }

  const handleSort = (column: string, direction: 'asc' | 'desc' | null) => {
    setSortBy(direction ? column : null)
    setSortOrder(direction)
    // Reset to first page when sorting changes
    setPagination((prev) => ({ ...prev, currentPage: 1 }))
  }

  // Handle add new customer (New mode)
  const handleAddCustomer = () => {
    setCustomerToEdit(null)
    setCustomerFormMode('create')
    setShowCustomerForm(true)
  }

  // Handle save customer
  const handleSaveCustomer = async (data: CustomerFormData) => {
    const endpoint = data.id
      ? `/api/new/customers/${data.id}`
      : '/api/new/customers'
    const method = data.id ? 'PUT' : 'POST'

    const response = await fetch(endpoint, {
      method,
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        firstName: data.firstName,
        lastName: data.lastName || null,
        phone: data.phone || null,
        email: data.email || null,
        idCard: data.idCard || null,
        address: data.address || null,
        notes: data.notes || null,
      }),
    })

    const result = await response.json()

    if (!response.ok || !result.success) {
      throw new Error(result.message || 'Failed to save customer')
    }

    // Refresh the list
    fetchNewCustomers()
  }

  // Handle delete customer
  const handleDeleteCustomer = async (id: number) => {
    const response = await fetch(`/api/new/customers/${id}`, {
      method: 'DELETE',
    })

    const result = await response.json()

    if (!response.ok || !result.success) {
      throw new Error(result.message || 'Failed to delete customer')
    }

    // Refresh the list
    fetchNewCustomers()
  }

  // Close customer form
  const handleCloseCustomerForm = () => {
    setShowCustomerForm(false)
    setCustomerToEdit(null)
    setSelectedNewCustomer(null)
  }

  // Legacy mode columns
  const legacyColumns: Column<Customer>[] = [
    {
      key: 'id',
      header: 'ID',
      sortable: true,
      width: 'w-20',
    },
    {
      key: 'name',
      header: 'ชื่อ',
      sortable: true,
      render: (item) => (
        <div className="flex items-center gap-2">
          <User className="w-4 h-4 text-gray-400" />
          <span className="font-medium">{item.name}</span>
        </div>
      ),
    },
    {
      key: 'type',
      header: 'ประเภท',
      sortable: true,
      render: (item) => (
        <span
          className={`px-2 py-1 text-xs rounded-full ${
            item.type === 'VIP'
              ? 'bg-yellow-100 text-yellow-700'
              : item.type === 'Regular'
              ? 'bg-blue-100 text-blue-700'
              : 'bg-gray-100 text-gray-700'
          }`}
        >
          {item.type}
        </span>
      ),
    },
    {
      key: 'phone',
      header: 'โทรศัพท์',
      sortable: true,
      render: (item) => (
        <div className="flex items-center gap-2">
          <Phone className="w-4 h-4 text-gray-400" />
          <span>{item.phone || '-'}</span>
        </div>
      ),
    },
    {
      key: 'idCard',
      header: 'บัตรประชาชน',
      sortable: false,
      render: (item) => (
        <div className="flex items-center gap-2">
          <CreditCard className="w-4 h-4 text-gray-400" />
          <span className="font-mono text-sm">{item.idCard || '-'}</span>
        </div>
      ),
    },
    {
      key: 'lastVisit',
      header: 'เข้าพักล่าสุด',
      sortable: true,
      render: (item) => (
        <div className="flex items-center gap-2">
          <Calendar className="w-4 h-4 text-gray-400" />
          <span>{item.lastVisit ? format(new Date(item.lastVisit), 'dd/MM/yyyy') : '-'}</span>
        </div>
      ),
    },
    {
      key: 'address',
      header: 'ที่อยู่',
      sortable: false,
      render: (item) => (
        <div className="max-w-xs truncate" title={item.address}>
          {item.address || '-'}
        </div>
      ),
    },
  ]

  // New mode columns
  const newColumns: Column<NewCustomer>[] = [
    {
      key: 'id',
      header: 'ID',
      sortable: true,
      width: 'w-20',
    },
    {
      key: 'firstName',
      header: 'ชื่อ',
      sortable: true,
      render: (item) => (
        <div className="flex items-center gap-2">
          <User className="w-4 h-4 text-gray-400" />
          <span className="font-medium">
            {`${item.firstName || ''} ${item.lastName || ''}`.trim() || '-'}
          </span>
        </div>
      ),
    },
    {
      key: 'phone',
      header: 'โทรศัพท์',
      sortable: true,
      render: (item) => (
        <div className="flex items-center gap-2">
          <Phone className="w-4 h-4 text-gray-400" />
          <span>{item.phone || '-'}</span>
        </div>
      ),
    },
    {
      key: 'email',
      header: 'อีเมล',
      sortable: true,
      render: (item) => <span>{item.email || '-'}</span>,
    },
    {
      key: 'idCard',
      header: 'บัตรประชาชน',
      sortable: false,
      render: (item) => (
        <div className="flex items-center gap-2">
          <CreditCard className="w-4 h-4 text-gray-400" />
          <span className="font-mono text-sm">{item.idCard || '-'}</span>
        </div>
      ),
    },
    {
      key: 'address',
      header: 'ที่อยู่',
      sortable: false,
      render: (item) => (
        <div className="max-w-xs truncate" title={item.address}>
          {item.address || '-'}
        </div>
      ),
    },
    {
      key: 'actions',
      header: '',
      sortable: false,
      width: 'w-16',
      render: (item) => (
        <button
          onClick={(e) => {
            e.stopPropagation()
            handleNewRowClick(item)
          }}
          className="p-1.5 text-gray-400 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-colors"
          title="แก้ไข"
        >
          <Edit3 className="w-4 h-4" />
        </button>
      ),
    },
  ]

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Users className="w-8 h-8 text-blue-600" />
          <div>
            <h1 className="text-2xl font-bold text-gray-800">รายชื่อลูกค้า</h1>
            <p className="text-gray-600">
              จำนวนลูกค้าทั้งหมด {pagination.totalItems.toLocaleString()} คน
            </p>
          </div>
        </div>

        {/* Add Customer Button (only in New mode) */}
        {isNew && (
          <button
            onClick={handleAddCustomer}
            className="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
          >
            <Plus className="w-5 h-5" />
            เพิ่มลูกค้า
          </button>
        )}
      </div>

      {/* Search Bar */}
      <div className="bg-white rounded-lg shadow p-4">
        <div className="relative max-w-md">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="ค้นหาด้วยชื่อ, เบอร์โทร, เลขบัตรประชาชน หรือ ID..."
            className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors"
          />
          {searchQuery && (
            <button
              onClick={() => setSearchQuery('')}
              className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600"
              aria-label="ล้างการค้นหา"
            >
              <X className="w-4 h-4" />
            </button>
          )}
        </div>
        {debouncedSearch && (
          <p className="mt-2 text-sm text-gray-600">
            ผลการค้นหา &quot;{debouncedSearch}&quot;: พบ {pagination.totalItems.toLocaleString()} รายการ
          </p>
        )}
      </div>

      {/* Data Table */}
      {isNew ? (
        <DataTable
          columns={newColumns}
          data={newCustomers}
          onRowClick={handleNewRowClick}
          pagination={pagination}
          onPageChange={handlePageChange}
          loading={loading}
          onSort={handleSort}
          sortColumn={sortBy}
          sortDirection={sortOrder}
        />
      ) : (
        <DataTable
          columns={legacyColumns}
          data={customers}
          onRowClick={handleLegacyRowClick}
          pagination={pagination}
          onPageChange={handlePageChange}
          loading={loading}
          onSort={handleSort}
          sortColumn={sortBy}
          sortDirection={sortOrder}
        />
      )}

      {/* Customer Detail Modal (Legacy mode) */}
      {showModal && selectedCustomer && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-lg shadow-xl w-full max-w-3xl max-h-[90vh] overflow-hidden">
            {/* Modal Header */}
            <div className="flex items-center justify-between p-4 border-b bg-gray-50">
              <div className="flex items-center gap-3">
                <div className="w-12 h-12 bg-blue-100 rounded-full flex items-center justify-center">
                  <User className="w-6 h-6 text-blue-600" />
                </div>
                <div>
                  <h2 className="text-xl font-bold text-gray-800">
                    {selectedCustomer.name}
                  </h2>
                  <span
                    className={`px-2 py-0.5 text-xs rounded-full ${
                      selectedCustomer.type === 'VIP'
                        ? 'bg-yellow-100 text-yellow-700'
                        : selectedCustomer.type === 'Regular'
                        ? 'bg-blue-100 text-blue-700'
                        : 'bg-gray-100 text-gray-700'
                    }`}
                  >
                    {selectedCustomer.type}
                  </span>
                </div>
              </div>
              <button
                onClick={closeModal}
                className="p-2 hover:bg-gray-200 rounded-full transition-colors"
                aria-label="ปิด"
              >
                <X className="w-5 h-5" />
              </button>
            </div>

            {/* Modal Content */}
            <div className="p-4 overflow-y-auto max-h-[60vh]">
              {/* Customer Info */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                <div className="flex items-start gap-3 p-3 bg-gray-50 rounded-lg">
                  <Phone className="w-5 h-5 text-gray-500 mt-0.5" />
                  <div>
                    <p className="text-sm text-gray-500">โทรศัพท์</p>
                    <p className="font-medium text-gray-800">
                      {selectedCustomer.phone || '-'}
                    </p>
                  </div>
                </div>
                <div className="flex items-start gap-3 p-3 bg-gray-50 rounded-lg">
                  <CreditCard className="w-5 h-5 text-gray-500 mt-0.5" />
                  <div>
                    <p className="text-sm text-gray-500">บัตรประชาชน</p>
                    <p className="font-medium font-mono text-gray-800">
                      {selectedCustomer.idCard || '-'}
                    </p>
                  </div>
                </div>
                <div className="flex items-start gap-3 p-3 bg-gray-50 rounded-lg md:col-span-2">
                  <MapPin className="w-5 h-5 text-gray-500 mt-0.5" />
                  <div>
                    <p className="text-sm text-gray-500">ที่อยู่</p>
                    <p className="font-medium text-gray-800">
                      {selectedCustomer.address || '-'}
                    </p>
                  </div>
                </div>
              </div>

              {/* Customer Stats */}
              <div className="mb-6">
                <h3 className="flex items-center gap-2 text-lg font-semibold text-gray-800 mb-3">
                  <TrendingUp className="w-5 h-5" />
                  สถิติลูกค้า
                </h3>
                {loadingStats ? (
                  <div className="flex items-center justify-center py-4">
                    <div className="w-5 h-5 border-2 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
                    <span className="ml-2 text-gray-600">กำลังโหลดสถิติ...</span>
                  </div>
                ) : (
                  <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
                    {/* Total Bookings */}
                    <div className="p-3 bg-blue-50 rounded-lg border border-blue-100">
                      <div className="flex items-center gap-2 mb-1">
                        <CalendarDays className="w-4 h-4 text-blue-600" />
                        <span className="text-sm text-blue-700">จำนวนการจอง</span>
                      </div>
                      <p className="text-2xl font-bold text-blue-800">
                        {customerStats?.totalBookings ?? 0}
                      </p>
                    </div>

                    {/* Total Stays */}
                    <div className="p-3 bg-green-50 rounded-lg border border-green-100">
                      <div className="flex items-center gap-2 mb-1">
                        <TrendingUp className="w-4 h-4 text-green-600" />
                        <span className="text-sm text-green-700">จำนวนครั้งที่เข้าพัก</span>
                      </div>
                      <p className="text-2xl font-bold text-green-800">
                        {customerStats?.totalStays ?? 0}
                      </p>
                    </div>

                    {/* First Visit */}
                    <div className="p-3 bg-purple-50 rounded-lg border border-purple-100">
                      <div className="flex items-center gap-2 mb-1">
                        <Clock className="w-4 h-4 text-purple-600" />
                        <span className="text-sm text-purple-700">ลูกค้าตั้งแต่</span>
                      </div>
                      <p className="text-lg font-bold text-purple-800">
                        {customerStats?.firstVisit
                          ? format(new Date(customerStats.firstVisit), 'dd/MM/yyyy')
                          : '-'}
                      </p>
                    </div>

                    {/* Last Visit */}
                    <div className="p-3 bg-indigo-50 rounded-lg border border-indigo-100">
                      <div className="flex items-center gap-2 mb-1">
                        <Calendar className="w-4 h-4 text-indigo-600" />
                        <span className="text-sm text-indigo-700">เข้าพักล่าสุด</span>
                      </div>
                      <p className="text-lg font-bold text-indigo-800">
                        {customerStats?.lastVisit
                          ? format(new Date(customerStats.lastVisit), 'dd/MM/yyyy')
                          : '-'}
                      </p>
                    </div>

                    {/* Favorite Room Type */}
                    <div className="p-3 bg-yellow-50 rounded-lg border border-yellow-100">
                      <div className="flex items-center gap-2 mb-1">
                        <Home className="w-4 h-4 text-yellow-600" />
                        <span className="text-sm text-yellow-700">ห้องที่ชอบ</span>
                      </div>
                      <p className="text-lg font-bold text-yellow-800">
                        {customerStats?.favoriteRoomType || '-'}
                      </p>
                    </div>

                    {/* Average Stay Days */}
                    <div className="p-3 bg-orange-50 rounded-lg border border-orange-100">
                      <div className="flex items-center gap-2 mb-1">
                        <Star className="w-4 h-4 text-orange-600" />
                        <span className="text-sm text-orange-700">เฉลี่ย/ครั้ง</span>
                      </div>
                      <p className="text-lg font-bold text-orange-800">
                        {customerStats?.avgStayDays != null
                          ? `${customerStats.avgStayDays} วัน`
                          : '-'}
                      </p>
                    </div>
                  </div>
                )}
              </div>

              {/* Booking History */}
              <div>
                <h3 className="flex items-center gap-2 text-lg font-semibold text-gray-800 mb-3">
                  <Calendar className="w-5 h-5" />
                  ประวัติการจอง
                </h3>

                {loadingHistory ? (
                  <div className="flex items-center justify-center py-8">
                    <div className="w-5 h-5 border-2 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
                    <span className="ml-2 text-gray-600">กำลังโหลดประวัติ...</span>
                  </div>
                ) : bookingHistory.length === 0 ? (
                  <div className="text-center py-8 text-gray-500">
                    <Calendar className="w-12 h-12 text-gray-300 mx-auto mb-3" />
                    <p>ไม่พบประวัติการจอง</p>
                  </div>
                ) : (
                  <div className="space-y-3">
                    {bookingHistory.map((booking) => (
                      <div
                        key={booking.id}
                        className="p-3 border border-gray-200 rounded-lg hover:bg-gray-50"
                      >
                        <div className="flex items-start justify-between">
                          <div>
                            <div className="flex items-center gap-2 mb-1">
                              <Home className="w-4 h-4 text-gray-500" />
                              <span className="font-medium text-gray-800">
                                ห้อง {booking.roomNumber}
                              </span>
                              <span className="text-sm text-gray-500">
                                ({booking.roomType})
                              </span>
                            </div>
                            <div className="flex items-center gap-1 text-sm text-gray-600">
                              <Calendar className="w-4 h-4" />
                              {format(new Date(booking.checkInDate), 'dd/MM/yyyy')} -{' '}
                              {format(new Date(booking.checkOutDate), 'dd/MM/yyyy')}
                            </div>
                            {booking.totalAmount > 0 && (
                              <p className="text-sm text-gray-600 mt-1">
                                ยอดรวม: {booking.totalAmount.toLocaleString()} บาท
                              </p>
                            )}
                          </div>
                          <span
                            className={`px-2 py-1 text-xs rounded-full ${
                              booking.status === 'completed'
                                ? 'bg-green-100 text-green-700'
                                : booking.status === 'confirmed'
                                ? 'bg-blue-100 text-blue-700'
                                : booking.status === 'cancelled'
                                ? 'bg-red-100 text-red-700'
                                : 'bg-gray-100 text-gray-700'
                            }`}
                          >
                            {booking.status === 'completed'
                              ? 'เสร็จสิ้น'
                              : booking.status === 'confirmed'
                              ? 'ยืนยันแล้ว'
                              : booking.status === 'cancelled'
                              ? 'ยกเลิก'
                              : booking.status}
                          </span>
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </div>

            {/* Modal Footer */}
            <div className="p-4 border-t bg-gray-50">
              <button
                onClick={closeModal}
                className="w-full px-4 py-2 bg-gray-200 hover:bg-gray-300 text-gray-800 rounded-lg transition-colors"
              >
                ปิด
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Customer Form Modal (New mode) */}
      {isNew && (
        <CustomerForm
          isOpen={showCustomerForm}
          onClose={handleCloseCustomerForm}
          onSave={handleSaveCustomer}
          onDelete={handleDeleteCustomer}
          initialData={customerToEdit}
          mode={customerFormMode}
        />
      )}
    </div>
  )
}
