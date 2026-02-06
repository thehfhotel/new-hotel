'use client'

import { useEffect, useState, useCallback } from 'react'
import { Clock, LogIn } from 'lucide-react'
import StatsCard from '@/components/StatsCard'
import RoomGrid, { Room, RoomStatus } from '@/components/RoomGrid'
import { OccupancyChart, OccupancyData } from '@/components/Charts'
import { useMode } from '@/contexts/ModeContext'
import QuickCheckInModal from '@/components/modals/QuickCheckInModal'
import CheckOutModal from '@/components/modals/CheckOutModal'

interface Stats {
  totalRooms: number
  occupiedRooms: number
  checkoutRooms: number
  bookedRooms: number
  todayCheckIns: number
  activeBookings: number
}

interface CheckIn {
  id: number
  guestName: string
  roomNumber: string
  checkInDate: string
  checkOutDate: string
  status?: string
}

// API response types
interface ApiRoom {
  Room_no: string
  Room_Type: string
  Room_Details: string
  Room_Clean: string
  Room_Use: string
  Room_Book: string
  Room_Manternace: string
}

interface ApiCheckIn {
  Cin_no: string
  Cin_Room_No: string
  Cin_Room_In: string
  Cin_Room_Out: string
  Cin_cust_name: string
  Cin_status: string
}

// New mode API types
interface NewRoom {
  id: number
  roomNo: string
  roomTypeName: string | null
  floor: number | null
  status: string
  isClean: boolean
  isMaintenance: boolean
}

interface NewCheckIn {
  id: number
  cinNo: string
  customerId: number
  customerName: string | null
  roomId: number
  roomNo: string | null
  checkInTime: string | null
  expectedCheckout: string | null
  ratePerNight: number | null
  status: string
}

// Room info for modals
interface ModalRoom {
  id: number
  roomNumber: string
  type: string
  details?: string
}

interface CheckInDetails {
  id: number
  cinNo: string
  customerId: number
  customerName: string | null
  roomId: number
  roomNo: string | null
  checkInTime: string | null
  expectedCheckout: string | null
  ratePerNight: number | null
  status: string
}

// Function to determine room status from API data
function getRoomStatus(room: ApiRoom, isCheckoutToday: boolean): RoomStatus {
  // Check if it's past 6 AM and room has checkout today
  const now = new Date()
  const hour = now.getHours()
  if (isCheckoutToday && hour >= 6) return 'checkout'

  if (room.Room_Manternace === 'yes') return 'maintenance'
  if (room.Room_Use === 'yes') return 'occupied'
  if (room.Room_Book && room.Room_Book !== '') return 'booked' // Has a booking but not checked in
  return 'available'
}

export default function Dashboard() {
  const { isNew } = useMode()

  const [stats, setStats] = useState<Stats>({
    totalRooms: 0,
    occupiedRooms: 0,
    checkoutRooms: 0,
    bookedRooms: 0,
    todayCheckIns: 0,
    activeBookings: 0,
  })
  const [rooms, setRooms] = useState<Room[]>([])
  const [checkIns, setCheckIns] = useState<CheckIn[]>([])
  const [occupancyData, setOccupancyData] = useState<OccupancyData[]>([])
  const [loading, setLoading] = useState(true)

  // New mode specific state
  const [newRooms, setNewRooms] = useState<NewRoom[]>([])
  const [activeCheckIns, setActiveCheckIns] = useState<Map<number, NewCheckIn>>(new Map())

  // Modal state
  const [showCheckInModal, setShowCheckInModal] = useState(false)
  const [showCheckOutModal, setShowCheckOutModal] = useState(false)
  const [selectedRoomForModal, setSelectedRoomForModal] = useState<ModalRoom | null>(null)
  const [selectedCheckIn, setSelectedCheckIn] = useState<CheckInDetails | null>(null)

  // Fetch new mode data (rooms and active check-ins)
  const fetchNewModeData = useCallback(async () => {
    try {
      // Fetch new rooms
      const roomsRes = await fetch('/api/new/rooms?limit=100')
      if (roomsRes.ok) {
        const data = await roomsRes.json()
        if (data.success) {
          setNewRooms(data.data)
        }
      }

      // Fetch active check-ins to map rooms to check-ins
      const checkInsRes = await fetch('/api/new/checkins?status=active&limit=100')
      if (checkInsRes.ok) {
        const data = await checkInsRes.json()
        if (data.success) {
          // Create a map of roomId -> checkIn
          const checkInMap = new Map<number, NewCheckIn>()
          data.data.forEach((checkIn: NewCheckIn) => {
            checkInMap.set(checkIn.roomId, checkIn)
          })
          setActiveCheckIns(checkInMap)
        }
      }
    } catch (error) {
      console.error('Error fetching new mode data:', error)
    }
  }, [])

  // Get room status for new mode
  const getNewRoomStatus = (room: NewRoom): RoomStatus => {
    if (room.isMaintenance) return 'maintenance'
    if (room.status === 'occupied') return 'occupied'
    if (room.status === 'booked') return 'booked'
    if (!room.isClean) return 'cleaning'
    return 'available'
  }

  // Handle quick check-in for new mode
  const handleQuickCheckIn = (room: Room) => {
    // Find the new room by room number
    const newRoom = newRooms.find(r => r.roomNo === room.roomNumber)
    if (newRoom) {
      setSelectedRoomForModal({
        id: newRoom.id,
        roomNumber: newRoom.roomNo,
        type: newRoom.roomTypeName || 'ไม่ระบุ',
      })
      setShowCheckInModal(true)
    }
  }

  // Handle check-out for new mode
  const handleCheckOut = (room: Room) => {
    // Find the new room by room number
    const newRoom = newRooms.find(r => r.roomNo === room.roomNumber)
    if (newRoom) {
      // Find the active check-in for this room
      const checkIn = activeCheckIns.get(newRoom.id)
      if (checkIn) {
        setSelectedCheckIn({
          id: checkIn.id,
          cinNo: checkIn.cinNo,
          customerId: checkIn.customerId,
          customerName: checkIn.customerName,
          roomId: checkIn.roomId,
          roomNo: checkIn.roomNo,
          checkInTime: checkIn.checkInTime,
          expectedCheckout: checkIn.expectedCheckout,
          ratePerNight: checkIn.ratePerNight,
          status: checkIn.status,
        })
        setShowCheckOutModal(true)
      }
    }
  }

  // Handle modal success (refresh data)
  const handleModalSuccess = () => {
    fetchNewModeData()
    // Also refresh legacy data
    setLoading(true)
    fetchData()
  }

  const fetchData = useCallback(async () => {
    try {
      // Fetch stats
      const statsRes = await fetch('/api/stats')
      if (statsRes.ok) {
        const statsData = await statsRes.json()
        if (statsData.success && statsData.data) {
          setStats({
            totalRooms: statsData.data.totalRooms || 0,
            occupiedRooms: statsData.data.occupiedRooms || 0,
            checkoutRooms: statsData.data.checkoutRooms || 0,
            bookedRooms: statsData.data.bookedRooms || 0,
            todayCheckIns: statsData.data.todayCheckIns || 0,
            activeBookings: statsData.data.activeBookings || 0,
          })
        }
      }
    } catch (error) {
      console.error('Error fetching stats:', error)
    }

    try {
      // Fetch rooms and checkout status in parallel
      const [roomsRes, checkoutsRes] = await Promise.all([
        fetch('/api/rooms'),
        fetch('/api/rooms/checkouts-today')
      ])

      // Build checkout rooms set from dedicated API
      let checkoutRooms = new Set<string>()
      if (checkoutsRes.ok) {
        const checkoutsData = await checkoutsRes.json()
        if (checkoutsData.success && checkoutsData.data) {
          checkoutRooms = new Set<string>(checkoutsData.data)
        }
      }

      if (roomsRes.ok) {
        const roomsData = await roomsRes.json()
        if (roomsData.success && roomsData.data) {
          // Map API response to Room type
          const mappedRooms: Room[] = roomsData.data.map((room: ApiRoom, index: number) => ({
            id: index + 1,
            roomNumber: room.Room_no,
            type: room.Room_Type?.trim() || 'ไม่ระบุ',
            details: room.Room_Details?.trim() || '',
            status: getRoomStatus(room, checkoutRooms.has(room.Room_no)),
          }))
          setRooms(mappedRooms)
        }
      }
    } catch (error) {
      console.error('Error fetching rooms:', error)
    }

    try {
      // Fetch check-ins (limited to recent)
      const checkInsRes = await fetch('/api/checkins?limit=10')
      if (checkInsRes.ok) {
        const checkInsData = await checkInsRes.json()
        if (checkInsData.success && checkInsData.data) {
          // Map API response to CheckIn type
          const mappedCheckIns: CheckIn[] = checkInsData.data.map((checkin: ApiCheckIn, index: number) => ({
            id: index + 1,
            guestName: checkin.Cin_cust_name?.trim() || 'ไม่ระบุ',
            roomNumber: checkin.Cin_Room_No,
            checkInDate: checkin.Cin_Room_In,
            checkOutDate: checkin.Cin_Room_Out,
            status: checkin.Cin_status,
          }))
          setCheckIns(mappedCheckIns)
        }
      }
    } catch (error) {
      console.error('Error fetching check-ins:', error)
    }

    try {
      // Fetch occupancy data for chart
      const occupancyRes = await fetch('/api/occupancy?days=7')
      if (occupancyRes.ok) {
        const occupancyResult = await occupancyRes.json()
        if (occupancyResult.success && occupancyResult.data) {
          // Format dates for display
          const formattedData: OccupancyData[] = occupancyResult.data.map((item: { date: string; occupiedRooms: number }) => ({
            date: new Date(item.date).toLocaleDateString('th-TH', { day: 'numeric', month: 'short', timeZone: 'UTC' }),
            occupiedRooms: item.occupiedRooms || 0,
          }))
          setOccupancyData(formattedData)
        }
      }
    } catch (error) {
      console.error('Error fetching occupancy data:', error)
    }

    setLoading(false)
  }, [])

  useEffect(() => {
    fetchData()
  }, [fetchData])

  // Fetch new mode data when mode changes
  useEffect(() => {
    if (isNew) {
      fetchNewModeData()
    }
  }, [isNew, fetchNewModeData])

  // Combine rooms for new mode display
  const displayRooms: Room[] = isNew && newRooms.length > 0
    ? newRooms.map((room) => ({
        id: room.id,
        roomNumber: room.roomNo,
        type: room.roomTypeName || 'ไม่ระบุ',
        status: getNewRoomStatus(room),
        newRoomId: room.id,
        checkInId: activeCheckIns.get(room.id)?.id,
      }))
    : rooms

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-[50vh]">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
          <p className="text-gray-600">กำลังโหลดข้อมูล...</p>
        </div>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      {/* Page Title */}
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold text-gray-800">หน้าหลัก</h1>
        <p className="text-gray-500 text-sm">
          อัปเดตล่าสุด: {new Date().toLocaleDateString('th-TH', {
            year: 'numeric',
            month: 'long',
            day: 'numeric',
            hour: '2-digit',
            minute: '2-digit',
          })}
        </p>
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
        <StatsCard
          title="ห้องที่มีผู้เข้าพัก"
          value={stats.occupiedRooms}
        />
        <StatsCard
          title="ห้องที่จองแล้ว"
          value={stats.bookedRooms}
        />
        <StatsCard
          title="ห้องรอเช็คเอาท์"
          value={stats.checkoutRooms}
        />
      </div>

      {/* Room Grid */}
      <div className="bg-white rounded-xl shadow-sm border border-gray-100 p-6">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-lg font-semibold text-gray-800">สถานะห้องพัก</h2>
          {isNew && (
            <span className="text-xs bg-green-100 text-green-700 px-2 py-1 rounded-full">
              โหมดใหม่ - คลิกห้องเพื่อเช็คอิน/เช็คเอาท์
            </span>
          )}
        </div>
        <RoomGrid
          rooms={displayRooms}
          isNewMode={isNew}
          onQuickCheckIn={handleQuickCheckIn}
          onCheckOut={handleCheckOut}
        />
      </div>

      {/* Charts and Recent Activity */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Occupancy Chart */}
        <OccupancyChart
          data={occupancyData}
          chartType="bar"
          title="จำนวนห้องที่มีผู้เข้าพัก 7 วันล่าสุด"
        />

        {/* Recent Activity */}
        <div className="bg-white rounded-xl shadow-sm border border-gray-100 p-6">
          <h3 className="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
            <Clock size={20} className="text-blue-600" />
            กิจกรรมล่าสุด
          </h3>
          <div className="space-y-4">
            {checkIns.length > 0 ? (
              checkIns.slice(0, 5).map((checkin) => (
                <div
                  key={checkin.id}
                  className="flex items-center justify-between p-3 bg-gray-50 rounded-lg hover:bg-gray-100"
                >
                  <div className="flex items-center gap-3">
                    <div className="w-10 h-10 bg-blue-100 rounded-full flex items-center justify-center">
                      <LogIn size={18} className="text-blue-600" />
                    </div>
                    <div>
                      <p className="font-medium text-gray-800">{checkin.guestName}</p>
                      <p className="text-sm text-gray-500">ห้อง {checkin.roomNumber}</p>
                    </div>
                  </div>
                  <div className="text-right">
                    <p className="text-sm text-gray-600">
                      {new Date(checkin.checkInDate).toLocaleDateString('th-TH', {
                        day: 'numeric',
                        month: 'short',
                        timeZone: 'UTC',
                      })}
                      {' '}
                      {new Date(checkin.checkInDate).toLocaleTimeString('th-TH', {
                        hour: '2-digit',
                        minute: '2-digit',
                        timeZone: 'UTC',
                      })}
                      {' - '}
                      {new Date(checkin.checkOutDate).toLocaleDateString('th-TH', {
                        day: 'numeric',
                        month: 'short',
                        timeZone: 'UTC',
                      })}
                      {' '}
                      {new Date(checkin.checkOutDate).toLocaleTimeString('th-TH', {
                        hour: '2-digit',
                        minute: '2-digit',
                        timeZone: 'UTC',
                      })}
                    </p>
                    <span className="text-xs text-green-600 bg-green-100 px-2 py-1 rounded-full">
                      {checkin.status || 'เช็คอิน'}
                    </span>
                  </div>
                </div>
              ))
            ) : (
              <p className="text-gray-500 text-center py-4">ไม่มีกิจกรรมล่าสุด</p>
            )}
          </div>
        </div>
      </div>

      {/* Quick Check-In Modal */}
      {selectedRoomForModal && (
        <QuickCheckInModal
          isOpen={showCheckInModal}
          onClose={() => {
            setShowCheckInModal(false)
            setSelectedRoomForModal(null)
          }}
          room={selectedRoomForModal}
          onSuccess={handleModalSuccess}
        />
      )}

      {/* Check-Out Modal */}
      <CheckOutModal
        isOpen={showCheckOutModal}
        onClose={() => {
          setShowCheckOutModal(false)
          setSelectedCheckIn(null)
        }}
        checkIn={selectedCheckIn}
        onSuccess={handleModalSuccess}
      />
    </div>
  )
}
