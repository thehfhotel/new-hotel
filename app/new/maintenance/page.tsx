'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  RefreshCw,
  Filter,
  Loader2,
  AlertCircle,
  Wrench,
  Plus,
} from 'lucide-react'
import MaintenanceCard from '@/components/maintenance/MaintenanceCard'
import MaintenanceRequestModal from '@/components/modals/MaintenanceRequestModal'
import { MaintenanceRequest, MaintenanceCategory, MaintenanceStatus } from '@/types/reports'

// API response types
interface RequestsResponse {
  success: boolean
  data: MaintenanceRequest[]
  pagination: {
    page: number
    limit: number
    total: number
    totalPages: number
  }
}

interface CategoriesResponse {
  success: boolean
  data: MaintenanceCategory[]
  total: number
}

interface RoomsResponse {
  success: boolean
  data: Array<{
    id: number
    roomNo: string
    roomTypeName: string | null
  }>
}

// Kanban column status type (excludes 'cancelled' which is not shown on board)
type KanbanStatus = 'open' | 'in_progress' | 'completed'

// Column configuration
const columns: Array<{
  id: KanbanStatus
  title: string
  subtitle: string
  bgColor: string
  borderColor: string
  headerColor: string
}> = [
  {
    id: 'open',
    title: 'รอดำเนินการ',
    subtitle: 'งานใหม่',
    bgColor: 'bg-red-50',
    borderColor: 'border-red-200',
    headerColor: 'bg-red-500',
  },
  {
    id: 'in_progress',
    title: 'กำลังดำเนินการ',
    subtitle: 'กำลังซ่อม',
    bgColor: 'bg-yellow-50',
    borderColor: 'border-yellow-200',
    headerColor: 'bg-yellow-500',
  },
  {
    id: 'completed',
    title: 'เสร็จสิ้น',
    subtitle: 'ซ่อมเรียบร้อย',
    bgColor: 'bg-green-50',
    borderColor: 'border-green-200',
    headerColor: 'bg-green-500',
  },
]

export default function MaintenancePage() {
  const [requests, setRequests] = useState<MaintenanceRequest[]>([])
  const [categories, setCategories] = useState<MaintenanceCategory[]>([])
  const [rooms, setRooms] = useState<Array<{ id: number; roomNo: string; roomTypeName: string | null }>>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  // Filters
  const [roomFilter, setRoomFilter] = useState<number | null>(null)
  const [categoryFilter, setCategoryFilter] = useState<number | null>(null)
  const [priorityFilter, setPriorityFilter] = useState<number | null>(null)

  // Modal state
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [editingRequest, setEditingRequest] = useState<MaintenanceRequest | null>(null)

  const [lastRefresh, setLastRefresh] = useState<Date>(new Date())

  // Fetch categories and rooms on mount
  useEffect(() => {
    const fetchMetadata = async () => {
      try {
        const [catRes, roomsRes] = await Promise.all([
          fetch('/api/new/maintenance/categories'),
          fetch('/api/new/rooms?limit=200'),
        ])

        if (catRes.ok) {
          const catData: CategoriesResponse = await catRes.json()
          if (catData.success) {
            setCategories(catData.data)
          }
        }

        if (roomsRes.ok) {
          const roomsData: RoomsResponse = await roomsRes.json()
          if (roomsData.success) {
            setRooms(roomsData.data)
          }
        }
      } catch (err) {
        console.error('Error fetching metadata:', err)
      }
    }

    fetchMetadata()
  }, [])

  // Fetch requests
  const fetchRequests = useCallback(async () => {
    try {
      const params = new URLSearchParams()
      params.set('limit', '200')
      if (roomFilter) params.set('roomId', roomFilter.toString())
      if (categoryFilter) params.set('categoryId', categoryFilter.toString())
      if (priorityFilter) params.set('priority', priorityFilter.toString())

      const response = await fetch(`/api/new/maintenance/requests?${params.toString()}`)
      if (!response.ok) {
        throw new Error('ไม่สามารถดึงข้อมูลได้')
      }

      const data: RequestsResponse = await response.json()
      if (!data.success) {
        throw new Error('ไม่สามารถดึงข้อมูลได้')
      }

      setRequests(data.data)
      setError(null)
      setLastRefresh(new Date())
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setLoading(false)
    }
  }, [roomFilter, categoryFilter, priorityFilter])

  // Initial fetch and auto-refresh every 30 seconds
  useEffect(() => {
    fetchRequests()

    const interval = setInterval(() => {
      fetchRequests()
    }, 30000)

    return () => clearInterval(interval)
  }, [fetchRequests])

  // Handle status change
  const handleStatusChange = async (requestId: number, newStatus: MaintenanceStatus) => {
    try {
      const response = await fetch(`/api/new/maintenance/requests/${requestId}/status`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ status: newStatus }),
      })

      if (!response.ok) {
        throw new Error('ไม่สามารถอัพเดทสถานะได้')
      }

      // Update local state optimistically
      setRequests((prev) =>
        prev.map((req) =>
          req.id === requestId
            ? {
                ...req,
                status: newStatus,
                startedAt: newStatus === 'in_progress' ? new Date().toISOString() : req.startedAt,
                completedAt: newStatus === 'completed' ? new Date().toISOString() : req.completedAt,
              }
            : req
        )
      )
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
      fetchRequests()
    }
  }

  // Handle edit request
  const handleEditRequest = (request: MaintenanceRequest) => {
    setEditingRequest(request)
    setIsModalOpen(true)
  }

  // Handle modal close
  const handleModalClose = () => {
    setIsModalOpen(false)
    setEditingRequest(null)
  }

  // Handle modal success
  const handleModalSuccess = () => {
    handleModalClose()
    fetchRequests()
  }

  // Group requests by status (for Kanban columns)
  const requestsByStatus: Record<KanbanStatus, MaintenanceRequest[]> = {
    open: requests.filter((r) => r.status === 'open'),
    in_progress: requests.filter((r) => r.status === 'in_progress'),
    completed: requests.filter((r) => r.status === 'completed'),
  }

  // Format last refresh time
  const formatLastRefresh = () => {
    return lastRefresh.toLocaleTimeString('th-TH', {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    })
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold text-gray-900 flex items-center gap-2">
            <Wrench className="w-7 h-7 text-orange-600" />
            แจ้งซ่อม
          </h1>
          <p className="text-sm text-gray-500 mt-1">
            จัดการคำขอซ่อมบำรุง | รีเฟรชล่าสุด: {formatLastRefresh()}
          </p>
        </div>

        <div className="flex items-center gap-3 flex-wrap">
          {/* Room Filter */}
          <div className="flex items-center gap-2">
            <Filter className="w-4 h-4 text-gray-400" />
            <select
              value={roomFilter ?? ''}
              onChange={(e) =>
                setRoomFilter(e.target.value ? parseInt(e.target.value) : null)
              }
              className="px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-orange-500 focus:border-orange-500"
            >
              <option value="">ทุกห้อง</option>
              {rooms.map((room) => (
                <option key={room.id} value={room.id}>
                  {room.roomNo}
                </option>
              ))}
            </select>
          </div>

          {/* Category Filter */}
          <select
            value={categoryFilter ?? ''}
            onChange={(e) =>
              setCategoryFilter(e.target.value ? parseInt(e.target.value) : null)
            }
            className="px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-orange-500 focus:border-orange-500"
          >
            <option value="">ทุกหมวด</option>
            {categories.map((cat) => (
              <option key={cat.id} value={cat.id}>
                {cat.name}
              </option>
            ))}
          </select>

          {/* Priority Filter */}
          <select
            value={priorityFilter ?? ''}
            onChange={(e) =>
              setPriorityFilter(e.target.value ? parseInt(e.target.value) : null)
            }
            className="px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-orange-500 focus:border-orange-500"
          >
            <option value="">ทุกความเร่งด่วน</option>
            <option value="3">สูง</option>
            <option value="2">ปานกลาง</option>
            <option value="1">ต่ำ</option>
          </select>

          {/* Refresh Button */}
          <button
            onClick={fetchRequests}
            disabled={loading}
            className="flex items-center gap-2 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 disabled:opacity-50"
          >
            <RefreshCw className={`w-4 h-4 ${loading ? 'animate-spin' : ''}`} />
            <span className="hidden sm:inline">รีเฟรช</span>
          </button>

          {/* Add Request Button */}
          <button
            onClick={() => setIsModalOpen(true)}
            className="flex items-center gap-2 px-4 py-2 bg-orange-600 text-white rounded-lg hover:bg-orange-700"
          >
            <Plus className="w-4 h-4" />
            <span>แจ้งซ่อม</span>
          </button>
        </div>
      </div>

      {/* Error Message */}
      {error && (
        <div className="flex items-center gap-2 p-4 bg-red-50 border border-red-200 rounded-lg text-red-700">
          <AlertCircle className="w-5 h-5" />
          <span>{error}</span>
        </div>
      )}

      {/* Kanban Board */}
      {loading && requests.length === 0 ? (
        <div className="flex items-center justify-center py-12">
          <Loader2 className="w-8 h-8 animate-spin text-orange-600" />
          <span className="ml-2 text-gray-600">กำลังโหลด...</span>
        </div>
      ) : (
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {columns.map((column) => (
            <div
              key={column.id}
              className={`${column.bgColor} ${column.borderColor} border rounded-xl overflow-hidden`}
            >
              {/* Column Header */}
              <div className={`${column.headerColor} px-4 py-3`}>
                <div className="flex items-center justify-between">
                  <div>
                    <h2 className="text-lg font-bold text-white">
                      {column.title}
                    </h2>
                    <p className="text-sm text-white text-opacity-80">
                      {column.subtitle}
                    </p>
                  </div>
                  <div className="flex items-center justify-center w-10 h-10 bg-white bg-opacity-20 rounded-full">
                    <span className="text-xl font-bold text-white">
                      {requestsByStatus[column.id].length}
                    </span>
                  </div>
                </div>
              </div>

              {/* Column Content */}
              <div className="p-4 space-y-3 max-h-[600px] overflow-y-auto">
                {requestsByStatus[column.id].length === 0 ? (
                  <div className="text-center py-8 text-gray-500">
                    <p>ไม่มีงานในหมวดนี้</p>
                  </div>
                ) : (
                  requestsByStatus[column.id]
                    .sort((a, b) => b.priority - a.priority) // Sort by priority descending
                    .map((request) => (
                      <MaintenanceCard
                        key={request.id}
                        request={request}
                        onStatusChange={handleStatusChange}
                        onEdit={handleEditRequest}
                        disabled={loading}
                      />
                    ))
                )}
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Auto-refresh indicator */}
      <div className="text-center text-sm text-gray-400">
        ระบบจะรีเฟรชอัตโนมัติทุก 30 วินาที
      </div>

      {/* Maintenance Request Modal */}
      <MaintenanceRequestModal
        isOpen={isModalOpen}
        onClose={handleModalClose}
        onSuccess={handleModalSuccess}
        editRequest={editingRequest}
        categories={categories}
        rooms={rooms}
      />
    </div>
  )
}
