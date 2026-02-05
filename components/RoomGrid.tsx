'use client'

import { useState } from 'react'
import { X, LogIn, LogOut } from 'lucide-react'

export type RoomStatus = 'available' | 'occupied' | 'booked' | 'maintenance' | 'cleaning' | 'checkout'

export interface Room {
  id: number
  roomNumber: string
  type: string
  details?: string
  status: RoomStatus
  floor?: number
  guestName?: string
  checkIn?: string
  checkOut?: string
  // New mode specific fields
  newRoomId?: number
  checkInId?: number
}

interface RoomGridProps {
  rooms: Room[]
  onRoomClick?: (room: Room) => void
  // New mode specific props
  isNewMode?: boolean
  onQuickCheckIn?: (room: Room) => void
  onCheckOut?: (room: Room) => void
}

const statusConfig = {
  available: {
    dot: 'bg-green-500',
    border: 'border-b-green-500',
    bg: 'bg-white hover:bg-green-50',
    label: 'ว่าง',
  },
  occupied: {
    dot: 'bg-red-500',
    border: 'border-b-red-500',
    bg: 'bg-red-50 hover:bg-red-100',
    label: 'มีผู้เข้าพัก',
  },
  booked: {
    dot: 'bg-yellow-500',
    border: 'border-b-yellow-500',
    bg: 'bg-yellow-50 hover:bg-yellow-100',
    label: 'จองแล้ว',
  },
  maintenance: {
    dot: 'bg-gray-400',
    border: 'border-b-gray-400',
    bg: 'bg-gray-100 hover:bg-gray-200',
    label: 'ซ่อมบำรุง',
  },
  cleaning: {
    dot: 'bg-orange-500',
    border: 'border-b-orange-500',
    bg: 'bg-orange-50 hover:bg-orange-100',
    label: 'ทำความสะอาด',
  },
  checkout: {
    dot: 'bg-blue-400',
    border: 'border-b-blue-400',
    bg: 'bg-blue-50 hover:bg-blue-100',
    label: 'รอเช็คเอาท์',
  },
}

// Custom room layout - null represents blank spaces
const roomLayout: (string | null)[][] = [
  ['509', '510', '511', '512', '513', '514', '515', '516', '517', '518'],
  ['508', '507', null, '506', '505', '504', '503', '502', '501', null, null, 'A4-1', 'A4-2', 'A4-3'],
  ['409', '410', '411', '412', '413', '414', '415', '416', '417', '418', null, 'A3-1', 'A3-2', 'A3-3'],
  ['408', '407', null, '406', '405', '404', '403', '402', '401', null, null, 'V.201', 'A2-1', 'A2-3'],
  ['307', '308', '309', '310', '311', '312', '313'],
  ['306', '305', null, '304', '303', '302', '301'],
]

export default function RoomGrid({
  rooms,
  onRoomClick,
  isNewMode = false,
  onQuickCheckIn,
  onCheckOut,
}: RoomGridProps) {
  const [selectedRoom, setSelectedRoom] = useState<Room | null>(null)

  // Create a map for quick room lookup by room number (case-insensitive)
  const roomMap = new Map<string, Room>()
  rooms.forEach(room => {
    roomMap.set(room.roomNumber.toUpperCase(), room)
  })

  const handleRoomClick = (room: Room) => {
    setSelectedRoom(room)
    onRoomClick?.(room)
  }

  const closeModal = () => {
    setSelectedRoom(null)
  }

  // Find max columns for grid
  const maxColumns = Math.max(...roomLayout.map(row => row.length))

  // Sort rooms by room number for mobile list view
  const sortedRooms = [...rooms].sort((a, b) =>
    a.roomNumber.localeCompare(b.roomNumber, undefined, { numeric: true })
  )

  return (
    <div className="relative">
      {/* Mobile List View */}
      <div className="md:hidden space-y-2">
        {sortedRooms.map(room => {
          const config = statusConfig[room.status]
          return (
            <button
              key={room.id}
              onClick={() => handleRoomClick(room)}
              className={`${config.bg} w-full flex items-center gap-3 p-3 rounded-lg border border-gray-200 border-l-4 ${config.border}`}
            >
              <div className={`w-3 h-3 rounded-full ${config.dot}`} />
              <span className="font-bold">{room.roomNumber}</span>
              <span className="text-gray-600">{room.type}</span>
              <span className="text-gray-500 text-sm">{room.details}</span>
            </button>
          )
        })}
      </div>

      {/* Desktop Room Grid */}
      <div className="hidden md:block">
        <div className="grid gap-2" style={{ gridTemplateColumns: `repeat(${maxColumns}, minmax(0, 1fr))` }}>
          {roomLayout.map((row, rowIndex) => (
            <div key={`row-${rowIndex}`} className="contents">
              {row.map((roomNumber, colIndex) => {
                if (roomNumber === null) {
                  // Blank space
                  return (
                    <div
                      key={`blank-${rowIndex}-${colIndex}`}
                      className="h-[70px]"
                    />
                  )
                }

                const room = roomMap.get(roomNumber.toUpperCase())
                if (!room) {
                  // Room not found in data - show as gray placeholder
                  return (
                    <div
                      key={`missing-${roomNumber}`}
                      className="h-[70px] bg-gray-200 rounded-lg p-1 flex flex-col items-center justify-center"
                    >
                      <span className="font-bold text-xs text-gray-400">{roomNumber}</span>
                      <span className="text-[9px] text-gray-400">ไม่พบ</span>
                    </div>
                  )
                }

                const config = statusConfig[room.status]

                return (
                  <button
                    key={room.id}
                    onClick={() => handleRoomClick(room)}
                    className={`${config.bg} border border-gray-200 border-b-4 ${config.border} rounded-lg p-1
                      hover:shadow-md
                      flex flex-col items-center justify-center h-[70px] overflow-hidden`}
                    title={`${room.roomNumber} - ${room.type} - ${room.details || ''}`}
                  >
                    <span className="font-bold text-xs leading-tight text-gray-800">{room.roomNumber}</span>
                    <span className="text-[8px] leading-tight text-gray-600">{room.type}</span>
                    <span className="text-[8px] leading-tight text-gray-500">{room.details}</span>
                  </button>
                )
              })}
              {/* Fill remaining columns with empty cells */}
              {Array.from({ length: maxColumns - row.length }).map((_, i) => (
                <div key={`filler-${rowIndex}-${i}`} className="h-[70px]" />
              ))}
            </div>
          ))}
        </div>
      </div>

      {/* Legend */}
      <div className="flex flex-wrap gap-2 md:gap-4 mt-4">
        {Object.entries(statusConfig).map(([status, config]) => (
          <div key={status} className="flex items-center gap-1 md:gap-2">
            <div className={`w-2 h-2 md:w-3 md:h-3 rounded-full ${config.dot}`} />
            <span className="text-xs md:text-sm text-gray-600">{config.label}</span>
          </div>
        ))}
      </div>

      {/* Room Detail Modal */}
      {selectedRoom && (
        <div
          className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
          onClick={closeModal}
        >
          <div
            className="bg-white rounded-xl shadow-2xl p-6 max-w-md w-full mx-4"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="flex justify-between items-start mb-4">
              <div>
                <h3 className="text-2xl font-bold text-gray-800">
                  ห้อง {selectedRoom.roomNumber}
                </h3>
                <p className="text-gray-500">{selectedRoom.type}</p>
              </div>
              <button
                onClick={closeModal}
                className="text-gray-400 hover:text-gray-600 transition-colors"
              >
                <X size={24} />
              </button>
            </div>

            <div className="space-y-3">
              <div className="flex items-center gap-2">
                <span className="text-gray-600">สถานะ:</span>
                <span className="flex items-center gap-2 px-3 py-1 rounded-full text-sm bg-gray-100">
                  <span className={`w-2 h-2 rounded-full ${statusConfig[selectedRoom.status].dot}`}></span>
                  {statusConfig[selectedRoom.status].label}
                </span>
              </div>

              {selectedRoom.floor && (
                <div className="flex items-center gap-2">
                  <span className="text-gray-600">ชั้น:</span>
                  <span className="font-medium">{selectedRoom.floor}</span>
                </div>
              )}

              {selectedRoom.guestName && (
                <div className="flex items-center gap-2">
                  <span className="text-gray-600">ผู้เข้าพัก:</span>
                  <span className="font-medium">{selectedRoom.guestName}</span>
                </div>
              )}

              {selectedRoom.checkIn && (
                <div className="flex items-center gap-2">
                  <span className="text-gray-600">เช็คอิน:</span>
                  <span className="font-medium">{selectedRoom.checkIn}</span>
                </div>
              )}

              {selectedRoom.checkOut && (
                <div className="flex items-center gap-2">
                  <span className="text-gray-600">เช็คเอาท์:</span>
                  <span className="font-medium">{selectedRoom.checkOut}</span>
                </div>
              )}
            </div>

            {/* Action buttons for New Mode */}
            {isNewMode && (
              <div className="mt-6 space-y-2">
                {/* Check-in button - only for available rooms */}
                {selectedRoom.status === 'available' && onQuickCheckIn && (
                  <button
                    onClick={() => {
                      onQuickCheckIn(selectedRoom)
                      closeModal()
                    }}
                    className="w-full flex items-center justify-center gap-2 bg-green-600 hover:bg-green-700 text-white py-2 rounded-lg"
                  >
                    <LogIn size={18} />
                    เช็คอินด่วน
                  </button>
                )}

                {/* Check-out button - for occupied or checkout rooms */}
                {(selectedRoom.status === 'occupied' || selectedRoom.status === 'checkout') && onCheckOut && (
                  <button
                    onClick={() => {
                      onCheckOut(selectedRoom)
                      closeModal()
                    }}
                    className="w-full flex items-center justify-center gap-2 bg-blue-600 hover:bg-blue-700 text-white py-2 rounded-lg"
                  >
                    <LogOut size={18} />
                    เช็คเอาท์
                  </button>
                )}

                <button
                  onClick={closeModal}
                  className="w-full border border-gray-300 text-gray-700 py-2 rounded-lg hover:bg-gray-50"
                >
                  ปิด
                </button>
              </div>
            )}

            {/* Close button for Legacy Mode */}
            {!isNewMode && (
              <button
                onClick={closeModal}
                className="mt-6 w-full bg-blue-600 hover:bg-blue-700 text-white py-2 rounded-lg"
              >
                ปิด
              </button>
            )}
          </div>
        </div>
      )}
    </div>
  )
}
