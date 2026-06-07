'use client'

import { useState } from 'react'
import {
  format,
  startOfMonth,
  endOfMonth,
  startOfWeek,
  endOfWeek,
  addDays,
  addMonths,
  subMonths,
  isSameMonth,
  isSameDay,
  isToday,
} from 'date-fns'
import { ChevronLeft, ChevronRight } from 'lucide-react'

export interface Booking {
  id: number
  customerId: number
  customerName: string
  roomNumber: string
  roomType: string
  checkInDate: string
  checkOutDate: string
  status: string
}

export interface CheckIn {
  id: number
  customerId: number
  customerName: string
  roomNumber: string
  roomType: string
  checkInTime: string
  checkOutTime?: string
}

interface CalendarProps {
  bookings: Booking[]
  checkins: CheckIn[]
  selectedMonth: Date
  onMonthChange: (date: Date) => void
  onDateClick?: (date: Date, bookings: Booking[], checkins: CheckIn[]) => void
}

const THAI_DAYS = ['อา', 'จ', 'อ', 'พ', 'พฤ', 'ศ', 'ส']
const THAI_MONTHS = [
  'มกราคม',
  'กุมภาพันธ์',
  'มีนาคม',
  'เมษายน',
  'พฤษภาคม',
  'มิถุนายน',
  'กรกฎาคม',
  'สิงหาคม',
  'กันยายน',
  'ตุลาคม',
  'พฤศจิกายน',
  'ธันวาคม',
]

export default function Calendar({
  bookings,
  checkins,
  selectedMonth,
  onMonthChange,
  onDateClick,
}: CalendarProps) {
  const [selectedDate, setSelectedDate] = useState<Date | null>(null)

  const monthStart = startOfMonth(selectedMonth)
  const monthEnd = endOfMonth(monthStart)
  const calendarStart = startOfWeek(monthStart, { weekStartsOn: 0 })
  const calendarEnd = endOfWeek(monthEnd, { weekStartsOn: 0 })

  const getBookingsForDate = (date: Date): Booking[] => {
    const dateStr = format(date, 'yyyy-MM-dd')
    return bookings.filter((booking) => {
      const checkIn = booking.checkInDate.split('T')[0]
      const checkOut = booking.checkOutDate.split('T')[0]
      return dateStr >= checkIn && dateStr <= checkOut
    })
  }

  const getCheckInsForDate = (date: Date): CheckIn[] => {
    const dateStr = format(date, 'yyyy-MM-dd')
    return checkins.filter((checkin) => {
      const checkInDate = checkin.checkInTime.split('T')[0]
      return checkInDate === dateStr
    })
  }

  const handleDateClick = (date: Date) => {
    setSelectedDate(date)
    const dayBookings = getBookingsForDate(date)
    const dayCheckins = getCheckInsForDate(date)
    if (onDateClick) {
      onDateClick(date, dayBookings, dayCheckins)
    }
  }

  const renderDays = () => {
    const days = []
    let day = calendarStart

    while (day <= calendarEnd) {
      const currentDay = day
      const dayBookings = getBookingsForDate(currentDay)
      const dayCheckins = getCheckInsForDate(currentDay)
      const isCurrentMonth = isSameMonth(currentDay, monthStart)
      const isSelected = selectedDate && isSameDay(currentDay, selectedDate)
      const isTodayDate = isToday(currentDay)

      days.push(
        <div
          key={currentDay.toISOString()}
          onClick={() => handleDateClick(currentDay)}
          className={`
            min-h-[100px] p-2 border border-gray-200 cursor-pointer transition-colors
            ${!isCurrentMonth ? 'bg-gray-50 text-gray-400' : 'bg-white hover:bg-gray-50'}
            ${isSelected ? 'ring-2 ring-blue-500 bg-blue-50' : ''}
            ${isTodayDate ? 'border-blue-500 border-2' : ''}
          `}
        >
          <div className={`text-sm font-medium mb-1 ${isTodayDate ? 'text-blue-600' : ''}`}>
            {format(currentDay, 'd')}
          </div>
          <div className="space-y-1">
            {dayBookings.length > 0 && (
              <div className="flex items-center gap-1">
                <span className="w-2 h-2 bg-green-500 rounded-full"></span>
                <span className="text-xs text-gray-600">
                  จอง {dayBookings.length}
                </span>
              </div>
            )}
            {dayCheckins.length > 0 && (
              <div className="flex items-center gap-1">
                <span className="w-2 h-2 bg-blue-500 rounded-full"></span>
                <span className="text-xs text-gray-600">
                  เข้าพัก {dayCheckins.length}
                </span>
              </div>
            )}
          </div>
        </div>
      )

      day = addDays(day, 1)
    }

    return days
  }

  return (
    <div className="bg-white rounded-lg shadow-lg overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between p-4 bg-gray-50 border-b">
        <button
          onClick={() => onMonthChange(subMonths(selectedMonth, 1))}
          className="p-2 hover:bg-gray-200 rounded-full transition-colors"
          aria-label="เดือนก่อนหน้า"
        >
          <ChevronLeft className="w-5 h-5" />
        </button>
        <h2 className="text-xl font-semibold text-gray-800">
          {THAI_MONTHS[selectedMonth.getMonth()]} {selectedMonth.getFullYear() + 543}
        </h2>
        <button
          onClick={() => onMonthChange(addMonths(selectedMonth, 1))}
          className="p-2 hover:bg-gray-200 rounded-full transition-colors"
          aria-label="เดือนถัดไป"
        >
          <ChevronRight className="w-5 h-5" />
        </button>
      </div>

      {/* Day Headers */}
      <div className="grid grid-cols-7 bg-gray-100">
        {THAI_DAYS.map((day, index) => (
          <div
            key={day}
            className={`p-3 text-center text-sm font-medium ${
              index === 0 ? 'text-red-600' : index === 6 ? 'text-blue-600' : 'text-gray-700'
            }`}
          >
            {day}
          </div>
        ))}
      </div>

      {/* Calendar Grid */}
      <div className="grid grid-cols-7">{renderDays()}</div>

      {/* Legend */}
      <div className="flex items-center gap-6 p-4 bg-gray-50 border-t">
        <div className="flex items-center gap-2">
          <span className="w-3 h-3 bg-green-500 rounded-full"></span>
          <span className="text-sm text-gray-600">การจอง</span>
        </div>
        <div className="flex items-center gap-2">
          <span className="w-3 h-3 bg-blue-500 rounded-full"></span>
          <span className="text-sm text-gray-600">เช็คอิน</span>
        </div>
        <div className="flex items-center gap-2">
          <span className="w-4 h-4 border-2 border-blue-500 rounded"></span>
          <span className="text-sm text-gray-600">วันนี้</span>
        </div>
      </div>
    </div>
  )
}
