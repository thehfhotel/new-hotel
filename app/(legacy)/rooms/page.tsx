'use client'

import { useState, useEffect, useCallback } from 'react'
import { format, parseISO } from 'date-fns'
import { th } from 'date-fns/locale'
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
  Clock,
  Wrench,
  Sparkles,
  CheckCircle,
  XCircle,
  History,
} from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'

// Types
interface Room {
  Room_No: string
  Room_Type: string
  Room_Status: string
  Room_Clean: boolean
  Room_Use: boolean
  Room_Manternace: boolean
  Room_PriceA?: number
  Room_PriceB?: number
  Room_PriceC?: number
  Room_Group?: string
  Room_Book_Name?: string
}

interface GuestInfo {
  name: string
  checkIn: string
  checkOut: string
}

interface RoomHistory {
  date: string
  action: string
  guestName?: string
}

interface RoomDetail {
  success?: boolean
  room?: Room & { currentGuest?: GuestInfo }
  currentGuest?: GuestInfo
  recentHistory?: RoomHistory[]
}

// Status configuration
const roomStatusConfig: Record<string, { label: string; bgColor: string; textColor: string; borderColor: string }> = {
  'Available': { label: 'ว่าง', bgColor: 'bg-green-50', textColor: 'text-green-700', borderColor: 'border-green-200' },
  'ว่าง': { label: 'ว่าง', bgColor: 'bg-green-50', textColor: 'text-green-700', borderColor: 'border-green-200' },
  'Occupied': { label: 'มีผู้เข้าพัก', bgColor: 'bg-blue-50', textColor: 'text-blue-700', borderColor: 'border-blue-200' },
  'มีผู้เข้าพัก': { label: 'มีผู้เข้าพัก', bgColor: 'bg-blue-50', textColor: 'text-blue-700', borderColor: 'border-blue-200' },
  'Maintenance': { label: 'ซ่อมบำรุง', bgColor: 'bg-orange-50', textColor: 'text-orange-700', borderColor: 'border-orange-200' },
  'ซ่อมบำรุง': { label: 'ซ่อมบำรุง', bgColor: 'bg-orange-50', textColor: 'text-orange-700', borderColor: 'border-orange-200' },
  'Needs Cleaning': { label: 'รอทำความสะอาด', bgColor: 'bg-yellow-50', textColor: 'text-yellow-700', borderColor: 'border-yellow-200' },
  'รอทำความสะอาด': { label: 'รอทำความสะอาด', bgColor: 'bg-yellow-50', textColor: 'text-yellow-700', borderColor: 'border-yellow-200' },
}

const statusFilterOptions = [
  { value: '', label: 'ทั้งหมด' },
  { value: 'Available', label: 'ว่าง' },
  { value: 'Occupied', label: 'มีผู้เข้าพัก' },
  { value: 'Maintenance', label: 'ซ่อมบำรุง' },
  { value: 'Needs Cleaning', label: 'รอทำความสะอาด' },
]

export default function RoomsPage() {
  // Branch-aware fetch
  const branchFetch = useBranchFetch()

  // State
  const [rooms, setRooms] = useState<Room[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  // View mode
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid')

  // Filters
  const [statusFilter, setStatusFilter] = useState('')
  const [typeFilter, setTypeFilter] = useState('')
  const [roomTypes, setRoomTypes] = useState<string[]>([])

  // Detail panel
  const [selectedRoom, setSelectedRoom] = useState<Room | null>(null)
  const [roomDetail, setRoomDetail] = useState<RoomDetail | null>(null)
  const [loadingDetail, setLoadingDetail] = useState(false)

  // Fetch rooms
  const fetchRooms = useCallback(async () => {
    setLoading(true)
    setError(null)

    try {
      const params = new URLSearchParams()
      if (statusFilter) params.append('status', statusFilter)
      if (typeFilter) params.append('type', typeFilter)

      const response = await branchFetch(`/api/rooms?${params.toString()}`)

      if (!response.ok) {
        throw new Error('ไม่สามารถดึงข้อมูลห้องพักได้')
      }

      const data = await response.json()
      const roomsData = data.data || data
      setRooms(roomsData)

      // Extract unique room types
      const types = [...new Set(roomsData.map((room: Room) => room.Room_Type))] as string[]
      setRoomTypes(types)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setLoading(false)
    }
  }, [statusFilter, typeFilter, branchFetch])

  useEffect(() => {
    fetchRooms()
  }, [fetchRooms])

  // Fetch room detail
  const fetchRoomDetail = useCallback(async (room: Room) => {
    setSelectedRoom(room)
    setLoadingDetail(true)

    try {
      const response = await branchFetch(`/api/rooms/${room.Room_No}`)

      if (response.ok) {
        const data = await response.json()
        // Map API response to RoomDetail structure
        if (data.success && data.room) {
          setRoomDetail({
            room: data.room,
            currentGuest: data.room.currentGuest,
            recentHistory: [],
          })
        } else {
          setRoomDetail({ room, recentHistory: [] })
        }
      } else {
        setRoomDetail({ room, recentHistory: [] })
      }
    } catch {
      setRoomDetail({ room, recentHistory: [] })
    } finally {
      setLoadingDetail(false)
    }
  }, [branchFetch])

  // Format date helper
  const formatDate = (dateString: string) => {
    try {
      const date = parseISO(dateString)
      return format(date, 'd MMM yyyy', { locale: th })
    } catch {
      return dateString
    }
  }

  // Get status config
  const getStatusConfig = (status: string) => {
    return roomStatusConfig[status] || {
      label: status,
      bgColor: 'bg-gray-50',
      textColor: 'text-gray-700',
      borderColor: 'border-gray-200'
    }
  }

  // Get status icon
  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'Available':
      case 'ว่าง':
        return <CheckCircle className="h-4 w-4 text-green-500" />
      case 'Occupied':
      case 'มีผู้เข้าพัก':
        return <User className="h-4 w-4 text-blue-500" />
      case 'Maintenance':
      case 'ซ่อมบำรุง':
        return <Wrench className="h-4 w-4 text-orange-500" />
      case 'Needs Cleaning':
      case 'รอทำความสะอาด':
        return <Sparkles className="h-4 w-4 text-yellow-500" />
      default:
        return <BedDouble className="h-4 w-4 text-gray-500" />
    }
  }

  // Handle filter reset
  const handleResetFilters = () => {
    setStatusFilter('')
    setTypeFilter('')
  }

  // Room Card Component
  const RoomCard = ({ room }: { room: Room }) => {
    const config = getStatusConfig(room.Room_Status)

    return (
      <div
        onClick={() => fetchRoomDetail(room)}
        className={`p-4 rounded-lg border-2 cursor-pointer hover:shadow-md ${config.bgColor} ${config.borderColor} ${
          selectedRoom?.Room_No === room.Room_No ? 'ring-2 ring-blue-500' : ''
        }`}
      >
        <div className="flex items-center justify-between mb-2">
          <span className="text-lg font-bold text-gray-800">{room.Room_No}</span>
          {getStatusIcon(room.Room_Status)}
        </div>

        <p className="text-sm text-gray-600 mb-2">{room.Room_Type}</p>

        <div className={`inline-flex items-center px-2 py-1 rounded text-xs font-medium ${config.bgColor} ${config.textColor}`}>
          {config.label}
        </div>

        {/* Flag badges */}
        <div className="flex flex-wrap gap-1 mt-2">
          {room.Room_Clean && (
            <span className="inline-flex items-center px-1.5 py-0.5 rounded text-xs bg-emerald-100 text-emerald-700">
              <Sparkles className="h-3 w-3 mr-1" />
              สะอาด
            </span>
          )}
          {room.Room_Use && (
            <span className="inline-flex items-center px-1.5 py-0.5 rounded text-xs bg-blue-100 text-blue-700">
              <User className="h-3 w-3 mr-1" />
              ใช้งาน
            </span>
          )}
          {room.Room_Manternace && (
            <span className="inline-flex items-center px-1.5 py-0.5 rounded text-xs bg-orange-100 text-orange-700">
              <Wrench className="h-3 w-3 mr-1" />
              ซ่อม
            </span>
          )}
        </div>
      </div>
    )
  }

  // Room List Row Component
  const RoomListRow = ({ room }: { room: Room }) => {
    const config = getStatusConfig(room.Room_Status)

    return (
      <tr
        onClick={() => fetchRoomDetail(room)}
        className={`cursor-pointer hover:bg-gray-50 ${
          selectedRoom?.Room_No === room.Room_No ? 'bg-blue-50' : ''
        }`}
      >
        <td className="px-6 py-4 whitespace-nowrap">
          <div className="flex items-center">
            {getStatusIcon(room.Room_Status)}
            <span className="ml-2 font-medium text-gray-900">{room.Room_No}</span>
          </div>
        </td>
        <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
          {room.Room_Type}
        </td>
        <td className="px-6 py-4 whitespace-nowrap">
          <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${config.bgColor} ${config.textColor}`}>
            {config.label}
          </span>
        </td>
        <td className="px-6 py-4 whitespace-nowrap">
          <div className="flex gap-1">
            {room.Room_Clean ? (
              <span title="สะอาด"><CheckCircle className="h-5 w-5 text-emerald-500" /></span>
            ) : (
              <span title="ไม่สะอาด"><XCircle className="h-5 w-5 text-gray-300" /></span>
            )}
          </div>
        </td>
        <td className="px-6 py-4 whitespace-nowrap">
          <div className="flex gap-1">
            {room.Room_Use ? (
              <span title="ใช้งาน"><CheckCircle className="h-5 w-5 text-blue-500" /></span>
            ) : (
              <span title="ไม่ใช้งาน"><XCircle className="h-5 w-5 text-gray-300" /></span>
            )}
          </div>
        </td>
        <td className="px-6 py-4 whitespace-nowrap">
          <div className="flex gap-1">
            {room.Room_Manternace ? (
              <span title="ซ่อมบำรุง"><CheckCircle className="h-5 w-5 text-orange-500" /></span>
            ) : (
              <span title="ไม่ซ่อม"><XCircle className="h-5 w-5 text-gray-300" /></span>
            )}
          </div>
        </td>
      </tr>
    )
  }

  return (
    <div className="flex gap-6">
      {/* Main Content */}
      <div className={`flex-1 space-y-6 ${selectedRoom ? 'lg:mr-80' : ''}`}>
        {/* Header */}
        <div className="flex items-center justify-between">
          <h1 className="text-2xl font-bold text-gray-900">จัดการห้องพัก</h1>
          <div className="flex items-center space-x-2">
            {/* View Toggle */}
            <div className="flex items-center bg-gray-100 rounded-lg p-1">
              <button
                onClick={() => setViewMode('grid')}
                className={`p-2 rounded ${viewMode === 'grid' ? 'bg-white shadow' : 'text-gray-500 hover:text-gray-700'}`}
                title="มุมมองตาราง"
              >
                <Grid3X3 className="h-5 w-5" />
              </button>
              <button
                onClick={() => setViewMode('list')}
                className={`p-2 rounded ${viewMode === 'list' ? 'bg-white shadow' : 'text-gray-500 hover:text-gray-700'}`}
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
        <div className="bg-white rounded-lg shadow p-4">
          <div className="flex items-center space-x-2 mb-4">
            <Filter className="h-5 w-5 text-gray-500" />
            <span className="font-medium text-gray-700">ตัวกรอง</span>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {/* Room Type Filter */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">ประเภทห้อง</label>
              <select
                value={typeFilter}
                onChange={(e) => setTypeFilter(e.target.value)}
                className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              >
                <option value="">ทั้งหมด</option>
                {roomTypes.map((type) => (
                  <option key={type} value={type}>
                    {type}
                  </option>
                ))}
              </select>
            </div>

            {/* Status Filter */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">สถานะ</label>
              <select
                value={statusFilter}
                onChange={(e) => setStatusFilter(e.target.value)}
                className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              >
                {statusFilterOptions.map((option) => (
                  <option key={option.value} value={option.value}>
                    {option.label}
                  </option>
                ))}
              </select>
            </div>

            {/* Reset Button */}
            <div className="flex items-end">
              <button
                onClick={handleResetFilters}
                className="w-full px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50 transition-colors"
              >
                ล้างตัวกรอง
              </button>
            </div>
          </div>
        </div>

        {/* Room Count */}
        <div className="text-sm text-gray-600">
          พบ {rooms.length} ห้อง
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
        ) : rooms.length === 0 ? (
          <div className="text-center py-12 text-gray-500">
            ไม่พบข้อมูลห้องพัก
          </div>
        ) : viewMode === 'grid' ? (
          /* Grid View */
          <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
            {rooms.map((room) => (
              <RoomCard key={room.Room_No} room={room} />
            ))}
          </div>
        ) : (
          /* List View */
          <div className="bg-white rounded-lg shadow overflow-hidden">
            <div className="overflow-x-auto">
              <table className="min-w-full divide-y divide-gray-200">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      เลขห้อง
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      ประเภท
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      สถานะ
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      สะอาด
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      ใช้งาน
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      ซ่อมบำรุง
                    </th>
                  </tr>
                </thead>
                <tbody className="bg-white divide-y divide-gray-200">
                  {rooms.map((room) => (
                    <RoomListRow key={room.Room_No} room={room} />
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
            {/* Panel Header */}
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-lg font-bold text-gray-900">รายละเอียดห้อง</h2>
              <button
                onClick={() => {
                  setSelectedRoom(null)
                  setRoomDetail(null)
                }}
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
                    <span className="text-2xl font-bold text-gray-900">{selectedRoom.Room_No}</span>
                    {getStatusIcon(selectedRoom.Room_Status)}
                  </div>

                  <div className="space-y-2">
                    <div className="flex justify-between">
                      <span className="text-sm text-gray-600">ประเภท:</span>
                      <span className="text-sm font-medium">{selectedRoom.Room_Type}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-sm text-gray-600">สถานะ:</span>
                      <span className={`text-sm font-medium ${getStatusConfig(selectedRoom.Room_Status).textColor}`}>
                        {getStatusConfig(selectedRoom.Room_Status).label}
                      </span>
                    </div>
                    {selectedRoom.Room_Group && (
                      <div className="flex justify-between">
                        <span className="text-sm text-gray-600">กลุ่ม:</span>
                        <span className="text-sm font-medium">{selectedRoom.Room_Group}</span>
                      </div>
                    )}
                    {selectedRoom.Room_Book_Name && (
                      <div className="flex justify-between">
                        <span className="text-sm text-gray-600">ผู้จอง:</span>
                        <span className="text-sm font-medium">{selectedRoom.Room_Book_Name}</span>
                      </div>
                    )}
                    {(selectedRoom.Room_PriceA || selectedRoom.Room_PriceB || selectedRoom.Room_PriceC) && (
                      <div className="mt-2 pt-2 border-t border-gray-200">
                        <span className="text-sm text-gray-600 font-medium">ราคาห้อง:</span>
                        <div className="mt-1 space-y-1">
                          {selectedRoom.Room_PriceA && (
                            <div className="flex justify-between">
                              <span className="text-xs text-gray-500">ราคา A:</span>
                              <span className="text-sm font-medium">{selectedRoom.Room_PriceA.toLocaleString()} บาท</span>
                            </div>
                          )}
                          {selectedRoom.Room_PriceB && (
                            <div className="flex justify-between">
                              <span className="text-xs text-gray-500">ราคา B:</span>
                              <span className="text-sm font-medium">{selectedRoom.Room_PriceB.toLocaleString()} บาท</span>
                            </div>
                          )}
                          {selectedRoom.Room_PriceC && (
                            <div className="flex justify-between">
                              <span className="text-xs text-gray-500">ราคา C:</span>
                              <span className="text-sm font-medium">{selectedRoom.Room_PriceC.toLocaleString()} บาท</span>
                            </div>
                          )}
                        </div>
                      </div>
                    )}
                  </div>
                </div>

                {/* Flags */}
                <div>
                  <h3 className="text-sm font-medium text-gray-700 mb-2">สถานะห้อง</h3>
                  <div className="space-y-2">
                    <div className={`flex items-center justify-between p-2 rounded ${selectedRoom.Room_Clean ? 'bg-emerald-50' : 'bg-gray-50'}`}>
                      <div className="flex items-center">
                        <Sparkles className={`h-4 w-4 mr-2 ${selectedRoom.Room_Clean ? 'text-emerald-500' : 'text-gray-400'}`} />
                        <span className="text-sm">ความสะอาด</span>
                      </div>
                      {selectedRoom.Room_Clean ? (
                        <CheckCircle className="h-5 w-5 text-emerald-500" />
                      ) : (
                        <XCircle className="h-5 w-5 text-gray-400" />
                      )}
                    </div>

                    <div className={`flex items-center justify-between p-2 rounded ${selectedRoom.Room_Use ? 'bg-blue-50' : 'bg-gray-50'}`}>
                      <div className="flex items-center">
                        <User className={`h-4 w-4 mr-2 ${selectedRoom.Room_Use ? 'text-blue-500' : 'text-gray-400'}`} />
                        <span className="text-sm">การใช้งาน</span>
                      </div>
                      {selectedRoom.Room_Use ? (
                        <CheckCircle className="h-5 w-5 text-blue-500" />
                      ) : (
                        <XCircle className="h-5 w-5 text-gray-400" />
                      )}
                    </div>

                    <div className={`flex items-center justify-between p-2 rounded ${selectedRoom.Room_Manternace ? 'bg-orange-50' : 'bg-gray-50'}`}>
                      <div className="flex items-center">
                        <Wrench className={`h-4 w-4 mr-2 ${selectedRoom.Room_Manternace ? 'text-orange-500' : 'text-gray-400'}`} />
                        <span className="text-sm">ซ่อมบำรุง</span>
                      </div>
                      {selectedRoom.Room_Manternace ? (
                        <CheckCircle className="h-5 w-5 text-orange-500" />
                      ) : (
                        <XCircle className="h-5 w-5 text-gray-400" />
                      )}
                    </div>
                  </div>
                </div>

                {/* Current Guest Info */}
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

                {/* Recent History */}
                {roomDetail?.recentHistory && roomDetail.recentHistory.length > 0 && (
                  <div>
                    <h3 className="text-sm font-medium text-gray-700 mb-2 flex items-center">
                      <History className="h-4 w-4 mr-1" />
                      ประวัติล่าสุด
                    </h3>
                    <div className="space-y-2">
                      {roomDetail.recentHistory.map((history, index) => (
                        <div key={index} className="flex items-start text-sm">
                          <Clock className="h-4 w-4 mr-2 mt-0.5 text-gray-400" />
                          <div>
                            <div className="text-gray-900">{history.action}</div>
                            <div className="text-gray-500 text-xs">
                              {formatDate(history.date)}
                              {history.guestName && ` - ${history.guestName}`}
                            </div>
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                )}

                {/* No history message */}
                {(!roomDetail?.recentHistory || roomDetail.recentHistory.length === 0) && (
                  <div className="text-center text-gray-500 text-sm py-4">
                    ไม่มีประวัติล่าสุด
                  </div>
                )}
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  )
}
