'use client'

import { useMemo, useState } from 'react'
import {
  format,
  startOfMonth,
  endOfMonth,
  startOfWeek,
  endOfWeek,
  eachDayOfInterval,
  differenceInDays,
  isSameDay,
  isWithinInterval,
  addDays,
  addWeeks,
  addMonths,
  subDays,
  subWeeks,
  subMonths,
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

type ViewMode = 'day' | 'week' | 'month'

interface StayTimelineProps {
  stays: Stay[]
  selectedDate: Date
  onDateChange: (date: Date) => void
}

const THAI_MONTHS = [
  'มกราคม', 'กุมภาพันธ์', 'มีนาคม', 'เมษายน', 'พฤษภาคม', 'มิถุนายน',
  'กรกฎาคม', 'สิงหาคม', 'กันยายน', 'ตุลาคม', 'พฤศจิกายน', 'ธันวาคม',
]

const THAI_DAYS_SHORT = ['อา', 'จ', 'อ', 'พ', 'พฤ', 'ศ', 'ส']

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
  selectedDate,
  onDateChange,
}: StayTimelineProps) {
  const [viewMode, setViewMode] = useState<ViewMode>('month')
  const [hoveredBar, setHoveredBar] = useState<string | null>(null)

  // Calculate date range based on view mode
  const { rangeStart, rangeEnd, days } = useMemo(() => {
    let start: Date, end: Date

    switch (viewMode) {
      case 'day':
        start = selectedDate
        end = selectedDate
        break
      case 'week':
        start = startOfWeek(selectedDate, { weekStartsOn: 0 })
        end = endOfWeek(selectedDate, { weekStartsOn: 0 })
        break
      case 'month':
      default:
        start = startOfMonth(selectedDate)
        end = endOfMonth(selectedDate)
        break
    }

    return {
      rangeStart: start,
      rangeEnd: end,
      days: eachDayOfInterval({ start, end }),
    }
  }, [selectedDate, viewMode])

  const totalDays = days.length

  // Filter stays that overlap with current range
  const visibleStays = useMemo(() => {
    return stays.filter(stay =>
      stay.checkOut >= rangeStart && stay.checkIn <= rangeEnd
    )
  }, [stays, rangeStart, rangeEnd])

  // Group stays by night count, then aggregate by check-in date
  const staysByNight = useMemo(() => {
    const groups: Record<number, StayBar[]> = {}
    const aggregated = new Map<string, StayBar>()

    visibleStays.forEach(stay => {
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

    aggregated.forEach(bar => {
      if (!groups[bar.nights]) {
        groups[bar.nights] = []
      }
      groups[bar.nights].push(bar)
    })

    Object.values(groups).forEach(bars => {
      bars.sort((a, b) => a.checkIn.getTime() - b.checkIn.getTime())
    })

    return groups
  }, [visibleStays])

  const nightCounts = Object.keys(staysByNight).map(Number).sort((a, b) => a - b)

  // Calculate daily occupancy
  const dailyOccupancy = useMemo(() => {
    const occupancy: Record<string, number> = {}
    days.forEach(day => {
      const dateKey = format(day, 'yyyy-MM-dd')
      occupancy[dateKey] = stays.filter(stay => {
        return isWithinInterval(day, { start: stay.checkIn, end: stay.checkOut }) ||
               isSameDay(day, stay.checkIn) || isSameDay(day, stay.checkOut)
      }).length
    })
    return occupancy
  }, [stays, days])

  const maxOccupancy = Math.max(...Object.values(dailyOccupancy), 1)

  // Calculate bar position
  const getBarStyle = (bar: StayBar) => {
    const stayStart = Math.max(bar.checkIn.getTime(), rangeStart.getTime())
    const stayEnd = Math.min(bar.checkOut.getTime(), rangeEnd.getTime())

    const startOffset = differenceInDays(new Date(stayStart), rangeStart)
    const duration = differenceInDays(new Date(stayEnd), new Date(stayStart)) + 1

    const leftPercent = (startOffset / totalDays) * 100
    const widthPercent = (duration / totalDays) * 100

    return {
      left: `${leftPercent}%`,
      width: `${Math.max(widthPercent, viewMode === 'month' ? 2.5 : 8)}%`,
    }
  }

  // Stats
  const stats = useMemo(() => {
    const total = visibleStays.length
    const totalNights = visibleStays.reduce((sum, s) => sum + s.nights, 0)
    const avgStay = total > 0 ? (totalNights / total).toFixed(1) : '0'
    return { total, totalNights, avgStay }
  }, [visibleStays])

  // Navigation
  const navigate = (direction: 'prev' | 'next') => {
    const mult = direction === 'prev' ? -1 : 1
    switch (viewMode) {
      case 'day':
        onDateChange(addDays(selectedDate, mult))
        break
      case 'week':
        onDateChange(addWeeks(selectedDate, mult))
        break
      case 'month':
        onDateChange(addMonths(selectedDate, mult))
        break
    }
  }

  // Format header title based on view mode
  const getHeaderTitle = () => {
    switch (viewMode) {
      case 'day':
        return `${selectedDate.getDate()} ${THAI_MONTHS[selectedDate.getMonth()]} ${selectedDate.getFullYear() + 543}`
      case 'week':
        return `${format(rangeStart, 'd')} - ${format(rangeEnd, 'd')} ${THAI_MONTHS[rangeEnd.getMonth()]} ${rangeEnd.getFullYear() + 543}`
      case 'month':
      default:
        return `${THAI_MONTHS[selectedDate.getMonth()]} ${selectedDate.getFullYear() + 543}`
    }
  }

  return (
    <div className="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b bg-gray-50">
        <button onClick={() => navigate('prev')} className="p-2 hover:bg-gray-200 rounded-full">
          <ChevronLeft className="w-5 h-5" />
        </button>
        <div className="text-center">
          <h2 className="text-xl font-semibold text-gray-800">{getHeaderTitle()}</h2>
          <p className="text-sm text-gray-500">
            {stats.total} รายการ · {stats.totalNights} คืน · เฉลี่ย {stats.avgStay} คืน/ครั้ง
          </p>
        </div>
        <button onClick={() => navigate('next')} className="p-2 hover:bg-gray-200 rounded-full">
          <ChevronRight className="w-5 h-5" />
        </button>
      </div>

      {/* View Mode Tabs */}
      <div className="flex items-center justify-center gap-1 p-2 border-b bg-gray-50">
        {(['day', 'week', 'month'] as ViewMode[]).map(mode => (
          <button
            key={mode}
            onClick={() => setViewMode(mode)}
            className={`px-4 py-1.5 text-sm rounded-lg transition-colors ${
              viewMode === mode
                ? 'bg-blue-600 text-white'
                : 'text-gray-600 hover:bg-gray-200'
            }`}
          >
            {mode === 'day' ? 'วัน' : mode === 'week' ? 'สัปดาห์' : 'เดือน'}
          </button>
        ))}
      </div>

      {/* Day of Week Headers */}
      <div className="px-4 pt-3">
        <div className="flex">
          <div className="w-16 flex-shrink-0" />
          <div className="flex-1 flex">
            {days.map(day => {
              const isToday = isSameDay(day, new Date())
              const isWeekend = day.getDay() === 0 || day.getDay() === 6
              return (
                <div
                  key={day.toISOString()}
                  className={`flex-1 text-center text-[10px] font-medium ${
                    isToday ? 'text-blue-600' : isWeekend ? 'text-red-400' : 'text-gray-400'
                  }`}
                >
                  {THAI_DAYS_SHORT[day.getDay()]}
                </div>
              )
            })}
          </div>
        </div>
      </div>

      {/* Occupancy Heat Bar with Date Numbers */}
      <div className="px-4 py-1 border-b">
        <div className="flex">
          <div className="w-16 flex-shrink-0" />
          <div className="flex-1 flex h-8 rounded overflow-hidden">
            {days.map(day => {
              const dateKey = format(day, 'yyyy-MM-dd')
              const count = dailyOccupancy[dateKey] || 0
              const intensity = count / maxOccupancy
              const isToday = isSameDay(day, new Date())
              const isWeekend = day.getDay() === 0 || day.getDay() === 6

              return (
                <div
                  key={dateKey}
                  className={`flex-1 flex items-end justify-center relative group
                    ${isToday ? 'ring-2 ring-blue-500 ring-inset z-10' : ''}`}
                  style={{
                    backgroundColor: count > 0
                      ? `rgba(59, 130, 246, ${0.15 + intensity * 0.6})`
                      : isWeekend ? '#fef2f2' : '#f9fafb'
                  }}
                >
                  <span className={`text-[10px] ${isToday ? 'font-bold text-blue-600' : 'text-gray-600'}`}>
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
      </div>

      {/* Timeline by Night Count */}
      <div className="p-4">
        {nightCounts.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            <Calendar className="w-10 h-10 mx-auto mb-2 text-gray-300" />
            <p>ไม่มีข้อมูลในช่วงนี้</p>
          </div>
        ) : (
          <div className="space-y-1">
            {nightCounts.map(nights => {
              const bars = staysByNight[nights]
              const totalCount = bars.reduce((sum, b) => sum + b.count, 0)

              return (
                <div key={nights} className="flex items-center">
                  <div className="w-16 flex-shrink-0 text-right pr-3">
                    <span className="text-sm font-medium text-gray-600">{nights} คืน</span>
                    <span className="text-xs text-gray-400 ml-1">({totalCount})</span>
                  </div>
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
