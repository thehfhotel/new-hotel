'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  DoorOpen,
  Package,
  Search,
  X,
  Loader2,
  AlertCircle,
  CheckCircle,
  AlertTriangle,
  ClipboardCheck,
  RefreshCw,
  Filter,
} from 'lucide-react'
import { RoomInventory, INVENTORY_CATEGORIES } from '@/types/inventory'
import RoomInventoryChecklist from '@/components/inventory/RoomInventoryChecklist'
import { useBranchFetch } from '@/lib/use-branch-fetch'

interface Room {
  id: number
  roomNumber: string
  roomType: string
  status: string
  inventoryCount: number
  lastChecked: string | null
  missingItems: number
}

export default function RoomInventoryPage() {
  const branchFetch = useBranchFetch()
  const [rooms, setRooms] = useState<Room[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [searchQuery, setSearchQuery] = useState('')
  const [debouncedSearch, setDebouncedSearch] = useState('')
  const [filterStatus, setFilterStatus] = useState<'all' | 'missing' | 'checked'>('all')

  // Checklist modal state
  const [showChecklist, setShowChecklist] = useState(false)
  const [selectedRoom, setSelectedRoom] = useState<{ id: number; number: string } | null>(null)

  // Debounce search input
  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedSearch(searchQuery)
    }, 300)

    return () => clearTimeout(timer)
  }, [searchQuery])

  // Fetch rooms with inventory
  const fetchRooms = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const params = new URLSearchParams()

      if (debouncedSearch) {
        params.append('search', debouncedSearch)
      }

      if (filterStatus !== 'all') {
        params.append('filter', filterStatus)
      }

      const response = await branchFetch(`/api/new/inventory/rooms?${params}`)
      if (!response.ok) {
        throw new Error('Failed to fetch rooms')
      }

      const data = await response.json()
      if (data.success) {
        setRooms(data.data || [])
      } else {
        throw new Error(data.message || 'Failed to fetch rooms')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการโหลดข้อมูล')
    } finally {
      setLoading(false)
    }
  }, [debouncedSearch, filterStatus, branchFetch])

  useEffect(() => {
    fetchRooms()
  }, [fetchRooms])

  // Handle open checklist
  const handleOpenChecklist = (room: Room) => {
    setSelectedRoom({ id: room.id, number: room.roomNumber })
    setShowChecklist(true)
  }

  // Handle checklist success
  const handleChecklistSuccess = () => {
    fetchRooms()
  }

  // Format date
  const formatDate = (dateString: string | null) => {
    if (!dateString) return 'ยังไม่เคยตรวจ'
    const date = new Date(dateString)
    return date.toLocaleDateString('th-TH', {
      day: 'numeric',
      month: 'short',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      timeZone: 'UTC',
    })
  }

  // Get room status color
  const getRoomStatusColor = (room: Room) => {
    if (room.missingItems > 0) {
      return 'border-amber-500/50 bg-amber-500/10'
    }
    if (room.lastChecked) {
      const lastCheckedDate = new Date(room.lastChecked)
      const daysSinceCheck = Math.floor((Date.now() - lastCheckedDate.getTime()) / (1000 * 60 * 60 * 24))
      if (daysSinceCheck < 1) {
        return 'border-emerald-500/50 bg-emerald-500/10'
      }
    }
    return 'border-gray-200 bg-white'
  }

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <DoorOpen className="w-8 h-8 text-red-600" />
          <div>
            <h1 className="text-2xl font-bold text-gray-900">สินค้าคงคลังตามห้อง</h1>
            <p className="text-gray-500">
              ตรวจสอบและจัดการสินค้าในแต่ละห้อง
            </p>
          </div>
        </div>
      </div>

      {/* Search and Filter Bar */}
      <div className="bg-white rounded-lg border border-gray-200 p-4">
        <div className="flex flex-col md:flex-row gap-4">
          {/* Search */}
          <div className="relative flex-1">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-500" />
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="ค้นหาเลขห้อง..."
              className="w-full pl-10 pr-4 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
            />
            {searchQuery && (
              <button
                onClick={() => setSearchQuery('')}
                className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-gray-500"
                aria-label="ล้างการค้นหา"
              >
                <X className="w-4 h-4" />
              </button>
            )}
          </div>

          {/* Filter Buttons */}
          <div className="flex items-center gap-2">
            <Filter className="w-5 h-5 text-gray-500" />
            <button
              onClick={() => setFilterStatus('all')}
              className={`px-3 py-2 text-sm rounded-lg border transition-colors ${
                filterStatus === 'all'
                  ? 'border-red-500 bg-red-500/10 text-red-600'
                  : 'border-gray-300 text-gray-500 hover:bg-gray-100'
              }`}
            >
              ทั้งหมด
            </button>
            <button
              onClick={() => setFilterStatus('missing')}
              className={`px-3 py-2 text-sm rounded-lg border transition-colors ${
                filterStatus === 'missing'
                  ? 'border-amber-500 bg-amber-500/10 text-amber-600'
                  : 'border-gray-300 text-gray-500 hover:bg-gray-100'
              }`}
            >
              มีสินค้าขาด
            </button>
            <button
              onClick={() => setFilterStatus('checked')}
              className={`px-3 py-2 text-sm rounded-lg border transition-colors ${
                filterStatus === 'checked'
                  ? 'border-emerald-500 bg-emerald-500/10 text-emerald-600'
                  : 'border-gray-300 text-gray-500 hover:bg-gray-100'
              }`}
            >
              ตรวจแล้ววันนี้
            </button>
          </div>
        </div>
      </div>

      {/* Error Message */}
      {error && (
        <div className="flex items-center gap-2 p-4 bg-red-50 border border-red-200 rounded-lg text-red-600">
          <AlertCircle className="w-5 h-5 shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {/* Room Grid */}
      {loading ? (
        <div className="flex items-center justify-center py-12">
          <Loader2 className="w-6 h-6 animate-spin text-red-500" />
          <span className="ml-2 text-gray-500">กำลังโหลดข้อมูล...</span>
        </div>
      ) : rooms.length === 0 ? (
        <div className="text-center py-12 text-gray-500 bg-white rounded-lg border border-gray-200">
          <DoorOpen className="w-12 h-12 text-gray-400 mx-auto mb-3" />
          <p>ไม่พบห้องที่ตรงกับเงื่อนไข</p>
        </div>
      ) : (
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
          {rooms.map((room) => (
            <div
              key={room.id}
              className={`rounded-lg border-2 p-4 transition-all hover:border-gray-300 cursor-pointer ${getRoomStatusColor(room)}`}
              onClick={() => handleOpenChecklist(room)}
            >
              {/* Room Number */}
              <div className="flex items-center justify-between mb-3">
                <h3 className="text-xl font-bold text-gray-900">{room.roomNumber}</h3>
                {room.missingItems > 0 ? (
                  <AlertTriangle className="w-5 h-5 text-amber-400" />
                ) : room.lastChecked ? (
                  <CheckCircle className="w-5 h-5 text-emerald-400" />
                ) : (
                  <ClipboardCheck className="w-5 h-5 text-gray-500" />
                )}
              </div>

              {/* Room Type */}
              <p className="text-sm text-gray-500 mb-2">{room.roomType}</p>

              {/* Inventory Info */}
              <div className="space-y-1 text-sm">
                <div className="flex items-center justify-between">
                  <span className="text-gray-500">สินค้า:</span>
                  <span className="font-medium text-gray-900">{room.inventoryCount} รายการ</span>
                </div>
                {room.missingItems > 0 && (
                  <div className="flex items-center justify-between text-amber-400">
                    <span>ขาด:</span>
                    <span className="font-medium">{room.missingItems} รายการ</span>
                  </div>
                )}
              </div>

              {/* Last Checked */}
              <div className="mt-3 pt-3 border-t border-gray-200">
                <p className="text-xs text-gray-500">
                  ตรวจล่าสุด: {formatDate(room.lastChecked)}
                </p>
              </div>

              {/* Actions */}
              <div className="mt-3 flex gap-2">
                <button
                  onClick={(e) => {
                    e.stopPropagation()
                    handleOpenChecklist(room)
                  }}
                  className="flex-1 flex items-center justify-center gap-1 px-3 py-2 bg-red-600 text-white text-sm rounded-lg hover:bg-red-700 transition-colors"
                >
                  <ClipboardCheck className="w-4 h-4" />
                  ตรวจสอบ
                </button>
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Legend */}
      <div className="bg-white rounded-lg border border-gray-200 p-4">
        <h3 className="font-medium text-gray-900 mb-3">คำอธิบายสัญลักษณ์</h3>
        <div className="flex flex-wrap gap-4">
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 rounded border-2 border-emerald-500/50 bg-emerald-500/10" />
            <span className="text-sm text-gray-500">ตรวจแล้ววันนี้</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 rounded border-2 border-amber-500/50 bg-amber-500/10" />
            <span className="text-sm text-gray-500">มีสินค้าขาด</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 rounded border-2 border-gray-200 bg-white" />
            <span className="text-sm text-gray-500">ยังไม่ได้ตรวจ</span>
          </div>
        </div>
      </div>

      {/* Room Inventory Checklist Modal */}
      {selectedRoom && (
        <RoomInventoryChecklist
          isOpen={showChecklist}
          onClose={() => {
            setShowChecklist(false)
            setSelectedRoom(null)
          }}
          roomId={selectedRoom.id}
          roomNumber={selectedRoom.number}
          onSuccess={handleChecklistSuccess}
        />
      )}
    </div>
  )
}
