'use client'

import { useMemo, useState } from 'react'
import {
  format,
  startOfMonth,
  endOfMonth,
  startOfWeek,
  endOfWeek,
  eachDayOfInterval,
  isSameDay,
  isBefore,
  addWeeks,
  addMonths,
} from 'date-fns'
import { ChevronLeft, ChevronRight, Calendar } from 'lucide-react'

export interface Stay {
  id: string
  checkIn: Date
  checkOut: Date
  type: 'booking' | 'checkin'
  nights: number
}

interface DayData {
  date: Date
  continuing: number  // Guests staying from previous day
  newCheckin: number  // New check-ins today
  booking: number     // Bookings (not checked in yet)
  total: number
}

type ViewMode = 'week' | 'month'

interface StayTimelineProps {
  stays: Stay[]
  selectedDate: Date
  onDateChange: (date: Date) => void
  viewMode: ViewMode
  onViewModeChange: (mode: ViewMode) => void
}

const THAI_MONTHS = [
  'มกราคม', 'กุมภาพันธ์', 'มีนาคม', 'เมษายน', 'พฤษภาคม', 'มิถุนายน',
  'กรกฎาคม', 'สิงหาคม', 'กันยายน', 'ตุลาคม', 'พฤศจิกายน', 'ธันวาคม',
]

const THAI_DAYS_SHORT = ['อา', 'จ', 'อ', 'พ', 'พฤ', 'ศ', 'ส']

export default function StayTimeline({
  stays,
  selectedDate,
  onDateChange,
  viewMode,
  onViewModeChange,
}: StayTimelineProps) {
  const [hoveredDay, setHoveredDay] = useState<string | null>(null)

  // Calculate date range based on view mode
  const { rangeStart, rangeEnd, days } = useMemo(() => {
    let start: Date, end: Date

    if (viewMode === 'week') {
      start = startOfWeek(selectedDate, { weekStartsOn: 0 })
      end = endOfWeek(selectedDate, { weekStartsOn: 0 })
    } else {
      start = startOfMonth(selectedDate)
      end = endOfMonth(selectedDate)
    }

    return {
      rangeStart: start,
      rangeEnd: end,
      days: eachDayOfInterval({ start, end }),
    }
  }, [selectedDate, viewMode])

  // Calculate data for each day
  const dayData: DayData[] = useMemo(() => {
    return days.map(day => {
      let continuing = 0
      let newCheckin = 0
      let booking = 0

      stays.forEach(stay => {
        const isCheckInDay = isSameDay(stay.checkIn, day)
        const isStayingToday = isBefore(stay.checkIn, day) &&
                               (isSameDay(stay.checkOut, day) || isBefore(day, stay.checkOut))
        const isCheckOutDay = isSameDay(stay.checkOut, day)

        if (stay.type === 'checkin') {
          if (isCheckInDay) {
            newCheckin++
          } else if (isStayingToday || isCheckOutDay) {
            continuing++
          }
        } else {
          // booking
          if (isCheckInDay || isStayingToday || isCheckOutDay) {
            booking++
          }
        }
      })

      return {
        date: day,
        continuing,
        newCheckin,
        booking,
        total: continuing + newCheckin + booking,
      }
    })
  }, [days, stays])

  // Find max for scaling
  const maxTotal = Math.max(...dayData.map(d => d.total), 1)

  // Stats
  const stats = useMemo(() => {
    const totalCheckins = stays.filter(s => s.type === 'checkin').length
    const totalBookings = stays.filter(s => s.type === 'booking').length
    const avgOccupancy = dayData.length > 0
      ? (dayData.reduce((sum, d) => sum + d.total, 0) / dayData.length).toFixed(1)
      : '0'
    const peakDay = dayData.reduce((max, d) => d.total > max.total ? d : max, dayData[0])
    return { totalCheckins, totalBookings, avgOccupancy, peakDay }
  }, [stays, dayData])

  // Navigation
  const navigate = (direction: 'prev' | 'next') => {
    const mult = direction === 'prev' ? -1 : 1
    if (viewMode === 'week') {
      onDateChange(addWeeks(selectedDate, mult))
    } else {
      onDateChange(addMonths(selectedDate, mult))
    }
  }

  // Format header title
  const getHeaderTitle = () => {
    if (viewMode === 'week') {
      return `${format(rangeStart, 'd')} - ${format(rangeEnd, 'd')} ${THAI_MONTHS[rangeEnd.getMonth()]} ${rangeEnd.getFullYear() + 543}`
    }
    return `${THAI_MONTHS[selectedDate.getMonth()]} ${selectedDate.getFullYear() + 543}`
  }

  const barHeight = viewMode === 'week' ? 160 : 100

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
            เช็คอิน {stats.totalCheckins} · จอง {stats.totalBookings} · เฉลี่ย {stats.avgOccupancy} ห้อง/วัน
          </p>
        </div>
        <button onClick={() => navigate('next')} className="p-2 hover:bg-gray-200 rounded-full">
          <ChevronRight className="w-5 h-5" />
        </button>
      </div>

      {/* View Mode Tabs */}
      <div className="flex items-center justify-center gap-1 p-2 border-b bg-gray-50">
        {(['week', 'month'] as const).map(mode => (
          <button
            key={mode}
            onClick={() => onViewModeChange(mode)}
            className={`px-4 py-1.5 text-sm rounded-lg transition-colors ${
              viewMode === mode
                ? 'bg-blue-600 text-white'
                : 'text-gray-600 hover:bg-gray-200'
            }`}
          >
            {mode === 'week' ? 'สัปดาห์' : 'เดือน'}
          </button>
        ))}
      </div>

      {/* Bar Chart */}
      <div className="p-4">
        {dayData.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            <Calendar className="w-10 h-10 mx-auto mb-2 text-gray-300" />
            <p>ไม่มีข้อมูลในช่วงนี้</p>
          </div>
        ) : (
          <div className="flex items-end gap-1" style={{ height: barHeight + 60 }}>
            {dayData.map((d, idx) => {
              const isToday = isSameDay(d.date, new Date())
              const isWeekend = d.date.getDay() === 0 || d.date.getDay() === 6
              const isHovered = hoveredDay === format(d.date, 'yyyy-MM-dd')

              const continuingHeight = (d.continuing / maxTotal) * barHeight
              const newCheckinHeight = (d.newCheckin / maxTotal) * barHeight
              const bookingHeight = (d.booking / maxTotal) * barHeight

              return (
                <div
                  key={idx}
                  className={`flex-1 flex flex-col items-center ${isHovered ? 'z-10' : ''}`}
                  onMouseEnter={() => setHoveredDay(format(d.date, 'yyyy-MM-dd'))}
                  onMouseLeave={() => setHoveredDay(null)}
                >
                  {/* Tooltip */}
                  {isHovered && (
                    <div className="absolute -mt-20 bg-gray-800 text-white text-xs px-3 py-2 rounded shadow-lg whitespace-nowrap z-20">
                      <div className="font-medium mb-1">{format(d.date, 'd MMM yyyy')}</div>
                      <div className="flex items-center gap-2">
                        <span className="w-2 h-2 bg-blue-500 rounded-sm"></span>
                        <span>พักต่อ: {d.continuing}</span>
                      </div>
                      <div className="flex items-center gap-2">
                        <span className="w-2 h-2 bg-emerald-500 rounded-sm"></span>
                        <span>เช็คอินใหม่: {d.newCheckin}</span>
                      </div>
                      <div className="flex items-center gap-2">
                        <span className="w-2 h-2 bg-amber-400 rounded-sm"></span>
                        <span>จอง: {d.booking}</span>
                      </div>
                      <div className="border-t border-gray-600 mt-1 pt-1 font-medium">
                        รวม: {d.total} ห้อง
                      </div>
                    </div>
                  )}

                  {/* Stacked Bar */}
                  <div
                    className={`w-full flex flex-col-reverse rounded-t overflow-hidden ${
                      isHovered ? 'ring-2 ring-gray-400' : ''
                    } ${isToday ? 'ring-2 ring-blue-500' : ''}`}
                    style={{ height: barHeight }}
                  >
                    {/* Continuing stays (blue) */}
                    {d.continuing > 0 && (
                      <div
                        className="w-full bg-blue-500 flex items-center justify-center"
                        style={{ height: continuingHeight }}
                      >
                        {viewMode === 'week' && continuingHeight > 16 && (
                          <span className="text-white text-xs font-medium">{d.continuing}</span>
                        )}
                      </div>
                    )}
                    {/* New check-ins (green) */}
                    {d.newCheckin > 0 && (
                      <div
                        className="w-full bg-emerald-500 flex items-center justify-center"
                        style={{ height: newCheckinHeight }}
                      >
                        {viewMode === 'week' && newCheckinHeight > 16 && (
                          <span className="text-white text-xs font-medium">{d.newCheckin}</span>
                        )}
                      </div>
                    )}
                    {/* Bookings (amber) */}
                    {d.booking > 0 && (
                      <div
                        className="w-full bg-amber-400 flex items-center justify-center"
                        style={{ height: bookingHeight }}
                      >
                        {viewMode === 'week' && bookingHeight > 16 && (
                          <span className="text-white text-xs font-medium">{d.booking}</span>
                        )}
                      </div>
                    )}
                    {/* Empty state */}
                    {d.total === 0 && (
                      <div className="w-full h-1 bg-gray-200"></div>
                    )}
                  </div>

                  {/* Total label */}
                  <div className={`text-xs font-medium mt-1 ${d.total > 0 ? 'text-gray-700' : 'text-gray-300'}`}>
                    {d.total > 0 ? d.total : ''}
                  </div>

                  {/* Day of week */}
                  <div className={`text-[10px] ${isWeekend ? 'text-red-400' : 'text-gray-400'}`}>
                    {THAI_DAYS_SHORT[d.date.getDay()]}
                  </div>

                  {/* Date */}
                  <div className={`text-xs ${isToday ? 'font-bold text-blue-600' : 'text-gray-600'}`}>
                    {d.date.getDate()}
                  </div>
                </div>
              )
            })}
          </div>
        )}
      </div>

      {/* Legend */}
      <div className="flex items-center justify-center gap-6 px-4 py-3 bg-gray-50 border-t text-sm">
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 bg-blue-500 rounded" />
          <span className="text-gray-600">พักต่อเนื่อง</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 bg-emerald-500 rounded" />
          <span className="text-gray-600">เช็คอินใหม่</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 bg-amber-400 rounded" />
          <span className="text-gray-600">การจอง</span>
        </div>
      </div>
    </div>
  )
}
