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
import { ChevronLeft, ChevronRight, Calendar, X } from 'lucide-react'

export interface Stay {
  id: string
  checkIn: Date
  checkOut: Date
  type: 'booking' | 'checkin'
  nights: number
  // Common details
  customerName?: string
  status?: string
  // Booking-specific
  bookNo?: string
  bookDate?: Date
  roomCount?: number
  // Check-in specific
  checkinNo?: string
  roomNo?: string
  // Data source (for hybrid mode)
  source?: 'legacy' | 'new'
}

interface DayData {
  date: Date
  continuing: number  // Guests staying from previous day
  newCheckin: number  // New check-ins today
  booking: number     // Bookings (not checked in yet)
  total: number
  continuingStays: Stay[]  // List of continuing stays
  newCheckinStays: Stay[]  // List of new check-ins
  bookingStays: Stay[]     // List of bookings
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
  const [selectedSegment, setSelectedSegment] = useState<{
    date: Date
    type: 'continuing' | 'newCheckin' | 'booking'
    stays: Stay[]
  } | null>(null)

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
      const continuingStays: Stay[] = []
      const newCheckinStays: Stay[] = []
      const bookingStays: Stay[] = []

      const today = new Date()
      today.setHours(0, 0, 0, 0)

      stays.forEach(stay => {
        const isCheckInDay = isSameDay(stay.checkIn, day)
        // Guest is staying tonight = checkout DATE is AFTER today (compare dates only, ignore time)
        // This excludes guests checking out today regardless of checkout time
        const checkOutDate = new Date(stay.checkOut)
        checkOutDate.setHours(0, 0, 0, 0)
        const dayDate = new Date(day)
        dayDate.setHours(0, 0, 0, 0)
        const isStayingTonight = checkOutDate > dayDate
        // Continuing stay = checked in before today AND staying tonight
        const isContinuingStay = isBefore(stay.checkIn, day) && isStayingTonight

        if (stay.type === 'checkin') {
          // New check-in: arrives today AND staying tonight
          if (isCheckInDay && isStayingTonight) {
            newCheckinStays.push(stay)
          } else if (isContinuingStay) {
            // Continuing: checked in before today, staying tonight
            continuingStays.push(stay)
          }
          // Note: guests checking out today (checkOut date == day) are NOT counted
        } else {
          // booking - only show if the booking's check-in date is today or future
          // Past bookings (checkIn < today) should have been converted to check-ins already
          const bookingCheckInIsNotPast = !isBefore(stay.checkIn, today)
          // Show booking if it overlaps this day and checkIn is not in the past
          const overlapsDay = (isCheckInDay || isBefore(stay.checkIn, day)) && isStayingTonight
          if (bookingCheckInIsNotPast && overlapsDay) {
            bookingStays.push(stay)
          }
        }
      })

      // Sum roomCount for bookings (each booking can have multiple rooms)
      const bookingRoomCount = bookingStays.reduce((sum, s) => sum + (s.roomCount || 1), 0)

      return {
        date: day,
        continuing: continuingStays.length,
        newCheckin: newCheckinStays.length,
        booking: bookingRoomCount,
        total: continuingStays.length + newCheckinStays.length + bookingRoomCount,
        continuingStays,
        newCheckinStays,
        bookingStays,
      }
    })
  }, [days, stays])

  // Find max for scaling
  const maxTotal = Math.max(...dayData.map(d => d.total), 1)

  // Stats
  const stats = useMemo(() => {
    const totalCheckins = stays.filter(s => s.type === 'checkin').length
    // Sum roomCount for bookings (each booking can have multiple rooms)
    const totalBookings = stays.filter(s => s.type === 'booking').reduce((sum, s) => sum + (s.roomCount || 1), 0)
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
    <div className="bg-zinc-900 rounded-xl shadow-sm border border-zinc-800 overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b border-zinc-800 bg-zinc-800">
        <button onClick={() => navigate('prev')} className="p-2 hover:bg-zinc-800 rounded-full">
          <ChevronLeft className="w-5 h-5" />
        </button>
        <div className="text-center">
          <h2 className="text-xl font-semibold text-zinc-200">{getHeaderTitle()}</h2>
          <p className="text-sm text-zinc-500">
            เช็คอิน {stats.totalCheckins} · จอง {stats.totalBookings} · เฉลี่ย {stats.avgOccupancy} ห้อง/วัน
          </p>
        </div>
        <button onClick={() => navigate('next')} className="p-2 hover:bg-zinc-800 rounded-full">
          <ChevronRight className="w-5 h-5" />
        </button>
      </div>

      {/* View Mode Tabs */}
      <div className="flex items-center justify-center gap-1 p-2 border-b border-zinc-800 bg-zinc-800">
        {(['week', 'month'] as const).map(mode => (
          <button
            key={mode}
            onClick={() => onViewModeChange(mode)}
            className={`px-4 py-1.5 text-sm rounded-lg transition-colors ${
              viewMode === mode
                ? 'bg-red-600 text-white'
                : 'text-zinc-400 hover:bg-zinc-800'
            }`}
          >
            {mode === 'week' ? 'สัปดาห์' : 'เดือน'}
          </button>
        ))}
      </div>

      {/* Bar Chart */}
      <div className="p-4">
        {dayData.length === 0 ? (
          <div className="text-center py-8 text-zinc-500">
            <Calendar className="w-10 h-10 mx-auto mb-2 text-zinc-600" />
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
                    <div className="absolute -mt-20 bg-zinc-800 text-white text-xs px-3 py-2 rounded shadow-lg whitespace-nowrap z-20">
                      <div className="font-medium mb-1">{format(d.date, 'd MMM yyyy')}</div>
                      <div className="flex items-center gap-2">
                        <span className="w-2 h-2 bg-red-500 rounded-sm"></span>
                        <span>พักต่อ: {d.continuing}</span>
                      </div>
                      <div className="flex items-center gap-2">
                        <span className="w-2 h-2 bg-red-500 rounded-sm"></span>
                        <span>เช็คอินใหม่: {d.newCheckin}</span>
                      </div>
                      <div className="flex items-center gap-2">
                        <span className="w-2 h-2 bg-amber-400 rounded-sm"></span>
                        <span>จอง: {d.booking}</span>
                      </div>
                      <div className="border-t border-zinc-600 mt-1 pt-1 font-medium">
                        รวม: {d.total} ห้อง
                      </div>
                    </div>
                  )}

                  {/* Stacked Bar */}
                  <div
                    className={`w-full flex flex-col-reverse rounded-t overflow-hidden ${
                      isHovered ? 'ring-2 ring-zinc-600' : ''
                    } ${isToday ? 'ring-2 ring-red-500' : ''}`}
                    style={{ height: barHeight }}
                  >
                    {/* Continuing stays (red) */}
                    {d.continuing > 0 && (
                      <div
                        className="w-full bg-red-500 flex items-center justify-center cursor-pointer hover:bg-red-600"
                        style={{ height: continuingHeight }}
                        onClick={() => setSelectedSegment({ date: d.date, type: 'continuing', stays: d.continuingStays })}
                      >
                        {viewMode === 'week' && continuingHeight > 16 && (
                          <span className="text-white text-xs font-medium">{d.continuing}</span>
                        )}
                      </div>
                    )}
                    {/* New check-ins (red) */}
                    {d.newCheckin > 0 && (
                      <div
                        className="w-full bg-red-500 flex items-center justify-center cursor-pointer hover:bg-red-600"
                        style={{ height: newCheckinHeight }}
                        onClick={() => setSelectedSegment({ date: d.date, type: 'newCheckin', stays: d.newCheckinStays })}
                      >
                        {viewMode === 'week' && newCheckinHeight > 16 && (
                          <span className="text-white text-xs font-medium">{d.newCheckin}</span>
                        )}
                      </div>
                    )}
                    {/* Bookings (amber) */}
                    {d.booking > 0 && (
                      <div
                        className="w-full bg-amber-400 flex items-center justify-center cursor-pointer hover:bg-amber-500"
                        style={{ height: bookingHeight }}
                        onClick={() => setSelectedSegment({ date: d.date, type: 'booking', stays: d.bookingStays })}
                      >
                        {viewMode === 'week' && bookingHeight > 16 && (
                          <span className="text-white text-xs font-medium">{d.booking}</span>
                        )}
                      </div>
                    )}
                    {/* Empty state */}
                    {d.total === 0 && (
                      <div className="w-full h-1 bg-zinc-800"></div>
                    )}
                  </div>

                  {/* Total label */}
                  <div className={`text-xs font-medium mt-1 ${d.total > 0 ? 'text-zinc-300' : 'text-zinc-600'}`}>
                    {d.total > 0 ? d.total : ''}
                  </div>

                  {/* Day of week */}
                  <div className={`text-[10px] ${isWeekend ? 'text-red-400' : 'text-zinc-500'}`}>
                    {THAI_DAYS_SHORT[d.date.getDay()]}
                  </div>

                  {/* Date */}
                  <div className={`text-xs ${isToday ? 'font-bold text-red-400' : 'text-zinc-400'}`}>
                    {d.date.getDate()}
                  </div>
                </div>
              )
            })}
          </div>
        )}
      </div>

      {/* Legend */}
      <div className="flex items-center justify-center gap-6 px-4 py-3 bg-zinc-800 border-t border-zinc-800 text-sm">
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 bg-red-500 rounded" />
          <span className="text-zinc-400">พักต่อเนื่อง</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 bg-red-500 rounded" />
          <span className="text-zinc-400">เช็คอินใหม่</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 bg-amber-400 rounded" />
          <span className="text-zinc-400">การจอง</span>
        </div>
      </div>

      {/* Detail Panel */}
      {selectedSegment && (
        <div className="border-t border-zinc-800 bg-zinc-900 p-4">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-2">
              <div className={`w-3 h-3 rounded ${
                selectedSegment.type === 'continuing' ? 'bg-red-500' :
                selectedSegment.type === 'newCheckin' ? 'bg-red-500' : 'bg-amber-400'
              }`} />
              <h3 className="font-medium text-zinc-200">
                {selectedSegment.type === 'continuing' && 'พักต่อเนื่อง'}
                {selectedSegment.type === 'newCheckin' && 'เช็คอินใหม่'}
                {selectedSegment.type === 'booking' && 'การจอง'}
                <span className="text-zinc-500 font-normal ml-2">
                  {format(selectedSegment.date, 'd MMM yyyy')} ({selectedSegment.stays.length} รายการ)
                </span>
              </h3>
            </div>
            <button
              onClick={() => setSelectedSegment(null)}
              className="p-1 hover:bg-zinc-800 rounded"
            >
              <X className="w-5 h-5 text-zinc-500" />
            </button>
          </div>

          {selectedSegment.stays.length === 0 ? (
            <p className="text-zinc-500 text-sm">ไม่มีข้อมูล</p>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="bg-zinc-800">
                    {selectedSegment.type === 'booking' ? (
                      <>
                        <th className="px-3 py-2 text-left font-medium text-zinc-400">เลขที่จอง</th>
                        <th className="px-3 py-2 text-left font-medium text-zinc-400">ลูกค้า</th>
                        <th className="px-3 py-2 text-left font-medium text-zinc-400">วันจอง</th>
                        <th className="px-3 py-2 text-left font-medium text-zinc-400">เช็คอิน</th>
                        <th className="px-3 py-2 text-left font-medium text-zinc-400">เช็คเอาท์</th>
                        <th className="px-3 py-2 text-left font-medium text-zinc-400">คืน</th>
                        <th className="px-3 py-2 text-left font-medium text-zinc-400">ห้อง</th>
                        <th className="px-3 py-2 text-left font-medium text-zinc-400">สถานะ</th>
                      </>
                    ) : (
                      <>
                        <th className="px-3 py-2 text-left font-medium text-zinc-400">เลขที่</th>
                        <th className="px-3 py-2 text-left font-medium text-zinc-400">ห้อง</th>
                        <th className="px-3 py-2 text-left font-medium text-zinc-400">ลูกค้า</th>
                        <th className="px-3 py-2 text-left font-medium text-zinc-400">เช็คอิน</th>
                        <th className="px-3 py-2 text-left font-medium text-zinc-400">เช็คเอาท์</th>
                        <th className="px-3 py-2 text-left font-medium text-zinc-400">คืน</th>
                        <th className="px-3 py-2 text-left font-medium text-zinc-400">สถานะ</th>
                      </>
                    )}
                  </tr>
                </thead>
                <tbody>
                  {selectedSegment.stays.map((stay, idx) => (
                    <tr key={stay.id} className={`${idx % 2 === 0 ? 'bg-zinc-900' : 'bg-zinc-800'} ${stay.source === 'new' ? 'border-l-2 border-l-red-400' : ''}`}>
                      {selectedSegment.type === 'booking' ? (
                        <>
                          <td className="px-3 py-2 text-zinc-300 font-mono text-xs">
                            <span className="flex items-center gap-1">
                              {stay.source === 'new' && <span className="w-1.5 h-1.5 bg-red-500 rounded-full" title="New system" />}
                              {stay.bookNo || '-'}
                            </span>
                          </td>
                          <td className="px-3 py-2 text-zinc-300">
                            {stay.customerName || '-'}
                          </td>
                          <td className="px-3 py-2 text-zinc-300">
                            {stay.bookDate ? format(stay.bookDate, 'd MMM yyyy') : '-'}
                          </td>
                          <td className="px-3 py-2 text-zinc-300">
                            {format(stay.checkIn, 'd MMM yyyy HH:mm')}
                          </td>
                          <td className="px-3 py-2 text-zinc-300">
                            {format(stay.checkOut, 'd MMM yyyy HH:mm')}
                          </td>
                          <td className="px-3 py-2 text-zinc-300">{stay.nights}</td>
                          <td className="px-3 py-2 text-zinc-300">{stay.roomCount || 1}</td>
                          <td className="px-3 py-2">
                            <span className={`px-2 py-0.5 rounded-full text-xs ${
                              stay.status === 'จอง' ? 'bg-amber-500/10 text-amber-400' :
                              stay.status === 'เข้าพัก' ? 'bg-green-500/10 text-green-400' :
                              stay.status === 'เสร็จสิ้น' ? 'bg-zinc-800 text-zinc-400' :
                              stay.status === 'ยกเลิก' ? 'bg-red-500/10 text-red-400' :
                              'bg-zinc-800 text-zinc-400'
                            }`}>
                              {stay.status || '-'}
                            </span>
                          </td>
                        </>
                      ) : (
                        <>
                          <td className="px-3 py-2 text-zinc-300 font-mono text-xs">
                            <span className="flex items-center gap-1">
                              {stay.source === 'new' && <span className="w-1.5 h-1.5 bg-red-500 rounded-full" title="New system" />}
                              {stay.checkinNo || '-'}
                            </span>
                          </td>
                          <td className="px-3 py-2 text-zinc-300 font-medium">
                            {stay.roomNo || '-'}
                          </td>
                          <td className="px-3 py-2 text-zinc-300">
                            {stay.customerName || '-'}
                          </td>
                          <td className="px-3 py-2 text-zinc-300">
                            {format(stay.checkIn, 'd MMM yyyy HH:mm')}
                          </td>
                          <td className="px-3 py-2 text-zinc-300">
                            {format(stay.checkOut, 'd MMM yyyy HH:mm')}
                          </td>
                          <td className="px-3 py-2 text-zinc-300">{stay.nights}</td>
                          <td className="px-3 py-2">
                            <span className={`px-2 py-0.5 rounded-full text-xs ${
                              stay.status === 'เข้าพัก' ? 'bg-green-500/10 text-green-400' :
                              stay.status === 'เสร็จสิ้น' ? 'bg-zinc-800 text-zinc-400' :
                              'bg-red-500/10 text-red-400'
                            }`}>
                              {stay.status || '-'}
                            </span>
                          </td>
                        </>
                      )}
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      )}
    </div>
  )
}
