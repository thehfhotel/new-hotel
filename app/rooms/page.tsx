'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  Grid3X3,
  List,
  Filter,
  Loader2,
  AlertCircle,
  RefreshCw,
  X,
  BedDouble,
  User,
  Wrench,
  Sparkles,
  CheckCircle,
  XCircle,
  Clock,
  LogIn,
  LogOut,
  ArrowRightLeft,
  CalendarPlus,
} from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import CheckInModal from '@/components/CheckInModal'
import CheckOutModal from '@/components/CheckOutModal'
import ChangeRoomModal from '@/components/ChangeRoomModal'
import ExtendStayModal from '@/components/ExtendStayModal'

// API response types (from /api/new/rooms)
interface RoomApiItem {
  id: number
  roomNo: string
  roomTypeName: string | null
  floor: number | null
  status: string
  isClean: boolean
  isMaintenance: boolean
  notes: string | null
  updatedAt: string | null
  currentGuest?: {
    name: string
    checkIn: string
    checkOut: string
  }
}

// Status configuration
const roomStatusConfig: Record<string, { label: string; bgColor: string; textColor: string; borderColor: string }> = {
  available: { label: 'ว่าง', bgColor: 'bg-green-50', textColor: 'text-green-700', borderColor: 'border-green-200' },
  occupied: { label: 'มีผู้เข้าพัก', bgColor: 'bg-blue-50', textColor: 'text-blue-700', borderColor: 'border-blue-200' },
  maintenance: { label: 'ซ่อมบำรุง', bgColor: 'bg-orange-50', textColor: 'text-orange-700', borderColor: 'border-orange-200' },
  dirty: { label: 'รอทำความสะอาด', bgColor: 'bg-yellow-50', textColor: 'text-yellow-700', borderColor: 'border-yellow-200' },
  cleaning: { label: 'กำลังทำความสะอาด', bgColor: 'bg-amber-50', textColor: 'text-amber-700', borderColor: 'border-amber-200' },
  reserved: { label: 'จองแล้ว', bgColor: 'bg-purple-50', textColor: 'text-purple-700', borderColor: 'border-purple-200' },
}

const statusFilterOptions = [
  { value: '', label: 'ทั้งหมด' },
  { value: 'available', label: 'ว่าง' },
  { value: 'occupied', label: 'มีผู้เข้าพัก' },
  { value: 'maintenance', label: 'ซ่อมบำรุง' },
  { value: 'dirty', label: 'รอทำความสะอาด' },
  { value: 'cleaning', label: 'กำลังทำความสะอาด' },
]

export default function RoomsPage() {
  const branchFetch = useBranchFetch()

  const [rooms, setRooms] = useState<RoomApiItem[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid')
  const [statusFilter, setStatusFilter] = useState('')
  const [typeFilter, setTypeFilter] = useState('')
  const [roomTypes, setRoomTypes] = useState<string[]>([])
  const [selectedRoom, setSelectedRoom] = useState<RoomApiItem | null>(null)
  const [roomDetail, setRoomDetail] = useState<RoomApiItem | null>(null)
  const [loadingDetail, setLoadingDetail] = useState(false)
  const [showCheckIn, setShowCheckIn] = useState(false)
  const [showCheckOut, setShowCheckOut] = useState(false)
  // Track G1 / T4 HIGH-2: extend-stay modal (one-more-night flow).
  const [showExtendStay, setShowExtendStay] = useState(false)
  // Track G4 / T4 HIGH-3: change-room modal (mid-stay move).
  const [showChangeRoom, setShowChangeRoom] = useState(false)

  const fetchRooms = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const response = await branchFetch('/api/new/rooms?limit=200')
      if (!response.ok) throw new Error('ไม่สามารถดึงข้อมูลห้องพักได้')
      const data = await response.json()
      const roomsData: RoomApiItem[] = data.data || data || []
      setRooms(roomsData)
      const types = [...new Set(roomsData.map((r) => r.roomTypeName).filter(Boolean))] as string[]
      setRoomTypes(types)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setLoading(false)
    }
  }, [branchFetch])

  useEffect(() => {
    fetchRooms()
  }, [fetchRooms])

  const fetchRoomDetail = useCallback(async (room: RoomApiItem) => {
    setSelectedRoom(room)
    setLoadingDetail(true)
    try {
      const response = await branchFetch(`/api/new/rooms/${room.id}`)
      if (response.ok) {
        const data = await response.json()
        setRoomDetail(data.data || data)
      } else {
        setRoomDetail(room)
      }
    } catch {
      setRoomDetail(room)
    } finally {
      setLoadingDetail(false)
    }
  }, [branchFetch])

  // Filtered rooms
  const filteredRooms = rooms.filter((room) => {
    if (statusFilter && room.status !== statusFilter) return false
    if (typeFilter && room.roomTypeName !== typeFilter) return false
    return true
  })

  const getStatusConfig = (status: string) => {
    return roomStatusConfig[status] || {
      label: status,
      bgColor: 'bg-gray-50',
      textColor: 'text-gray-700',
      borderColor: 'border-gray-200',
    }
  }

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'available':
        return <CheckCircle className="h-4 w-4 text-green-500" />
      case 'occupied':
        return <User className="h-4 w-4 text-blue-500" />
      case 'maintenance':
        return <Wrench className="h-4 w-4 text-orange-500" />
      case 'dirty':
      case 'cleaning':
        return <Sparkles className="h-4 w-4 text-yellow-500" />
      default:
        return <BedDouble className="h-4 w-4 text-gray-500" />
    }
  }

  const formatDate = (dateString: string) => {
    try {
      const date = new Date(dateString)
      return date.toLocaleDateString('th-TH', {
        day: 'numeric',
        month: 'short',
        year: 'numeric',
        timeZone: 'UTC',
      })
    } catch {
      return dateString
    }
  }

  const RoomCard = ({ room }: { room: RoomApiItem }) => {
    const config = getStatusConfig(room.status)
    return (
      <div
        onClick={() => fetchRoomDetail(room)}
        className={`p-4 rounded-lg border-2 cursor-pointer hover:shadow-md ${config.bgColor} ${config.borderColor} ${
          selectedRoom?.id === room.id ? 'ring-2 ring-blue-500' : ''
        }`}
      >
        <div className="flex items-center justify-between mb-2">
          <span className="text-lg font-bold text-gray-800">{room.roomNo}</span>
          {getStatusIcon(room.status)}
        </div>
        <p className="text-sm text-gray-600 mb-2">{room.roomTypeName || '-'}</p>
        <div className={`inline-flex items-center px-2 py-1 rounded text-xs font-medium ${config.bgColor} ${config.textColor}`}>
          {config.label}
        </div>
        <div className="flex flex-wrap gap-1 mt-2">
          {room.isClean && (
            <span className="inline-flex items-center px-1.5 py-0.5 rounded text-xs bg-emerald-100 text-emerald-700">
              <Sparkles className="h-3 w-3 mr-1" />
              สะอาด
            </span>
          )}
          {room.isMaintenance && (
            <span className="inline-flex items-center px-1.5 py-0.5 rounded text-xs bg-orange-100 text-orange-700">
              <Wrench className="h-3 w-3 mr-1" />
              ซ่อม
            </span>
          )}
        </div>
      </div>
    )
  }

  const RoomListRow = ({ room }: { room: RoomApiItem }) => {
    const config = getStatusConfig(room.status)
    return (
      <tr
        onClick={() => fetchRoomDetail(room)}
        className={`cursor-pointer hover:bg-gray-50 ${selectedRoom?.id === room.id ? 'bg-blue-50' : ''}`}
      >
        <td className="px-6 py-4 whitespace-nowrap">
          <div className="flex items-center">
            {getStatusIcon(room.status)}
            <span className="ml-2 font-medium text-gray-900">{room.roomNo}</span>
          </div>
        </td>
        <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
          {room.roomTypeName || '-'}
        </td>
        <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
          {room.floor ?? '-'}
        </td>
        <td className="px-6 py-4 whitespace-nowrap">
          <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${config.bgColor} ${config.textColor}`}>
            {config.label}
          </span>
        </td>
        <td className="px-6 py-4 whitespace-nowrap">
          {room.isClean ? (
            <CheckCircle className="h-5 w-5 text-emerald-500" />
          ) : (
            <XCircle className="h-5 w-5 text-gray-300" />
          )}
        </td>
        <td className="px-6 py-4 whitespace-nowrap">
          {room.isMaintenance ? (
            <CheckCircle className="h-5 w-5 text-orange-500" />
          ) : (
            <XCircle className="h-5 w-5 text-gray-300" />
          )}
        </td>
      </tr>
    )
  }

  return (
    <div className="flex gap-6">
      {/* Main Content */}
      <div className={`flex-1 space-y-6 ${selectedRoom ? 'lg:mr-80' : ''}`}>
        {/* Page header bar */}
        <div className="flex items-center justify-between bg-panel border border-border h-10 px-3">
          <h1 className="text-base font-semibold text-text">จัดการห้องพัก</h1>
          <div className="flex items-center space-x-2">
            <div className="flex items-center bg-gray-100 rounded-lg p-1">
              <button
                onClick={() => setViewMode('grid')}
                className={`p-2 rounded ${viewMode === 'grid' ? 'bg-white shadow-sm' : 'text-gray-500 hover:text-gray-700'}`}
                title="มุมมองตาราง"
              >
                <Grid3X3 className="h-5 w-5" />
              </button>
              <button
                onClick={() => setViewMode('list')}
                className={`p-2 rounded ${viewMode === 'list' ? 'bg-white shadow-sm' : 'text-gray-500 hover:text-gray-700'}`}
                title="มุมมองรายการ"
              >
                <List className="h-5 w-5" />
              </button>
            </div>
            <button
              onClick={fetchRooms}
              className="flex items-center space-x-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
            >
              <RefreshCw className="h-4 w-4" />
              <span>รีเฟรช</span>
            </button>
          </div>
        </div>

        {/* Filters */}
        <div className="bg-white rounded-lg shadow-sm p-4">
          <div className="flex items-center space-x-2 mb-4">
            <Filter className="h-5 w-5 text-gray-500" />
            <span className="font-medium text-gray-700">ตัวกรอง</span>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">ประเภทห้อง</label>
              <select
                value={typeFilter}
                onChange={(e) => setTypeFilter(e.target.value)}
                className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              >
                <option value="">ทั้งหมด</option>
                {roomTypes.map((type) => (
                  <option key={type} value={type}>{type}</option>
                ))}
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">สถานะ</label>
              <select
                value={statusFilter}
                onChange={(e) => setStatusFilter(e.target.value)}
                className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              >
                {statusFilterOptions.map((option) => (
                  <option key={option.value} value={option.value}>{option.label}</option>
                ))}
              </select>
            </div>
            <div className="flex items-end">
              <button
                onClick={() => { setStatusFilter(''); setTypeFilter('') }}
                className="w-full px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50 transition-colors"
              >
                ล้างตัวกรอง
              </button>
            </div>
          </div>
        </div>

        {/* Room Count */}
        <div className="text-sm text-gray-600">
          พบ {filteredRooms.length} ห้อง
        </div>

        {/* Content */}
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
        ) : filteredRooms.length === 0 ? (
          <div className="text-center py-12 text-gray-500">ไม่พบข้อมูลห้องพัก</div>
        ) : viewMode === 'grid' ? (
          <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
            {filteredRooms.map((room) => (
              <RoomCard key={room.id} room={room} />
            ))}
          </div>
        ) : (
          <div className="bg-white rounded-lg shadow-sm overflow-hidden">
            <div className="overflow-x-auto">
              <table className="min-w-full divide-y divide-gray-200">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">เลขห้อง</th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">ประเภท</th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">ชั้น</th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">สถานะ</th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">สะอาด</th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">ซ่อมบำรุง</th>
                  </tr>
                </thead>
                <tbody className="bg-white divide-y divide-gray-200">
                  {filteredRooms.map((room) => (
                    <RoomListRow key={room.id} room={room} />
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        )}
      </div>

      {/* Detail Panel */}
      {selectedRoom && (
        <div className="fixed right-0 top-0 h-full w-80 bg-white shadow-xl border-l border-gray-200 overflow-y-auto z-50">
          <div className="p-4">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-lg font-bold text-gray-900">รายละเอียดห้อง</h2>
              <button
                onClick={() => { setSelectedRoom(null); setRoomDetail(null) }}
                className="p-1 hover:bg-gray-100 rounded"
              >
                <X className="h-5 w-5 text-gray-500" />
              </button>
            </div>

            {loadingDetail ? (
              <div className="flex items-center justify-center py-8">
                <Loader2 className="h-6 w-6 animate-spin text-blue-600" />
              </div>
            ) : (
              <div className="space-y-6">
                {/* Room Info */}
                <div className="bg-gray-50 rounded-lg p-4">
                  <div className="flex items-center justify-between mb-3">
                    <span className="text-2xl font-bold text-gray-900">{selectedRoom.roomNo}</span>
                    {getStatusIcon(selectedRoom.status)}
                  </div>
                  <div className="space-y-2">
                    <div className="flex justify-between">
                      <span className="text-sm text-gray-600">ประเภท:</span>
                      <span className="text-sm font-medium">{selectedRoom.roomTypeName || '-'}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-sm text-gray-600">ชั้น:</span>
                      <span className="text-sm font-medium">{selectedRoom.floor ?? '-'}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-sm text-gray-600">สถานะ:</span>
                      <span className={`text-sm font-medium ${getStatusConfig(selectedRoom.status).textColor}`}>
                        {getStatusConfig(selectedRoom.status).label}
                      </span>
                    </div>
                    {selectedRoom.notes && (
                      <div className="mt-2 pt-2 border-t border-gray-200">
                        <span className="text-sm text-gray-600">หมายเหตุ:</span>
                        <p className="text-sm mt-1">{selectedRoom.notes}</p>
                      </div>
                    )}
                  </div>
                </div>

                {/* Flags */}
                <div>
                  <h3 className="text-sm font-medium text-gray-700 mb-2">สถานะห้อง</h3>
                  <div className="space-y-2">
                    <div className={`flex items-center justify-between p-2 rounded ${selectedRoom.isClean ? 'bg-emerald-50' : 'bg-gray-50'}`}>
                      <div className="flex items-center">
                        <Sparkles className={`h-4 w-4 mr-2 ${selectedRoom.isClean ? 'text-emerald-500' : 'text-gray-400'}`} />
                        <span className="text-sm">ความสะอาด</span>
                      </div>
                      {selectedRoom.isClean ? (
                        <CheckCircle className="h-5 w-5 text-emerald-500" />
                      ) : (
                        <XCircle className="h-5 w-5 text-gray-400" />
                      )}
                    </div>
                    <div className={`flex items-center justify-between p-2 rounded ${selectedRoom.isMaintenance ? 'bg-orange-50' : 'bg-gray-50'}`}>
                      <div className="flex items-center">
                        <Wrench className={`h-4 w-4 mr-2 ${selectedRoom.isMaintenance ? 'text-orange-500' : 'text-gray-400'}`} />
                        <span className="text-sm">ซ่อมบำรุง</span>
                      </div>
                      {selectedRoom.isMaintenance ? (
                        <CheckCircle className="h-5 w-5 text-orange-500" />
                      ) : (
                        <XCircle className="h-5 w-5 text-gray-400" />
                      )}
                    </div>
                  </div>
                </div>

                {/* Current Guest */}
                {roomDetail?.currentGuest && (
                  <div>
                    <h3 className="text-sm font-medium text-gray-700 mb-2 flex items-center">
                      <User className="h-4 w-4 mr-1" />
                      ผู้เข้าพักปัจจุบัน
                    </h3>
                    <div className="bg-blue-50 rounded-lg p-3 space-y-2">
                      <div className="flex justify-between">
                        <span className="text-sm text-gray-600">ชื่อ:</span>
                        <span className="text-sm font-medium">{roomDetail.currentGuest.name}</span>
                      </div>
                      <div className="flex justify-between">
                        <span className="text-sm text-gray-600">วันเข้าพัก:</span>
                        <span className="text-sm font-medium">{formatDate(roomDetail.currentGuest.checkIn)}</span>
                      </div>
                      <div className="flex justify-between">
                        <span className="text-sm text-gray-600">วันออก:</span>
                        <span className="text-sm font-medium">{formatDate(roomDetail.currentGuest.checkOut)}</span>
                      </div>
                    </div>
                  </div>
                )}

                {/* Action buttons — check-in / check-out depending on status.
                    Maintenance / dirty rooms get neither (legacy app's policy:
                    can't check in to a dirty/under-repair room). */}
                <div className="flex gap-2 pt-2 border-t border-gray-200">
                  {selectedRoom.status === 'available' && (
                    <button
                      onClick={() => setShowCheckIn(true)}
                      className="flex-1 flex items-center justify-center gap-2 px-3 py-2 bg-red-600 text-white text-sm font-medium rounded hover:bg-red-700"
                    >
                      <LogIn size={14} />
                      เช็คอิน
                    </button>
                  )}
                  {selectedRoom.status === 'occupied' && (
                    <>
                      {/* Track G1 / T4 HIGH-2: extend-stay button sits
                          alongside check-out so receptionists see both
                          options the moment a guest asks for either. */}
                      <button
                        onClick={() => setShowExtendStay(true)}
                        className="flex-1 flex items-center justify-center gap-2 px-3 py-2 bg-emerald-600 text-white text-sm font-medium rounded hover:bg-emerald-700"
                      >
                        <CalendarPlus size={14} />
                        ขยายเวลาเข้าพัก
                      </button>
                      {/* Track G4 / T4 HIGH-3: change-room (mid-stay move). */}
                      <button
                        onClick={() => setShowChangeRoom(true)}
                        className="flex-1 flex items-center justify-center gap-2 px-3 py-2 bg-amber-600 text-white text-sm font-medium rounded hover:bg-amber-700"
                      >
                        <ArrowRightLeft size={14} />
                        เปลี่ยนห้อง
                      </button>
                      <button
                        onClick={() => setShowCheckOut(true)}
                        className="flex-1 flex items-center justify-center gap-2 px-3 py-2 bg-sky-600 text-white text-sm font-medium rounded hover:bg-sky-700"
                      >
                        <LogOut size={14} />
                        เช็คเอ้าท์
                      </button>
                    </>
                  )}
                </div>

                {/* Updated at */}
                {selectedRoom.updatedAt && (
                  <div className="text-xs text-gray-400 flex items-center">
                    <Clock className="h-3 w-3 mr-1" />
                    อัปเดตล่าสุด: {formatDate(selectedRoom.updatedAt)}
                  </div>
                )}
              </div>
            )}
          </div>
        </div>
      )}

      {/* Check-in / check-out modals */}
      {showCheckIn && selectedRoom && (
        <CheckInModal
          room={{
            id: selectedRoom.id,
            roomNo: selectedRoom.roomNo,
            roomTypeName: selectedRoom.roomTypeName,
          }}
          onClose={() => setShowCheckIn(false)}
          onSuccess={() => {
            fetchRooms()
            if (selectedRoom) fetchRoomDetail(selectedRoom)
          }}
        />
      )}
      {showCheckOut && selectedRoom && (
        <CheckOutModal
          room={{ id: selectedRoom.id, roomNo: selectedRoom.roomNo }}
          onClose={() => setShowCheckOut(false)}
          onSuccess={() => {
            fetchRooms()
            if (selectedRoom) fetchRoomDetail(selectedRoom)
          }}
        />
      )}
      {showExtendStay && selectedRoom && (
        <ExtendStayModal
          room={{ id: selectedRoom.id, roomNo: selectedRoom.roomNo }}
          onClose={() => setShowExtendStay(false)}
          onSuccess={() => {
            fetchRooms()
            if (selectedRoom) fetchRoomDetail(selectedRoom)
          }}
        />
      )}
      {showChangeRoom && selectedRoom && (
        <ChangeRoomModal
          room={{ id: selectedRoom.id, roomNo: selectedRoom.roomNo }}
          onClose={() => setShowChangeRoom(false)}
          onSuccess={() => {
            fetchRooms()
            if (selectedRoom) fetchRoomDetail(selectedRoom)
          }}
        />
      )}
    </div>
  )
}
