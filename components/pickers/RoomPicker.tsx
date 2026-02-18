'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  Bed,
  Check,
  Loader2,
  AlertCircle,
  Filter,
} from 'lucide-react'

export interface RoomOption {
  id: number
  roomNo: string
  roomTypeId: number | null
  roomTypeName: string | null
  floor: number | null
  status: string
  priceWeekday: number | null
  priceWeekend: number | null
}

interface RoomPickerProps {
  value: RoomOption[]
  onChange: (rooms: RoomOption[]) => void
  disabled?: boolean
  checkInDate?: string
  checkOutDate?: string
  excludeBookingId?: number
  className?: string
}

interface RoomType {
  id: number
  name: string
}

export default function RoomPicker({
  value,
  onChange,
  disabled = false,
  checkInDate,
  checkOutDate,
  excludeBookingId,
  className = '',
}: RoomPickerProps) {
  const [rooms, setRooms] = useState<RoomOption[]>([])
  const [roomTypes, setRoomTypes] = useState<RoomType[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [selectedTypeFilter, setSelectedTypeFilter] = useState<number | null>(null)

  // Fetch rooms
  const fetchRooms = useCallback(async () => {
    setLoading(true)
    setError(null)

    try {
      const params = new URLSearchParams({
        limit: '100',
      })

      // If we have dates, we could later add availability check
      // For now, just fetch all rooms and let user select
      if (checkInDate && checkOutDate) {
        params.append('checkIn', checkInDate)
        params.append('checkOut', checkOutDate)
        if (excludeBookingId) {
          params.append('excludeBookingId', excludeBookingId.toString())
        }
      }

      const response = await fetch(`/api/new/rooms?${params}`)
      if (!response.ok) {
        throw new Error('ไม่สามารถดึงข้อมูลห้องพักได้')
      }

      const data = await response.json()
      if (data.success) {
        const roomData: RoomOption[] = data.data.map((r: {
          id: number
          roomNo: string
          roomTypeId: number | null
          roomTypeName: string | null
          floor: number | null
          status: string
          priceWeekday: number | null
          priceWeekend: number | null
        }) => ({
          id: r.id,
          roomNo: r.roomNo,
          roomTypeId: r.roomTypeId,
          roomTypeName: r.roomTypeName,
          floor: r.floor,
          status: r.status,
          priceWeekday: r.priceWeekday,
          priceWeekend: r.priceWeekend,
        }))
        setRooms(roomData)

        // Extract unique room types
        const types: RoomType[] = []
        const seenTypes = new Set<number>()
        for (const room of roomData) {
          if (room.roomTypeId && !seenTypes.has(room.roomTypeId)) {
            seenTypes.add(room.roomTypeId)
            types.push({
              id: room.roomTypeId,
              name: room.roomTypeName || `ประเภท ${room.roomTypeId}`,
            })
          }
        }
        setRoomTypes(types.sort((a, b) => a.name.localeCompare(b.name, 'th')))
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setLoading(false)
    }
  }, [checkInDate, checkOutDate, excludeBookingId])

  useEffect(() => {
    fetchRooms()
  }, [fetchRooms])

  const isSelected = (roomId: number) => {
    return value.some((r) => r.id === roomId)
  }

  const toggleRoom = (room: RoomOption) => {
    if (disabled) return

    if (isSelected(room.id)) {
      onChange(value.filter((r) => r.id !== room.id))
    } else {
      onChange([...value, room])
    }
  }

  const filteredRooms = selectedTypeFilter
    ? rooms.filter((r) => r.roomTypeId === selectedTypeFilter)
    : rooms

  // Group rooms by floor
  const roomsByFloor = filteredRooms.reduce((acc, room) => {
    const floor = room.floor ?? 0
    if (!acc[floor]) {
      acc[floor] = []
    }
    acc[floor].push(room)
    return acc
  }, {} as Record<number, RoomOption[]>)

  const sortedFloors = Object.keys(roomsByFloor)
    .map(Number)
    .sort((a, b) => a - b)

  const formatPrice = (price: number | null) => {
    if (price == null) return '-'
    return price.toLocaleString('th-TH')
  }

  if (loading) {
    return (
      <div className={`flex items-center justify-center py-8 ${className}`}>
        <Loader2 className="w-6 h-6 animate-spin text-red-600 mr-2" />
        <span className="text-gray-500">กำลังโหลดห้องพัก...</span>
      </div>
    )
  }

  if (error) {
    return (
      <div className={`flex items-center justify-center py-8 text-red-600 ${className}`}>
        <AlertCircle className="w-5 h-5 mr-2" />
        <span>{error}</span>
      </div>
    )
  }

  return (
    <div className={className}>
      {/* Filter by room type */}
      <div className="flex items-center gap-2 mb-4 flex-wrap">
        <div className="flex items-center gap-1 text-sm text-gray-500">
          <Filter className="w-4 h-4" />
          <span>กรองประเภท:</span>
        </div>
        <button
          type="button"
          onClick={() => setSelectedTypeFilter(null)}
          className={`px-3 py-1 text-sm rounded-full transition-colors ${
            selectedTypeFilter === null
              ? 'bg-red-600 text-white'
              : 'bg-gray-100 text-gray-700 hover:bg-gray-100'
          }`}
        >
          ทั้งหมด
        </button>
        {roomTypes.map((type) => (
          <button
            key={type.id}
            type="button"
            onClick={() => setSelectedTypeFilter(type.id)}
            className={`px-3 py-1 text-sm rounded-full transition-colors ${
              selectedTypeFilter === type.id
                ? 'bg-red-600 text-white'
                : 'bg-gray-100 text-gray-700 hover:bg-gray-100'
            }`}
          >
            {type.name}
          </button>
        ))}
      </div>

      {/* Selected count */}
      {value.length > 0 && (
        <div className="mb-4 p-3 bg-red-500/10 border border-red-500 rounded-lg">
          <div className="flex items-center justify-between">
            <span className="text-sm text-red-600">
              เลือกแล้ว {value.length} ห้อง
            </span>
            <button
              type="button"
              onClick={() => onChange([])}
              className="text-sm text-red-600 hover:text-red-300 font-medium"
            >
              ล้างทั้งหมด
            </button>
          </div>
          <div className="flex flex-wrap gap-2 mt-2">
            {value.map((room) => (
              <span
                key={room.id}
                className="inline-flex items-center gap-1 px-2 py-1 bg-red-500/10 text-red-600 rounded text-sm"
              >
                <Bed className="w-3 h-3" />
                {room.roomNo}
                <button
                  type="button"
                  onClick={() => toggleRoom(room)}
                  className="ml-1 hover:text-red-300"
                >
                  &times;
                </button>
              </span>
            ))}
          </div>
        </div>
      )}

      {/* Room grid by floor */}
      <div className="space-y-4 max-h-80 overflow-y-auto">
        {sortedFloors.map((floor) => (
          <div key={floor}>
            <h4 className="text-sm font-medium text-gray-500 mb-2">
              {floor === 0 ? 'ไม่ระบุชั้น' : `ชั้น ${floor}`}
            </h4>
            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
              {roomsByFloor[floor]
                .sort((a, b) => a.roomNo.localeCompare(b.roomNo, 'th', { numeric: true }))
                .map((room) => {
                  const selected = isSelected(room.id)
                  const isAvailable = room.status === 'available'

                  return (
                    <button
                      key={room.id}
                      type="button"
                      onClick={() => toggleRoom(room)}
                      disabled={disabled || !isAvailable}
                      className={`
                        relative p-3 rounded-lg border-2 text-left transition-all
                        ${selected
                          ? 'border-red-500 bg-red-500/10 ring-2 ring-red-500/20'
                          : isAvailable
                            ? 'border-gray-200 bg-white hover:border-red-500 hover:bg-red-500/10'
                            : 'border-gray-200 bg-gray-100 opacity-60 cursor-not-allowed'
                        }
                        ${disabled ? 'cursor-not-allowed opacity-60' : ''}
                      `}
                    >
                      {/* Selected checkmark */}
                      {selected && (
                        <div className="absolute top-1 right-1 w-5 h-5 bg-red-500 rounded-full flex items-center justify-center">
                          <Check className="w-3 h-3 text-white" />
                        </div>
                      )}

                      {/* Room number */}
                      <div className="flex items-center gap-1 mb-1">
                        <Bed className="w-4 h-4 text-gray-500" />
                        <span className="font-bold text-gray-800">{room.roomNo}</span>
                      </div>

                      {/* Room type */}
                      <div className="text-xs text-gray-500 truncate mb-1">
                        {room.roomTypeName || '-'}
                      </div>

                      {/* Price */}
                      <div className="text-sm font-medium text-red-600">
                        {formatPrice(room.priceWeekday)} บ./คืน
                      </div>

                      {/* Status badge */}
                      {!isAvailable && (
                        <div className="mt-1">
                          <span className={`
                            inline-block px-1.5 py-0.5 text-xs rounded
                            ${room.status === 'occupied' ? 'bg-red-500/10 text-red-600' : ''}
                            ${room.status === 'maintenance' ? 'bg-yellow-500/10 text-yellow-400' : ''}
                            ${room.status === 'cleaning' ? 'bg-amber-500/10 text-amber-400' : ''}
                          `}>
                            {room.status === 'occupied' && 'มีผู้เข้าพัก'}
                            {room.status === 'maintenance' && 'ซ่อมบำรุง'}
                            {room.status === 'cleaning' && 'ทำความสะอาด'}
                          </span>
                        </div>
                      )}
                    </button>
                  )
                })}
            </div>
          </div>
        ))}
      </div>

      {filteredRooms.length === 0 && (
        <div className="text-center py-8 text-gray-500">
          <Bed className="w-12 h-12 text-gray-400 mx-auto mb-2" />
          <p>ไม่พบห้องพัก</p>
        </div>
      )}
    </div>
  )
}
