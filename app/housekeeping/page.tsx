'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  RefreshCw,
  Filter,
  Loader2,
  AlertCircle,
  Sparkles,
} from 'lucide-react'
import HousekeepingStats, {
  HousekeepingStatsData,
} from '@/components/housekeeping/HousekeepingStats'
import RoomCleaningCard, {
  HousekeepingRoom,
  HousekeepingStatus,
} from '@/components/housekeeping/RoomCleaningCard'
import { useBranchFetch } from '@/lib/use-branch-fetch'

// API response type
interface RoomsResponse {
  success: boolean
  data: Array<{
    id: number
    roomNo: string
    roomTypeName: string | null
    floor: number | null
    status: string
    isClean: boolean
    isMaintenance: boolean
    notes: string | null
    updatedAt: string | null
  }>
}

// Map API status to housekeeping status
function mapToHousekeepingStatus(
  status: string,
  isClean: boolean
): HousekeepingStatus {
  if (status === 'cleaning') return 'cleaning'
  if (!isClean || status === 'dirty') return 'dirty'
  return 'available'
}

// Column configuration
const columns = [
  {
    id: 'dirty' as HousekeepingStatus,
    title: 'ห้องสกปรก',
    subtitle: 'รอทำความสะอาด',
    bgColor: 'bg-red-500/5',
    borderColor: 'border-red-900/30',
    headerColor: 'bg-red-600',
  },
  {
    id: 'cleaning' as HousekeepingStatus,
    title: 'กำลังทำความสะอาด',
    subtitle: 'กำลังดำเนินการ',
    bgColor: 'bg-amber-500/5',
    borderColor: 'border-amber-900/30',
    headerColor: 'bg-amber-600',
  },
  {
    id: 'available' as HousekeepingStatus,
    title: 'พร้อมใช้งาน',
    subtitle: 'ห้องสะอาด',
    bgColor: 'bg-emerald-500/5',
    borderColor: 'border-emerald-900/30',
    headerColor: 'bg-emerald-600',
  },
]

export default function HousekeepingPage() {
  const branchFetch = useBranchFetch()
  const [rooms, setRooms] = useState<HousekeepingRoom[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [floorFilter, setFloorFilter] = useState<number | null>(null)
  const [lastRefresh, setLastRefresh] = useState<Date>(new Date())

  // Get unique floors from rooms
  const floors = [...new Set(rooms.map((r) => r.floor).filter((f): f is number => f !== null))].sort(
    (a, b) => a - b
  )

  // Calculate stats
  const stats: HousekeepingStatsData = {
    dirtyCount: rooms.filter((r) => r.status === 'dirty').length,
    cleaningCount: rooms.filter((r) => r.status === 'cleaning').length,
    cleanedTodayCount: rooms.filter((r) => r.status === 'available').length, // Simplified for now
    avgCleaningTimeMinutes: null, // Would need tracking data
  }

  // Fetch rooms data
  const fetchRooms = useCallback(async () => {
    try {
      const response = await branchFetch('/api/rooms?limit=200')
      if (!response.ok) {
        throw new Error('ไม่สามารถดึงข้อมูลห้องได้')
      }

      const data: RoomsResponse = await response.json()
      if (!data.success) {
        throw new Error('ไม่สามารถดึงข้อมูลห้องได้')
      }

      // Transform API data to housekeeping format
      const housekeepingRooms: HousekeepingRoom[] = data.data.map((room) => ({
        id: room.id,
        roomNo: room.roomNo,
        roomTypeName: room.roomTypeName,
        floor: room.floor,
        status: mapToHousekeepingStatus(room.status, room.isClean),
        lastCheckoutTime: null, // Would need checkout tracking
        statusChangedAt: room.updatedAt,
        housekeeperName: null, // Would need assignment tracking
        notes: room.notes,
      }))

      setRooms(housekeepingRooms)
      setError(null)
      setLastRefresh(new Date())
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setLoading(false)
    }
  }, [branchFetch])

  // Initial fetch and auto-refresh every 30 seconds
  useEffect(() => {
    fetchRooms()

    const interval = setInterval(() => {
      fetchRooms()
    }, 30000)

    return () => clearInterval(interval)
  }, [fetchRooms])

  // Handle room status change.
  //
  // 'dirty' / 'available' go through the housekeeping endpoints, which flip the
  // canonical `room_clean` flag AND mirror to legacy HT_Rooms.Room_Clean +
  // HT_Housewife. The old code PATCHed `/api/rooms/:id/status` with
  // 'available' for BOTH (the "dirty maps to available" hack) — a no-op that
  // never changed cleanliness and never reached iHOTEL.
  //
  // 'cleaning' is a transient in-progress state with no legacy counterpart
  // (iHOTEL's Room_Clean is a boolean) and no persistent backing (/api/rooms
  // derives status live and never returns 'cleaning'), so it stays a plain
  // canonical room_status change — the dirty writeback already fired when the
  // room entered the dirty state.
  const handleStatusChange = async (
    roomId: number,
    newStatus: HousekeepingStatus,
    notes?: string
  ) => {
    try {
      let response: Response
      if (newStatus === 'dirty') {
        response = await branchFetch(`/api/housekeeping/rooms/${roomId}/dirty`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ by: 'แม่บ้าน' }),
        })
      } else if (newStatus === 'available') {
        response = await branchFetch(`/api/housekeeping/rooms/${roomId}/clean`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ by: 'แม่บ้าน' }),
        })
      } else {
        response = await branchFetch(`/api/rooms/${roomId}/status`, {
          method: 'PATCH',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ status: 'cleaning' }),
        })
      }

      if (!response.ok) {
        throw new Error('ไม่สามารถอัพเดทสถานะได้')
      }

      // Update local state optimistically
      setRooms((prev) =>
        prev.map((room) =>
          room.id === roomId
            ? {
                ...room,
                status: newStatus,
                statusChangedAt: new Date().toISOString(),
                notes: notes || room.notes,
              }
            : room
        )
      )
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
      // Refresh to get correct state
      fetchRooms()
    }
  }

  // Filter rooms by floor
  const filteredRooms =
    floorFilter !== null
      ? rooms.filter((r) => r.floor === floorFilter)
      : rooms

  // Group rooms by status
  const roomsByStatus = {
    dirty: filteredRooms.filter((r) => r.status === 'dirty'),
    cleaning: filteredRooms.filter((r) => r.status === 'cleaning'),
    available: filteredRooms.filter((r) => r.status === 'available'),
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
            <Sparkles className="w-7 h-7 text-red-600" />
            แผนกแม่บ้าน
          </h1>
          <p className="text-sm text-gray-500 mt-1">
            จัดการสถานะความสะอาดห้องพัก | รีเฟรชล่าสุด: {formatLastRefresh()}
          </p>
        </div>

        <div className="flex items-center gap-3">
          {/* Floor Filter */}
          <div className="flex items-center gap-2">
            <Filter className="w-4 h-4 text-gray-500" />
            <select
              value={floorFilter ?? ''}
              onChange={(e) =>
                setFloorFilter(e.target.value ? parseInt(e.target.value) : null)
              }
              className="px-3 py-2 border border-gray-300 rounded-lg text-sm bg-white text-gray-800 focus:ring-2 focus:ring-red-500 focus:border-red-500"
            >
              <option value="">ทุกชั้น</option>
              {floors.map((floor) => (
                <option key={floor} value={floor}>
                  ชั้น {floor}
                </option>
              ))}
            </select>
          </div>

          {/* Refresh Button */}
          <button
            onClick={fetchRooms}
            disabled={loading}
            className="flex items-center gap-2 px-4 py-2 border border-gray-300 text-gray-500 rounded-lg hover:bg-gray-100 disabled:opacity-50"
          >
            <RefreshCw className={`w-4 h-4 ${loading ? 'animate-spin' : ''}`} />
            <span className="hidden sm:inline">รีเฟรช</span>
          </button>
        </div>
      </div>

      {/* Stats */}
      <HousekeepingStats stats={stats} loading={loading} />

      {/* Error Message */}
      {error && (
        <div className="flex items-center gap-2 p-4 bg-red-50 border border-red-200 rounded-lg text-red-600">
          <AlertCircle className="w-5 h-5" />
          <span>{error}</span>
        </div>
      )}

      {/* Kanban Board */}
      {loading && rooms.length === 0 ? (
        <div className="flex items-center justify-center py-12">
          <Loader2 className="w-8 h-8 animate-spin text-red-500" />
          <span className="ml-2 text-gray-500">กำลังโหลด...</span>
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
                      {roomsByStatus[column.id].length}
                    </span>
                  </div>
                </div>
              </div>

              {/* Column Content */}
              <div className="p-4 space-y-3 max-h-[600px] overflow-y-auto">
                {roomsByStatus[column.id].length === 0 ? (
                  <div className="text-center py-8 text-gray-500">
                    <p>ไม่มีห้องในหมวดนี้</p>
                  </div>
                ) : (
                  roomsByStatus[column.id]
                    .sort((a, b) => a.roomNo.localeCompare(b.roomNo, undefined, { numeric: true }))
                    .map((room) => (
                      <RoomCleaningCard
                        key={room.id}
                        room={room}
                        onStatusChange={handleStatusChange}
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
    </div>
  )
}
