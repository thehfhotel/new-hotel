'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  Loader2,
  AlertCircle,
  Clock,
  LogIn,
} from 'lucide-react'

interface Stats {
  totalRooms: number
  occupiedRooms: number
  availableRooms: number
  bookedRooms: number
  checkoutRooms: number
  totalCustomers: number
  activeBookings: number
  todayCheckIns: number
  todayCheckOuts: number
}

interface ApiRoom {
  Room_no: string
  Room_Type: string
  Room_Details: string
  Room_Clean: string
  Room_Use: string
  Room_Book: string
  Room_Manternace: string
}

type RoomStatus = 'available' | 'occupied' | 'booked' | 'maintenance' | 'cleaning' | 'checkout'

interface Room {
  roomNumber: string
  type: string
  details: string
  status: RoomStatus
}

interface CheckIn {
  guestName: string
  roomNumber: string
  checkInDate: string
  checkOutDate: string
}

const statusConfig: Record<RoomStatus, { dot: string; bg: string; border: string; label: string }> = {
  available: { dot: 'bg-emerald-500', bg: 'bg-emerald-500/10 hover:bg-emerald-500/20', border: 'border-emerald-500/30', label: 'ว่าง' },
  occupied: { dot: 'bg-red-500', bg: 'bg-red-500/10 hover:bg-red-500/20', border: 'border-red-500/30', label: 'มีผู้เข้าพัก' },
  booked: { dot: 'bg-amber-500', bg: 'bg-amber-500/10 hover:bg-amber-500/20', border: 'border-amber-500/30', label: 'จองแล้ว' },
  maintenance: { dot: 'bg-zinc-500', bg: 'bg-zinc-800 hover:bg-zinc-700', border: 'border-zinc-600', label: 'ซ่อมบำรุง' },
  cleaning: { dot: 'bg-orange-500', bg: 'bg-orange-500/10 hover:bg-orange-500/20', border: 'border-orange-500/30', label: 'ทำความสะอาด' },
  checkout: { dot: 'bg-sky-500', bg: 'bg-sky-500/10 hover:bg-sky-500/20', border: 'border-sky-500/30', label: 'รอเช็คเอาท์' },
}

// Custom room layout matching actual hotel floor plan
const roomLayout: (string | null)[][] = [
  ['509', '510', '511', '512', '513', '514', '515', '516', '517', '518'],
  ['508', '507', null, '506', '505', '504', '503', '502', '501', null, null, 'A4-1', 'A4-2', 'A4-3'],
  ['409', '410', '411', '412', '413', '414', '415', '416', '417', '418', null, 'A3-1', 'A3-2', 'A3-3'],
  ['408', '407', null, '406', '405', '404', '403', '402', '401', null, null, 'V.201', 'A2-1', 'A2-3'],
  ['307', '308', '309', '310', '311', '312', '313'],
  ['306', '305', null, '304', '303', '302', '301'],
]

function getRoomStatus(room: ApiRoom, isCheckoutToday: boolean): RoomStatus {
  const hour = new Date().getHours()
  if (isCheckoutToday && hour >= 6) return 'checkout'
  if (room.Room_Manternace === 'yes') return 'maintenance'
  if (room.Room_Use === 'yes') return 'occupied'
  if (room.Room_Book && room.Room_Book !== '') return 'booked'
  return 'available'
}

export default function NewDashboard() {
  const [stats, setStats] = useState<Stats>({
    totalRooms: 0, occupiedRooms: 0, availableRooms: 0, bookedRooms: 0,
    checkoutRooms: 0, totalCustomers: 0, activeBookings: 0, todayCheckIns: 0, todayCheckOuts: 0,
  })
  const [rooms, setRooms] = useState<Room[]>([])
  const [checkIns, setCheckIns] = useState<CheckIn[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [selectedRoom, setSelectedRoom] = useState<Room | null>(null)

  const fetchData = useCallback(async () => {
    try {
      const [statsRes, roomsRes, checkoutsRes, checkInsRes] = await Promise.all([
        fetch('/api/stats'),
        fetch('/api/rooms'),
        fetch('/api/rooms/checkouts-today'),
        fetch('/api/checkins?limit=10'),
      ])

      if (statsRes.ok) {
        const data = await statsRes.json()
        if (data.success) {
          const d = data.data
          setStats({
            ...d,
            availableRooms: d.totalRooms - d.occupiedRooms - d.checkoutRooms - d.bookedRooms,
          })
        }
      }

      let checkoutSet = new Set<string>()
      if (checkoutsRes.ok) {
        const data = await checkoutsRes.json()
        if (data.success && data.data) checkoutSet = new Set(data.data)
      }

      if (roomsRes.ok) {
        const data = await roomsRes.json()
        if (data.success && data.data) {
          setRooms(data.data.map((r: ApiRoom) => ({
            roomNumber: r.Room_no,
            type: r.Room_Type?.trim() || '',
            details: r.Room_Details?.trim() || '',
            status: getRoomStatus(r, checkoutSet.has(r.Room_no)),
          })))
        }
      }

      if (checkInsRes.ok) {
        const data = await checkInsRes.json()
        if (data.success && data.data) {
          setCheckIns(data.data.map((c: { Cin_cust_name: string; Cin_Room_No: string; Cin_Room_In: string; Cin_Room_Out: string }) => ({
            guestName: c.Cin_cust_name?.trim() || 'ไม่ระบุ',
            roomNumber: c.Cin_Room_No,
            checkInDate: c.Cin_Room_In,
            checkOutDate: c.Cin_Room_Out,
          })))
        }
      }
    } catch (err) {
      setError('ไม่สามารถโหลดข้อมูลได้')
      console.error('Error fetching dashboard data:', err)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    fetchData()
  }, [fetchData])

  const roomMap = new Map<string, Room>()
  rooms.forEach(r => roomMap.set(r.roomNumber.toUpperCase(), r))

  const maxColumns = Math.max(...roomLayout.map(row => row.length))

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-[50vh]">
        <div className="text-center">
          <Loader2 className="animate-spin h-12 w-12 text-red-500 mx-auto mb-4" />
          <p className="text-zinc-500">กำลังโหลดข้อมูล...</p>
        </div>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold text-zinc-100 tracking-tight">หน้าหลัก</h1>
        <p className="text-zinc-600 text-sm">
          อัปเดตล่าสุด: {new Date().toLocaleDateString('th-TH', {
            year: 'numeric', month: 'long', day: 'numeric', hour: '2-digit', minute: '2-digit',
          })}
        </p>
      </div>

      {error && (
        <div className="flex items-center gap-2 p-4 bg-red-950/50 border border-red-900/50 rounded-lg text-red-400">
          <AlertCircle className="w-5 h-5 flex-shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {/* Stats Cards */}
      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
        <div className="bg-zinc-900 rounded-xl px-6 py-5 border border-zinc-800">
          <p className="text-sm font-medium text-zinc-500 mb-1">ห้องทั้งหมด</p>
          <p className="text-3xl font-bold text-zinc-100">{stats.totalRooms}</p>
        </div>
        <div className="bg-zinc-900 rounded-xl px-6 py-5 border border-zinc-800">
          <p className="text-sm font-medium text-zinc-500 mb-1">ห้องว่าง</p>
          <p className="text-3xl font-bold text-emerald-400">{stats.availableRooms}</p>
        </div>
        <div className="bg-zinc-900 rounded-xl px-6 py-5 border border-zinc-800">
          <p className="text-sm font-medium text-zinc-500 mb-1">มีผู้เข้าพัก</p>
          <p className="text-3xl font-bold text-red-400">{stats.occupiedRooms}</p>
        </div>
        <div className="bg-zinc-900 rounded-xl px-6 py-5 border border-zinc-800">
          <p className="text-sm font-medium text-zinc-500 mb-1">จองแล้ว</p>
          <p className="text-3xl font-bold text-amber-400">{stats.bookedRooms}</p>
        </div>
        <div className="bg-zinc-900 rounded-xl px-6 py-5 border border-zinc-800">
          <p className="text-sm font-medium text-zinc-500 mb-1">เช็คอินวันนี้</p>
          <p className="text-3xl font-bold text-zinc-100">{stats.todayCheckIns}</p>
        </div>
        <div className="bg-zinc-900 rounded-xl px-6 py-5 border border-zinc-800">
          <p className="text-sm font-medium text-zinc-500 mb-1">เช็คเอาท์วันนี้</p>
          <p className="text-3xl font-bold text-sky-400">{stats.todayCheckOuts}</p>
        </div>
      </div>

      {/* Room Grid */}
      <div className="bg-zinc-900 rounded-xl border border-zinc-800 p-6">
        <h2 className="text-lg font-semibold text-zinc-200 mb-4">สถานะห้องพัก</h2>

        {/* Desktop Grid */}
        <div className="hidden md:block">
          <div className="grid gap-1.5" style={{ gridTemplateColumns: `repeat(${maxColumns}, minmax(0, 1fr))` }}>
            {roomLayout.map((row, rowIndex) => (
              <div key={`row-${rowIndex}`} className="contents">
                {row.map((roomNumber, colIndex) => {
                  if (roomNumber === null) {
                    return <div key={`blank-${rowIndex}-${colIndex}`} className="h-[60px]" />
                  }
                  const room = roomMap.get(roomNumber.toUpperCase())
                  if (!room) {
                    return (
                      <div key={`missing-${roomNumber}`} className="h-[60px] bg-zinc-800/50 rounded-lg flex flex-col items-center justify-center">
                        <span className="font-bold text-[10px] text-zinc-600">{roomNumber}</span>
                      </div>
                    )
                  }
                  const config = statusConfig[room.status]
                  return (
                    <button
                      key={roomNumber}
                      onClick={() => setSelectedRoom(room)}
                      className={`${config.bg} border ${config.border} rounded-lg p-1 flex flex-col items-center justify-center h-[60px] transition-colors`}
                      title={`${room.roomNumber} - ${room.type} ${room.details}`}
                    >
                      <span className="font-bold text-[11px] text-zinc-100">{room.roomNumber}</span>
                      <span className="text-[8px] text-zinc-400">{room.type}</span>
                    </button>
                  )
                })}
                {Array.from({ length: maxColumns - row.length }).map((_, i) => (
                  <div key={`filler-${rowIndex}-${i}`} className="h-[60px]" />
                ))}
              </div>
            ))}
          </div>
        </div>

        {/* Mobile List */}
        <div className="md:hidden space-y-1.5">
          {[...rooms].sort((a, b) => a.roomNumber.localeCompare(b.roomNumber, undefined, { numeric: true })).map(room => {
            const config = statusConfig[room.status]
            return (
              <button
                key={room.roomNumber}
                onClick={() => setSelectedRoom(room)}
                className={`${config.bg} w-full flex items-center gap-3 p-3 rounded-lg border ${config.border}`}
              >
                <div className={`w-2.5 h-2.5 rounded-full ${config.dot}`} />
                <span className="font-bold text-sm text-zinc-100">{room.roomNumber}</span>
                <span className="text-zinc-400 text-sm">{room.type}</span>
              </button>
            )
          })}
        </div>

        {/* Legend */}
        <div className="flex flex-wrap gap-4 mt-4 pt-4 border-t border-zinc-800">
          {Object.entries(statusConfig).map(([status, config]) => (
            <div key={status} className="flex items-center gap-2">
              <div className={`w-2.5 h-2.5 rounded-full ${config.dot}`} />
              <span className="text-xs text-zinc-400">{config.label}</span>
            </div>
          ))}
        </div>
      </div>

      {/* Recent Activity */}
      <div className="bg-zinc-900 rounded-xl border border-zinc-800 p-6">
        <h3 className="text-lg font-semibold text-zinc-200 mb-4 flex items-center gap-2">
          <Clock size={20} className="text-red-500" />
          กิจกรรมล่าสุด
        </h3>
        <div className="space-y-2">
          {checkIns.length > 0 ? (
            checkIns.slice(0, 5).map((checkin, i) => (
              <div key={i} className="flex items-center justify-between p-3 bg-zinc-800/50 rounded-lg">
                <div className="flex items-center gap-3">
                  <div className="w-9 h-9 bg-red-500/10 rounded-full flex items-center justify-center">
                    <LogIn size={16} className="text-red-400" />
                  </div>
                  <div>
                    <p className="font-medium text-zinc-200 text-sm">{checkin.guestName}</p>
                    <p className="text-xs text-zinc-500">ห้อง {checkin.roomNumber}</p>
                  </div>
                </div>
                <div className="text-right">
                  <p className="text-xs text-zinc-400">
                    {new Date(checkin.checkInDate).toLocaleDateString('th-TH', { day: 'numeric', month: 'short', timeZone: 'UTC' })}
                    {' - '}
                    {new Date(checkin.checkOutDate).toLocaleDateString('th-TH', { day: 'numeric', month: 'short', timeZone: 'UTC' })}
                  </p>
                </div>
              </div>
            ))
          ) : (
            <p className="text-zinc-500 text-center py-4">ไม่มีกิจกรรมล่าสุด</p>
          )}
        </div>
      </div>

      {/* Room Detail Modal */}
      {selectedRoom && (
        <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50" onClick={() => setSelectedRoom(null)}>
          <div className="bg-zinc-900 border border-zinc-800 rounded-xl shadow-2xl p-6 max-w-sm w-full mx-4" onClick={e => e.stopPropagation()}>
            <div className="flex justify-between items-start mb-4">
              <div>
                <h3 className="text-2xl font-bold text-zinc-100">ห้อง {selectedRoom.roomNumber}</h3>
                <p className="text-zinc-400">{selectedRoom.type} {selectedRoom.details}</p>
              </div>
              <span className={`flex items-center gap-1.5 px-3 py-1 rounded-full text-xs ${statusConfig[selectedRoom.status].bg} border ${statusConfig[selectedRoom.status].border}`}>
                <span className={`w-2 h-2 rounded-full ${statusConfig[selectedRoom.status].dot}`} />
                {statusConfig[selectedRoom.status].label}
              </span>
            </div>
            <button
              onClick={() => setSelectedRoom(null)}
              className="w-full mt-4 bg-zinc-800 hover:bg-zinc-700 text-zinc-200 py-2 rounded-lg transition-colors"
            >
              ปิด
            </button>
          </div>
        </div>
      )}
    </div>
  )
}
