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
} from 'date-fns'
import { ChevronLeft, ChevronRight, Calendar } from 'lucide-react'

export interface Stay {
  id: string
  checkIn: Date
  checkOut: Date
  type: 'booking' | 'checkin'
  nights: number
}

interface AggregatedStay {
  id: string
  checkIn: Date
  checkOut: Date
  nights: number
  count: number
  checkinCount: number
  bookingCount: number
}

interface StayTimelineProps {
  stays: Stay[]
  selectedMonth: Date
  onMonthChange: (date: Date) => void
}

const THAI_MONTHS = [
  'มกราคม', 'กุมภาพันธ์', 'มีนาคม', 'เมษายน', 'พฤษภาคม', 'มิถุนายน',
  'กรกฎาคม', 'สิงหาคม', 'กันยายน', 'ตุลาคม', 'พฤศจิกายน', 'ธันวาคม',
]

// Color based on stay length
function getStayColor(nights: number): { bg: string; text: string } {
  if (nights === 1) return { bg: 'bg-sky-400', text: 'text-white' }
  if (nights === 2) return { bg: 'bg-blue-500', text: 'text-white' }
  if (nights <= 4) return { bg: 'bg-indigo-500', text: 'text-white' }
  if (nights <= 7) return { bg: 'bg-violet-500', text: 'text-white' }
  return { bg: 'bg-purple-600', text: 'text-white' }
}

export default function StayTimeline({
  stays,
  selectedMonth,
  onMonthChange,
}: StayTimelineProps) {
  const [hoveredStay, setHoveredStay] = useState<string | null>(null)

  const monthStart = startOfMonth(selectedMonth)
  const monthEnd = endOfMonth(selectedMonth)
  const daysInMonth = eachDayOfInterval({ start: monthStart, end: monthEnd })

  // Aggregate stays by check-in and check-out dates
  const aggregatedStays: AggregatedStay[] = useMemo(() => {
    const groups = new Map<string, AggregatedStay>()

    stays.forEach(stay => {
      const key = `${format(stay.checkIn, 'yyyy-MM-dd')}_${format(stay.checkOut, 'yyyy-MM-dd')}`

      if (!groups.has(key)) {
        groups.set(key, {
          id: key,
          checkIn: stay.checkIn,
          checkOut: stay.checkOut,
          nights: stay.nights,
          count: 0,
          checkinCount: 0,
          bookingCount: 0,
        })
      }

      const group = groups.get(key)!
      group.count++
      if (stay.type === 'checkin') {
        group.checkinCount++
      } else {
        group.bookingCount++
      }
    })

    // Sort by check-in date, then by duration (longer first)
    return Array.from(groups.values()).sort((a, b) => {
      const dateCompare = a.checkIn.getTime() - b.checkIn.getTime()
      if (dateCompare !== 0) return dateCompare
      return b.nights - a.nights
    })
  }, [stays])

  // Calculate daily occupancy
  const dailyOccupancy = useMemo(() => {
    const occupancy: Record<string, number> = {}
    daysInMonth.forEach(day => {
      const dateKey = format(day, 'yyyy-MM-dd')
      occupancy[dateKey] = stays.filter(stay => {
        return isWithinInterval(day, { start: stay.checkIn, end: stay.checkOut }) ||
               isSameDay(day, stay.checkIn) || isSameDay(day, stay.checkOut)
      }).length
    })
    return occupancy
  }, [stays, daysInMonth])

  const maxOccupancy = Math.max(...Object.values(dailyOccupancy), 1)

  // Calculate stay bar position and width
  const getStayStyle = (stay: AggregatedStay) => {
    const totalDays = daysInMonth.length
    const stayStart = Math.max(stay.checkIn.getTime(), monthStart.getTime())
    const stayEnd = Math.min(stay.checkOut.getTime(), monthEnd.getTime())

    const startOffset = differenceInDays(new Date(stayStart), monthStart)
    const duration = differenceInDays(new Date(stayEnd), new Date(stayStart)) + 1

    const leftPercent = (startOffset / totalDays) * 100
    const widthPercent = (duration / totalDays) * 100

    return {
      left: `${leftPercent}%`,
      width: `${Math.max(widthPercent, 3)}%`,
    }
  }

  // Check if stay overlaps with the current month
  const stayOverlapsMonth = (stay: AggregatedStay) => {
    return stay.checkOut >= monthStart && stay.checkIn <= monthEnd
  }

  const visibleStays = aggregatedStays.filter(stayOverlapsMonth)

  // Calculate stats
  const stats = useMemo(() => {
    const totalStays = stays.length
    const checkins = stays.filter(s => s.type === 'checkin').length
    const bookings = stays.filter(s => s.type === 'booking').length
    const totalNights = stays.reduce((sum, s) => sum + s.nights, 0)
    const avgStay = totalStays > 0 ? (totalNights / totalStays).toFixed(1) : '0'

    // Stay length distribution
    const lengthDist: Record<string, number> = {}
    stays.forEach(s => {
      const key = s.nights === 1 ? '1 คืน' : s.nights <= 3 ? '2-3 คืน' : s.nights <= 7 ? '4-7 คืน' : '7+ คืน'
      lengthDist[key] = (lengthDist[key] || 0) + 1
    })

    return { totalStays, checkins, bookings, totalNights, avgStay, lengthDist }
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
        <button onClick={prevMonth} className="p-2 hover:bg-gray-200 rounded-full">
          <ChevronLeft className="w-5 h-5" />
        </button>
        <h2 className="text-xl font-semibold text-gray-800">
          {THAI_MONTHS[selectedMonth.getMonth()]} {selectedMonth.getFullYear() + 543}
        </h2>
        <button onClick={nextMonth} className="p-2 hover:bg-gray-200 rounded-full">
          <ChevronRight className="w-5 h-5" />
        </button>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-2 md:grid-cols-5 gap-4 p-4 bg-gray-50 border-b">
        <div className="text-center">
          <p className="text-2xl font-bold text-gray-900">{stats.totalStays}</p>
          <p className="text-xs text-gray-500">รวมทั้งหมด</p>
        </div>
        <div className="text-center">
          <p className="text-2xl font-bold text-blue-600">{stats.checkins}</p>
          <p className="text-xs text-gray-500">เช็คอิน</p>
        </div>
        <div className="text-center">
          <p className="text-2xl font-bold text-amber-600">{stats.bookings}</p>
          <p className="text-xs text-gray-500">การจอง</p>
        </div>
        <div className="text-center">
          <p className="text-2xl font-bold text-gray-900">{stats.totalNights}</p>
          <p className="text-xs text-gray-500">คืนรวม</p>
        </div>
        <div className="text-center">
          <p className="text-2xl font-bold text-gray-900">{stats.avgStay}</p>
          <p className="text-xs text-gray-500">คืน/ครั้ง</p>
        </div>
      </div>

      {/* Stay Length Distribution */}
      <div className="px-4 py-3 border-b flex items-center gap-4 text-sm">
        <span className="text-gray-500">การกระจายตัว:</span>
        {Object.entries(stats.lengthDist).map(([label, count]) => (
          <span key={label} className="px-2 py-1 bg-gray-100 rounded text-gray-700">
            {label}: <strong>{count}</strong>
          </span>
        ))}
      </div>

      {/* Occupancy Heat Bar */}
      <div className="px-4 py-3 border-b">
        <p className="text-xs text-gray-500 mb-2">ความหนาแน่นรายวัน</p>
        <div className="flex h-10 rounded overflow-hidden">
          {daysInMonth.map(day => {
            const dateKey = format(day, 'yyyy-MM-dd')
            const count = dailyOccupancy[dateKey] || 0
            const intensity = count / maxOccupancy
            const isToday = isSameDay(day, new Date())
            const isWeekend = day.getDay() === 0 || day.getDay() === 6

            return (
              <div
                key={dateKey}
                className={`flex-1 flex flex-col items-center justify-end relative group
                  ${isToday ? 'ring-2 ring-blue-500 ring-inset z-10' : ''}
                  ${isWeekend ? 'bg-gray-50' : ''}`}
                style={{
                  backgroundColor: count > 0
                    ? `rgba(59, 130, 246, ${0.15 + intensity * 0.7})`
                    : isWeekend ? '#f9fafb' : '#f3f4f6'
                }}
              >
                <span className="text-[10px] text-gray-500 mb-0.5">{count || ''}</span>
                <span className={`text-[10px] ${isToday ? 'font-bold text-blue-600' : 'text-gray-600'}`}>
                  {day.getDate()}
                </span>
                {/* Tooltip */}
                <div className="absolute bottom-full mb-1 hidden group-hover:block bg-gray-800 text-white text-xs px-2 py-1 rounded whitespace-nowrap z-20">
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
        <div className="flex mb-2 text-xs text-gray-400">
          <div className="w-28 flex-shrink-0 text-right pr-3">ระยะเวลา</div>
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
        <div className="space-y-1">
          {visibleStays.length === 0 ? (
            <div className="text-center py-12 text-gray-500">
              <Calendar className="w-12 h-12 mx-auto mb-3 text-gray-300" />
              <p>ไม่มีข้อมูลการเข้าพักในเดือนนี้</p>
            </div>
          ) : (
            visibleStays.map(stay => {
              const style = getStayStyle(stay)
              const color = getStayColor(stay.nights)
              const isHovered = hoveredStay === stay.id

              return (
                <div
                  key={stay.id}
                  className="flex items-center"
                  onMouseEnter={() => setHoveredStay(stay.id)}
                  onMouseLeave={() => setHoveredStay(null)}
                >
                  {/* Stay info */}
                  <div className="w-28 flex-shrink-0 text-right pr-3">
                    <span className="text-sm font-medium text-gray-700">
                      {stay.nights} คืน
                    </span>
                    <span className="text-xs text-gray-400 ml-1">
                      x{stay.count}
                    </span>
                  </div>

                  {/* Timeline bar */}
                  <div className="flex-1 relative h-7">
                    <div
                      className={`absolute top-0.5 h-6 rounded flex items-center justify-center px-2 text-xs font-medium cursor-default
                        ${color.bg} ${color.text} ${isHovered ? 'ring-2 ring-offset-1 ring-gray-400 z-10' : ''}`}
                      style={style}
                      title={`${format(stay.checkIn, 'd MMM')} - ${format(stay.checkOut, 'd MMM')} | ${stay.nights} คืน | ${stay.count} ครั้ง`}
                    >
                      {stay.count > 1 && <span>{stay.count}</span>}
                    </div>
                  </div>
                </div>
              )
            })
          )}
        </div>
      </div>

      {/* Legend */}
      <div className="flex flex-wrap items-center gap-4 p-4 bg-gray-50 border-t text-xs">
        <span className="text-gray-500">ระยะเวลาเข้าพัก:</span>
        <div className="flex items-center gap-1">
          <div className="w-4 h-4 bg-sky-400 rounded" />
          <span>1 คืน</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-4 h-4 bg-blue-500 rounded" />
          <span>2 คืน</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-4 h-4 bg-indigo-500 rounded" />
          <span>3-4 คืน</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-4 h-4 bg-violet-500 rounded" />
          <span>5-7 คืน</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-4 h-4 bg-purple-600 rounded" />
          <span>7+ คืน</span>
        </div>
      </div>
    </div>
  )
}
