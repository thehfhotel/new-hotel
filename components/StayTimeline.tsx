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
  checkedIn: number    // Rooms occupied (check-ins active this day)
  booked: number       // Rooms booked for this day
  checkinStays: Stay[] // Check-in stays active this day
  bookingStays: Stay[] // Booking stays for this day
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
    type: 'checkin' | 'booking'
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

  const today = (() => {
    const t = new Date()
    t.setHours(0, 0, 0, 0)
    return t
  })()

  // Calculate data for each day
  const dayData: DayData[] = useMemo(() => {
    return days.map(day => {
      const checkinStays: Stay[] = []
      const bookingStays: Stay[] = []

      // day comes from eachDayOfInterval — local time (e.g. Feb 19 00:00 GMT+7)
      // Use local time methods to extract the date part
      const dayMidnight = Date.UTC(day.getFullYear(), day.getMonth(), day.getDate())

      stays.forEach(stay => {
        // Checkin/checkout from API have Z suffix but are actually Thai local time
        // Use UTC methods to extract the stored date part correctly
        const checkInDate = new Date(stay.checkIn)
        const checkInMidnight = Date.UTC(checkInDate.getUTCFullYear(), checkInDate.getUTCMonth(), checkInDate.getUTCDate())
        const checkOutDate = new Date(stay.checkOut)
        const checkOutMidnight = Date.UTC(checkOutDate.getUTCFullYear(), checkOutDate.getUTCMonth(), checkOutDate.getUTCDate())

        // Stay overlaps this day if checkIn <= day AND checkOut > day
        const overlaps = checkInMidnight <= dayMidnight && checkOutMidnight > dayMidnight

        if (!overlaps) return

        if (stay.type === 'checkin') {
          checkinStays.push(stay)
        } else {
          bookingStays.push(stay)
        }
      })

      const bookingRoomCount = bookingStays.reduce((sum, s) => sum + (s.roomCount || 1), 0)

      return {
        date: day,
        checkedIn: checkinStays.length,
        booked: bookingRoomCount,
        checkinStays,
        bookingStays,
      }
    })
  }, [days, stays])

  // Find max for scaling — max across all checkedIn and booked values
  const maxCount = useMemo(() => {
    let max = 1
    for (const d of dayData) {
      if (d.checkedIn > max) max = d.checkedIn
      if (d.booked > max) max = d.booked
    }
    return max
  }, [dayData])

  // Stats
  const stats = useMemo(() => {
    const totalCheckins = stays.filter(s => s.type === 'checkin').length
    const totalBookings = stays.filter(s => s.type === 'booking').reduce((sum, s) => sum + (s.roomCount || 1), 0)
    const totalRooms = dayData.reduce((sum, d) => sum + d.checkedIn + d.booked, 0)
    const avgOccupancy = dayData.length > 0
      ? (totalRooms / dayData.length).toFixed(1)
      : '0'
    const peakDay = dayData.reduce((max, d) => {
      const dTotal = d.checkedIn + d.booked
      const maxTotal = max.checkedIn + max.booked
      return dTotal > maxTotal ? d : max
    }, dayData[0])
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
    <div className="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b border-gray-200 bg-gray-100">
        <button onClick={() => navigate('prev')} className="p-2 hover:bg-gray-100 rounded-full" aria-label="ก่อนหน้า">
          <ChevronLeft className="w-5 h-5" />
        </button>
        <div className="text-center">
          <h2 className="text-xl font-semibold text-gray-800">{getHeaderTitle()}</h2>
          <p className="text-sm text-gray-500">
            เช็คอิน {stats.totalCheckins} · จอง {stats.totalBookings} · เฉลี่ย {stats.avgOccupancy} ห้อง/วัน
          </p>
        </div>
        <button onClick={() => navigate('next')} className="p-2 hover:bg-gray-100 rounded-full" aria-label="ถัดไป">
          <ChevronRight className="w-5 h-5" />
        </button>
      </div>

      {/* View Mode Tabs */}
      <div className="flex items-center justify-center gap-1 p-2 border-b border-gray-200 bg-gray-100">
        {(['week', 'month'] as const).map(mode => (
          <button
            key={mode}
            onClick={() => onViewModeChange(mode)}
            className={`px-4 py-1.5 text-sm rounded-lg transition-colors ${
              viewMode === mode
                ? 'bg-red-600 text-white'
                : 'text-gray-500 hover:bg-gray-100'
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
            <Calendar className="w-10 h-10 mx-auto mb-2 text-gray-400" />
            <p>ไม่มีข้อมูลในช่วงนี้</p>
          </div>
        ) : (
          <div className="flex items-end gap-1" style={{ height: barHeight + 60 }}>
            {dayData.map((d, idx) => {
              const isDayToday = isSameDay(d.date, today)
              const dayDate = new Date(d.date)
              dayDate.setHours(0, 0, 0, 0)
              const isPast = isBefore(dayDate, today) && !isDayToday
              const isFuture = !isPast && !isDayToday
              const isWeekend = d.date.getDay() === 0 || d.date.getDay() === 6
              const isHovered = hoveredDay === format(d.date, 'yyyy-MM-dd')

              const checkedInHeight = (d.checkedIn / maxCount) * barHeight
              const bookedHeight = (d.booked / maxCount) * barHeight

              // Determine what to display
              const displayCount = isPast ? d.checkedIn : isFuture ? d.booked : (d.checkedIn + d.booked)

              return (
                <div
                  key={idx}
                  className={`flex-1 flex flex-col items-center ${isHovered ? 'z-10' : ''}`}
                  onMouseEnter={() => setHoveredDay(format(d.date, 'yyyy-MM-dd'))}
                  onMouseLeave={() => setHoveredDay(null)}
                >
                  {/* Tooltip */}
                  {isHovered && (
                    <div className="absolute -mt-16 bg-gray-800 text-white text-xs px-3 py-2 rounded shadow-lg whitespace-nowrap z-20">
                      <div className="font-medium mb-1">{format(d.date, 'd MMM yyyy')}</div>
                      {(isPast || isDayToday) && d.checkedIn > 0 && (
                        <div className="flex items-center gap-2">
                          <span className="w-2 h-2 bg-sky-500 rounded-sm"></span>
                          <span>เข้าพัก: {d.checkedIn} ห้อง</span>
                        </div>
                      )}
                      {(isFuture || isDayToday) && d.booked > 0 && (
                        <div className="flex items-center gap-2">
                          <span className="w-2 h-2 bg-amber-400 rounded-sm"></span>
                          <span>จอง: {d.booked} ห้อง</span>
                        </div>
                      )}
                      {displayCount === 0 && (
                        <div className="text-gray-400">ไม่มีข้อมูล</div>
                      )}
                    </div>
                  )}

                  {/* Bar area */}
                  <div
                    className={`w-full rounded-t overflow-hidden ${
                      isHovered ? 'ring-2 ring-gray-400' : ''
                    } ${isDayToday ? 'ring-2 ring-red-500' : ''}`}
                    style={{ height: barHeight }}
                  >
                    {isDayToday ? (
                      /* Today: two bars side by side */
                      <div className="w-full h-full flex items-end gap-px">
                        {/* Checked-in bar (left) */}
                        <div className="flex-1 flex flex-col justify-end h-full">
                          {d.checkedIn > 0 ? (
                            <div
                              className="w-full bg-sky-500 flex items-center justify-center cursor-pointer hover:bg-sky-600 rounded-tl"
                              style={{ height: checkedInHeight }}
                              onClick={() => setSelectedSegment({ date: d.date, type: 'checkin', stays: d.checkinStays })}
                            >
                              {viewMode === 'week' && checkedInHeight > 16 && (
                                <span className="text-white text-xs font-medium">{d.checkedIn}</span>
                              )}
                            </div>
                          ) : (
                            <div className="w-full h-1 bg-gray-100"></div>
                          )}
                        </div>
                        {/* Booked bar (right) */}
                        <div className="flex-1 flex flex-col justify-end h-full">
                          {d.booked > 0 ? (
                            <div
                              className="w-full bg-amber-400 flex items-center justify-center cursor-pointer hover:bg-amber-500 rounded-tr"
                              style={{ height: bookedHeight }}
                              onClick={() => setSelectedSegment({ date: d.date, type: 'booking', stays: d.bookingStays })}
                            >
                              {viewMode === 'week' && bookedHeight > 16 && (
                                <span className="text-white text-xs font-medium">{d.booked}</span>
                              )}
                            </div>
                          ) : (
                            <div className="w-full h-1 bg-gray-100"></div>
                          )}
                        </div>
                      </div>
                    ) : isPast ? (
                      /* Past: single sky bar for checked-in */
                      <div className="w-full h-full flex flex-col justify-end">
                        {d.checkedIn > 0 ? (
                          <div
                            className="w-full bg-sky-500 flex items-center justify-center cursor-pointer hover:bg-sky-600 rounded-t"
                            style={{ height: checkedInHeight }}
                            onClick={() => setSelectedSegment({ date: d.date, type: 'checkin', stays: d.checkinStays })}
                          >
                            {viewMode === 'week' && checkedInHeight > 16 && (
                              <span className="text-white text-xs font-medium">{d.checkedIn}</span>
                            )}
                          </div>
                        ) : (
                          <div className="w-full h-1 bg-gray-100"></div>
                        )}
                      </div>
                    ) : (
                      /* Future: single amber bar for booked */
                      <div className="w-full h-full flex flex-col justify-end">
                        {d.booked > 0 ? (
                          <div
                            className="w-full bg-amber-400 flex items-center justify-center cursor-pointer hover:bg-amber-500 rounded-t"
                            style={{ height: bookedHeight }}
                            onClick={() => setSelectedSegment({ date: d.date, type: 'booking', stays: d.bookingStays })}
                          >
                            {viewMode === 'week' && bookedHeight > 16 && (
                              <span className="text-white text-xs font-medium">{d.booked}</span>
                            )}
                          </div>
                        ) : (
                          <div className="w-full h-1 bg-gray-100"></div>
                        )}
                      </div>
                    )}
                  </div>

                  {/* Total label */}
                  <div className={`text-xs font-medium mt-1 ${displayCount > 0 ? 'text-gray-700' : 'text-gray-400'}`}>
                    {displayCount > 0 ? displayCount : ''}
                  </div>

                  {/* Day of week */}
                  <div className={`text-[10px] ${isWeekend ? 'text-red-600' : 'text-gray-500'}`}>
                    {THAI_DAYS_SHORT[d.date.getDay()]}
                  </div>

                  {/* Date */}
                  <div className={`text-xs ${isDayToday ? 'font-bold text-red-600' : 'text-gray-500'}`}>
                    {d.date.getDate()}
                  </div>
                </div>
              )
            })}
          </div>
        )}
      </div>

      {/* Legend */}
      <div className="flex items-center justify-center gap-6 px-4 py-3 bg-gray-100 border-t border-gray-200 text-sm">
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 bg-sky-500 rounded" />
          <span className="text-gray-500">เข้าพัก</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 bg-amber-400 rounded" />
          <span className="text-gray-500">การจอง</span>
        </div>
      </div>

      {/* Detail Panel */}
      {selectedSegment && (
        <div className="border-t border-gray-200 bg-white p-4">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-2">
              <div className={`w-3 h-3 rounded ${
                selectedSegment.type === 'checkin' ? 'bg-sky-500' : 'bg-amber-400'
              }`} />
              <h3 className="font-medium text-gray-800">
                {selectedSegment.type === 'checkin' && 'เข้าพัก'}
                {selectedSegment.type === 'booking' && 'การจอง'}
                <span className="text-gray-500 font-normal ml-2">
                  {format(selectedSegment.date, 'd MMM yyyy')} ({selectedSegment.stays.length} รายการ)
                </span>
              </h3>
            </div>
            <button
              onClick={() => setSelectedSegment(null)}
              className="p-1 hover:bg-gray-100 rounded"
            >
              <X className="w-5 h-5 text-gray-500" />
            </button>
          </div>

          {selectedSegment.stays.length === 0 ? (
            <p className="text-gray-500 text-sm">ไม่มีข้อมูล</p>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="bg-gray-100">
                    {selectedSegment.type === 'booking' ? (
                      <>
                        <th className="px-3 py-2 text-left font-medium text-gray-500">เลขที่จอง</th>
                        <th className="px-3 py-2 text-left font-medium text-gray-500">ลูกค้า</th>
                        <th className="px-3 py-2 text-left font-medium text-gray-500">วันจอง</th>
                        <th className="px-3 py-2 text-left font-medium text-gray-500">เช็คอิน</th>
                        <th className="px-3 py-2 text-left font-medium text-gray-500">เช็คเอาท์</th>
                        <th className="px-3 py-2 text-left font-medium text-gray-500">คืน</th>
                        <th className="px-3 py-2 text-left font-medium text-gray-500">ห้อง</th>
                        <th className="px-3 py-2 text-left font-medium text-gray-500">สถานะ</th>
                      </>
                    ) : (
                      <>
                        <th className="px-3 py-2 text-left font-medium text-gray-500">เลขที่</th>
                        <th className="px-3 py-2 text-left font-medium text-gray-500">ห้อง</th>
                        <th className="px-3 py-2 text-left font-medium text-gray-500">ลูกค้า</th>
                        <th className="px-3 py-2 text-left font-medium text-gray-500">เช็คอิน</th>
                        <th className="px-3 py-2 text-left font-medium text-gray-500">เช็คเอาท์</th>
                        <th className="px-3 py-2 text-left font-medium text-gray-500">คืน</th>
                        <th className="px-3 py-2 text-left font-medium text-gray-500">สถานะ</th>
                      </>
                    )}
                  </tr>
                </thead>
                <tbody>
                  {selectedSegment.stays.map((stay, idx) => (
                    <tr key={stay.id} className={`${idx % 2 === 0 ? 'bg-white' : 'bg-gray-100'} ${stay.source === 'new' ? 'border-l-2 border-l-red-400' : ''}`}>
                      {selectedSegment.type === 'booking' ? (
                        <>
                          <td className="px-3 py-2 text-gray-700 font-mono text-xs">
                            <span className="flex items-center gap-1">
                              {stay.source === 'new' && <span className="w-1.5 h-1.5 bg-red-500 rounded-full" title="New system" />}
                              {stay.bookNo || '-'}
                            </span>
                          </td>
                          <td className="px-3 py-2 text-gray-700">
                            {stay.customerName || '-'}
                          </td>
                          <td className="px-3 py-2 text-gray-700">
                            {stay.bookDate ? format(stay.bookDate, 'd MMM yyyy') : '-'}
                          </td>
                          <td className="px-3 py-2 text-gray-700">
                            {format(stay.checkIn, 'd MMM yyyy HH:mm')}
                          </td>
                          <td className="px-3 py-2 text-gray-700">
                            {format(stay.checkOut, 'd MMM yyyy HH:mm')}
                          </td>
                          <td className="px-3 py-2 text-gray-700">{stay.nights}</td>
                          <td className="px-3 py-2 text-gray-700">{stay.roomCount || 1}</td>
                          <td className="px-3 py-2">
                            <span className={`px-2 py-0.5 rounded-full text-xs ${
                              stay.status === 'จอง' ? 'bg-amber-500/10 text-amber-400' :
                              stay.status === 'เข้าพัก' ? 'bg-green-500/10 text-green-400' :
                              stay.status === 'เสร็จสิ้น' ? 'bg-gray-100 text-gray-500' :
                              stay.status === 'ยกเลิก' ? 'bg-red-500/10 text-red-600' :
                              'bg-gray-100 text-gray-500'
                            }`}>
                              {stay.status || '-'}
                            </span>
                          </td>
                        </>
                      ) : (
                        <>
                          <td className="px-3 py-2 text-gray-700 font-mono text-xs">
                            <span className="flex items-center gap-1">
                              {stay.source === 'new' && <span className="w-1.5 h-1.5 bg-red-500 rounded-full" title="New system" />}
                              {stay.checkinNo || '-'}
                            </span>
                          </td>
                          <td className="px-3 py-2 text-gray-700 font-medium">
                            {stay.roomNo || '-'}
                          </td>
                          <td className="px-3 py-2 text-gray-700">
                            {stay.customerName || '-'}
                          </td>
                          <td className="px-3 py-2 text-gray-700">
                            {format(stay.checkIn, 'd MMM yyyy HH:mm')}
                          </td>
                          <td className="px-3 py-2 text-gray-700">
                            {format(stay.checkOut, 'd MMM yyyy HH:mm')}
                          </td>
                          <td className="px-3 py-2 text-gray-700">{stay.nights}</td>
                          <td className="px-3 py-2">
                            <span className={`px-2 py-0.5 rounded-full text-xs ${
                              stay.status === 'เข้าพัก' ? 'bg-green-500/10 text-green-400' :
                              stay.status === 'เสร็จสิ้น' ? 'bg-gray-100 text-gray-500' :
                              'bg-red-500/10 text-red-600'
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
