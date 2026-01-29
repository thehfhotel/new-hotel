'use client'

import { useMemo, useState } from 'react'
import {
  format,
  startOfMonth,
  endOfMonth,
  eachDayOfInterval,
  differenceInDays,
  isSameDay,
  isWithinInterval,
  parseISO,
} from 'date-fns'
import { ChevronLeft, ChevronRight, Users, Calendar } from 'lucide-react'

export interface Stay {
  id: string
  customerName: string
  roomNumber: string
  checkIn: Date
  checkOut: Date
  type: 'booking' | 'checkin'
  status?: string
  nights: number
}

interface StayTimelineProps {
  stays: Stay[]
  selectedMonth: Date
  onMonthChange: (date: Date) => void
  onStayClick?: (stay: Stay) => void
}

const THAI_MONTHS = [
  'มกราคม', 'กุมภาพันธ์', 'มีนาคม', 'เมษายน', 'พฤษภาคม', 'มิถุนายน',
  'กรกฎาคม', 'สิงหาคม', 'กันยายน', 'ตุลาคม', 'พฤศจิกายน', 'ธันวาคม',
]

// Color palette for stays
const STAY_COLORS = [
  { bg: 'bg-blue-500', hover: 'hover:bg-blue-600', light: 'bg-blue-100' },
  { bg: 'bg-emerald-500', hover: 'hover:bg-emerald-600', light: 'bg-emerald-100' },
  { bg: 'bg-violet-500', hover: 'hover:bg-violet-600', light: 'bg-violet-100' },
  { bg: 'bg-amber-500', hover: 'hover:bg-amber-600', light: 'bg-amber-100' },
  { bg: 'bg-rose-500', hover: 'hover:bg-rose-600', light: 'bg-rose-100' },
  { bg: 'bg-cyan-500', hover: 'hover:bg-cyan-600', light: 'bg-cyan-100' },
  { bg: 'bg-orange-500', hover: 'hover:bg-orange-600', light: 'bg-orange-100' },
  { bg: 'bg-indigo-500', hover: 'hover:bg-indigo-600', light: 'bg-indigo-100' },
]

export default function StayTimeline({
  stays,
  selectedMonth,
  onMonthChange,
  onStayClick,
}: StayTimelineProps) {
  const [hoveredStay, setHoveredStay] = useState<string | null>(null)

  const monthStart = startOfMonth(selectedMonth)
  const monthEnd = endOfMonth(selectedMonth)
  const daysInMonth = eachDayOfInterval({ start: monthStart, end: monthEnd })

  // Calculate daily occupancy
  const dailyOccupancy = useMemo(() => {
    const occupancy: Record<string, number> = {}
    daysInMonth.forEach(day => {
      const dateKey = format(day, 'yyyy-MM-dd')
      occupancy[dateKey] = stays.filter(stay => {
        const checkIn = stay.checkIn
        const checkOut = stay.checkOut
        return isWithinInterval(day, { start: checkIn, end: checkOut }) ||
               isSameDay(day, checkIn) || isSameDay(day, checkOut)
      }).length
    })
    return occupancy
  }, [stays, daysInMonth])

  // Find max occupancy for scaling
  const maxOccupancy = Math.max(...Object.values(dailyOccupancy), 1)

  // Sort stays by check-in date, then by duration (longer stays first)
  const sortedStays = useMemo(() => {
    return [...stays].sort((a, b) => {
      const dateCompare = a.checkIn.getTime() - b.checkIn.getTime()
      if (dateCompare !== 0) return dateCompare
      return b.nights - a.nights
    })
  }, [stays])

  // Assign colors to stays
  const stayColors = useMemo(() => {
    const colors: Record<string, typeof STAY_COLORS[0]> = {}
    sortedStays.forEach((stay, index) => {
      colors[stay.id] = STAY_COLORS[index % STAY_COLORS.length]
    })
    return colors
  }, [sortedStays])

  // Calculate stay bar position and width
  const getStayStyle = (stay: Stay) => {
    const monthStartTime = monthStart.getTime()
    const monthEndTime = monthEnd.getTime()
    const totalDays = daysInMonth.length

    const stayStart = Math.max(stay.checkIn.getTime(), monthStartTime)
    const stayEnd = Math.min(stay.checkOut.getTime(), monthEndTime)

    const startOffset = differenceInDays(new Date(stayStart), monthStart)
    const duration = differenceInDays(new Date(stayEnd), new Date(stayStart)) + 1

    const leftPercent = (startOffset / totalDays) * 100
    const widthPercent = (duration / totalDays) * 100

    return {
      left: `${leftPercent}%`,
      width: `${Math.max(widthPercent, 2)}%`, // Min 2% width for visibility
    }
  }

  // Check if stay overlaps with the current month
  const stayOverlapsMonth = (stay: Stay) => {
    return stay.checkOut >= monthStart && stay.checkIn <= monthEnd
  }

  const visibleStays = sortedStays.filter(stayOverlapsMonth)

  // Calculate stats
  const stats = useMemo(() => {
    const bookings = stays.filter(s => s.type === 'booking').length
    const checkins = stays.filter(s => s.type === 'checkin').length
    const totalNights = stays.reduce((sum, s) => sum + s.nights, 0)
    const avgStay = stays.length > 0 ? (totalNights / stays.length).toFixed(1) : '0'
    return { bookings, checkins, totalNights, avgStay }
  }, [stays])

  const prevMonth = () => {
    onMonthChange(new Date(selectedMonth.getFullYear(), selectedMonth.getMonth() - 1, 1))
  }

  const nextMonth = () => {
    onMonthChange(new Date(selectedMonth.getFullYear(), selectedMonth.getMonth() + 1, 1))
  }

  return (
    <div className="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b bg-gray-50">
        <button
          onClick={prevMonth}
          className="p-2 hover:bg-gray-200 rounded-full"
          aria-label="เดือนก่อนหน้า"
        >
          <ChevronLeft className="w-5 h-5" />
        </button>
        <h2 className="text-xl font-semibold text-gray-800">
          {THAI_MONTHS[selectedMonth.getMonth()]} {selectedMonth.getFullYear() + 543}
        </h2>
        <button
          onClick={nextMonth}
          className="p-2 hover:bg-gray-200 rounded-full"
          aria-label="เดือนถัดไป"
        >
          <ChevronRight className="w-5 h-5" />
        </button>
      </div>

      {/* Stats Bar */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4 p-4 bg-gray-50 border-b">
        <div className="text-center">
          <p className="text-2xl font-bold text-gray-900">{stats.checkins}</p>
          <p className="text-xs text-gray-500">เช็คอิน</p>
        </div>
        <div className="text-center">
          <p className="text-2xl font-bold text-gray-900">{stats.bookings}</p>
          <p className="text-xs text-gray-500">การจอง</p>
        </div>
        <div className="text-center">
          <p className="text-2xl font-bold text-gray-900">{stats.totalNights}</p>
          <p className="text-xs text-gray-500">คืนรวม</p>
        </div>
        <div className="text-center">
          <p className="text-2xl font-bold text-gray-900">{stats.avgStay}</p>
          <p className="text-xs text-gray-500">คืน/ครั้ง (เฉลี่ย)</p>
        </div>
      </div>

      {/* Occupancy Heat Bar */}
      <div className="px-4 py-3 border-b">
        <p className="text-xs text-gray-500 mb-2">ความหนาแน่น (จำนวนห้อง/วัน)</p>
        <div className="flex h-8 rounded overflow-hidden">
          {daysInMonth.map(day => {
            const dateKey = format(day, 'yyyy-MM-dd')
            const count = dailyOccupancy[dateKey] || 0
            const intensity = count / maxOccupancy
            const isToday = isSameDay(day, new Date())

            return (
              <div
                key={dateKey}
                className={`flex-1 flex items-end justify-center relative group ${isToday ? 'ring-2 ring-blue-500 ring-inset' : ''}`}
                style={{
                  backgroundColor: count > 0
                    ? `rgba(59, 130, 246, ${0.2 + intensity * 0.6})`
                    : '#f3f4f6'
                }}
              >
                <span className="text-[10px] text-gray-600">{day.getDate()}</span>
                {/* Tooltip */}
                <div className="absolute bottom-full mb-1 hidden group-hover:block bg-gray-800 text-white text-xs px-2 py-1 rounded whitespace-nowrap z-10">
                  {format(day, 'd MMM')}: {count} ห้อง
                </div>
              </div>
            )
          })}
        </div>
      </div>

      {/* Timeline */}
      <div className="p-4">
        {/* Day headers */}
        <div className="flex mb-2 text-xs text-gray-500">
          <div className="w-48 flex-shrink-0"></div>
          <div className="flex-1 flex">
            {daysInMonth.map(day => (
              <div
                key={day.toISOString()}
                className={`flex-1 text-center ${isSameDay(day, new Date()) ? 'font-bold text-blue-600' : ''}`}
              >
                {day.getDate()}
              </div>
            ))}
          </div>
        </div>

        {/* Stay bars */}
        <div className="space-y-2">
          {visibleStays.length === 0 ? (
            <div className="text-center py-12 text-gray-500">
              <Calendar className="w-12 h-12 mx-auto mb-3 text-gray-300" />
              <p>ไม่มีข้อมูลการเข้าพักในเดือนนี้</p>
            </div>
          ) : (
            visibleStays.map(stay => {
              const style = getStayStyle(stay)
              const color = stayColors[stay.id]
              const isHovered = hoveredStay === stay.id

              return (
                <div
                  key={stay.id}
                  className="flex items-center group"
                  onMouseEnter={() => setHoveredStay(stay.id)}
                  onMouseLeave={() => setHoveredStay(null)}
                >
                  {/* Guest info */}
                  <div className="w-48 flex-shrink-0 pr-3">
                    <div className="flex items-center gap-2">
                      <div className={`w-2 h-2 rounded-full ${stay.type === 'checkin' ? 'bg-blue-500' : 'bg-amber-500'}`} />
                      <div className="truncate">
                        <p className="text-sm font-medium text-gray-800 truncate">
                          {stay.customerName || 'ไม่ระบุชื่อ'}
                        </p>
                        <p className="text-xs text-gray-500">
                          {stay.roomNumber && `ห้อง ${stay.roomNumber} · `}{stay.nights} คืน
                        </p>
                      </div>
                    </div>
                  </div>

                  {/* Timeline bar */}
                  <div className="flex-1 relative h-8">
                    <div
                      onClick={() => onStayClick?.(stay)}
                      className={`absolute top-1 h-6 rounded cursor-pointer flex items-center px-2 text-white text-xs font-medium truncate
                        ${color.bg} ${color.hover} ${isHovered ? 'ring-2 ring-offset-1 ring-gray-400' : ''}`}
                      style={style}
                      title={`${stay.customerName} | ${format(stay.checkIn, 'd MMM')} - ${format(stay.checkOut, 'd MMM')} (${stay.nights} คืน)`}
                    >
                      <span className="truncate">
                        {stay.nights > 1 ? `${stay.nights} คืน` : ''}
                      </span>
                    </div>
                  </div>
                </div>
              )
            })
          )}
        </div>
      </div>

      {/* Legend */}
      <div className="flex items-center gap-6 p-4 bg-gray-50 border-t text-sm">
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 bg-blue-500 rounded-full" />
          <span className="text-gray-600">เช็คอินแล้ว</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 bg-amber-500 rounded-full" />
          <span className="text-gray-600">การจอง</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 border-2 border-blue-500 rounded" />
          <span className="text-gray-600">วันนี้</span>
        </div>
      </div>
    </div>
  )
}
