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

interface StayBar {
  id: string
  checkIn: Date
  checkOut: Date
  nights: number
  count: number
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

const NIGHT_COLORS: Record<number, string> = {
  1: 'bg-sky-400',
  2: 'bg-blue-500',
  3: 'bg-indigo-500',
  4: 'bg-violet-500',
  5: 'bg-purple-500',
  6: 'bg-purple-600',
  7: 'bg-fuchsia-600',
}

function getNightColor(nights: number): string {
  if (nights >= 7) return 'bg-fuchsia-600'
  return NIGHT_COLORS[nights] || 'bg-gray-400'
}

export default function StayTimeline({
  stays,
  selectedMonth,
  onMonthChange,
}: StayTimelineProps) {
  const [hoveredBar, setHoveredBar] = useState<string | null>(null)

  const monthStart = startOfMonth(selectedMonth)
  const monthEnd = endOfMonth(selectedMonth)
  const daysInMonth = eachDayOfInterval({ start: monthStart, end: monthEnd })
  const totalDays = daysInMonth.length

  // Group stays by night count, then aggregate by check-in date
  const staysByNight = useMemo(() => {
    const groups: Record<number, StayBar[]> = {}

    // First, aggregate stays by (nights, checkIn date)
    const aggregated = new Map<string, StayBar>()

    stays.forEach(stay => {
      // Only include stays that overlap with current month
      if (stay.checkOut < monthStart || stay.checkIn > monthEnd) return

      const key = `${stay.nights}_${format(stay.checkIn, 'yyyy-MM-dd')}`

      if (!aggregated.has(key)) {
        aggregated.set(key, {
          id: key,
          checkIn: stay.checkIn,
          checkOut: stay.checkOut,
          nights: stay.nights,
          count: 0,
        })
      }
      aggregated.get(key)!.count++
    })

    // Group by night count
    aggregated.forEach(bar => {
      if (!groups[bar.nights]) {
        groups[bar.nights] = []
      }
      groups[bar.nights].push(bar)
    })

    // Sort bars within each group by check-in date
    Object.values(groups).forEach(bars => {
      bars.sort((a, b) => a.checkIn.getTime() - b.checkIn.getTime())
    })

    return groups
  }, [stays, monthStart, monthEnd])

  // Get sorted night counts (1, 2, 3, etc.)
  const nightCounts = Object.keys(staysByNight)
    .map(Number)
    .sort((a, b) => a - b)

  // Calculate daily occupancy for heat bar
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

  // Calculate bar position
  const getBarStyle = (bar: StayBar) => {
    const stayStart = Math.max(bar.checkIn.getTime(), monthStart.getTime())
    const stayEnd = Math.min(bar.checkOut.getTime(), monthEnd.getTime())

    const startOffset = differenceInDays(new Date(stayStart), monthStart)
    const duration = differenceInDays(new Date(stayEnd), new Date(stayStart)) + 1

    const leftPercent = (startOffset / totalDays) * 100
    const widthPercent = (duration / totalDays) * 100

    return {
      left: `${leftPercent}%`,
      width: `${Math.max(widthPercent, 2.5)}%`,
    }
  }

  // Stats
  const stats = useMemo(() => {
    const total = stays.length
    const totalNights = stays.reduce((sum, s) => sum + s.nights, 0)
    const avgStay = total > 0 ? (totalNights / total).toFixed(1) : '0'
    return { total, totalNights, avgStay }
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
        <div className="text-center">
          <h2 className="text-xl font-semibold text-gray-800">
            {THAI_MONTHS[selectedMonth.getMonth()]} {selectedMonth.getFullYear() + 543}
          </h2>
          <p className="text-sm text-gray-500">
            {stats.total} รายการ · {stats.totalNights} คืน · เฉลี่ย {stats.avgStay} คืน/ครั้ง
          </p>
        </div>
        <button onClick={nextMonth} className="p-2 hover:bg-gray-200 rounded-full">
          <ChevronRight className="w-5 h-5" />
        </button>
      </div>

      {/* Occupancy Heat Bar */}
      <div className="px-4 py-2 border-b bg-gray-50">
        <div className="flex h-8 rounded overflow-hidden">
          {daysInMonth.map(day => {
            const dateKey = format(day, 'yyyy-MM-dd')
            const count = dailyOccupancy[dateKey] || 0
            const intensity = count / maxOccupancy
            const isToday = isSameDay(day, new Date())

            return (
              <div
                key={dateKey}
                className={`flex-1 flex items-end justify-center relative group
                  ${isToday ? 'ring-2 ring-blue-500 ring-inset z-10' : ''}`}
                style={{
                  backgroundColor: count > 0
                    ? `rgba(59, 130, 246, ${0.15 + intensity * 0.6})`
                    : '#f9fafb'
                }}
              >
                <span className={`text-[9px] ${isToday ? 'font-bold text-blue-600' : 'text-gray-500'}`}>
                  {day.getDate()}
                </span>
                <div className="absolute bottom-full mb-1 hidden group-hover:block bg-gray-800 text-white text-xs px-2 py-1 rounded whitespace-nowrap z-20">
                  {format(day, 'd MMM')}: {count} ห้อง
                </div>
              </div>
            )
          })}
        </div>
      </div>

      {/* Timeline by Night Count */}
      <div className="p-4">
        {nightCounts.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            <Calendar className="w-10 h-10 mx-auto mb-2 text-gray-300" />
            <p>ไม่มีข้อมูลในเดือนนี้</p>
          </div>
        ) : (
          <div className="space-y-1">
            {nightCounts.map(nights => {
              const bars = staysByNight[nights]
              const totalCount = bars.reduce((sum, b) => sum + b.count, 0)

              return (
                <div key={nights} className="flex items-center">
                  {/* Y-axis label */}
                  <div className="w-16 flex-shrink-0 text-right pr-3">
                    <span className="text-sm font-medium text-gray-600">
                      {nights} คืน
                    </span>
                    <span className="text-xs text-gray-400 ml-1">({totalCount})</span>
                  </div>

                  {/* Timeline row */}
                  <div className="flex-1 relative h-6 bg-gray-50 rounded">
                    {bars.map(bar => {
                      const style = getBarStyle(bar)
                      const isHovered = hoveredBar === bar.id

                      return (
                        <div
                          key={bar.id}
                          className={`absolute top-0.5 h-5 rounded flex items-center justify-center text-white text-[10px] font-medium
                            ${getNightColor(nights)} ${isHovered ? 'ring-2 ring-gray-400 z-10' : ''}`}
                          style={style}
                          onMouseEnter={() => setHoveredBar(bar.id)}
                          onMouseLeave={() => setHoveredBar(null)}
                          title={`${format(bar.checkIn, 'd MMM')} - ${format(bar.checkOut, 'd MMM')} (${bar.count} ครั้ง)`}
                        >
                          {bar.count > 1 ? bar.count : ''}
                        </div>
                      )
                    })}
                  </div>
                </div>
              )
            })}
          </div>
        )}
      </div>

      {/* Legend */}
      <div className="flex items-center gap-3 px-4 py-3 bg-gray-50 border-t text-xs text-gray-500">
        <span>ระยะเวลา:</span>
        {[1, 2, 3, 4, 5, 6, 7].map(n => (
          <div key={n} className="flex items-center gap-1">
            <div className={`w-3 h-3 rounded ${getNightColor(n)}`} />
            <span>{n}{n === 7 ? '+' : ''}</span>
          </div>
        ))}
      </div>
    </div>
  )
}
