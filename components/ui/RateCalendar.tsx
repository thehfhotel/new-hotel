'use client'

import { useState, useMemo } from 'react'
import {
  startOfMonth,
  endOfMonth,
  startOfWeek,
  endOfWeek,
  addDays,
  addMonths,
  subMonths,
  isSameMonth,
  isToday,
  format,
  getDay,
} from 'date-fns'
import { ChevronLeft, ChevronRight, X, Percent, Tag, Calendar } from 'lucide-react'

export interface Rate {
  id: number
  rateName: string
  roomTypeId: number
  rateType: 'multiplier' | 'fixed'
  rateValue: number
  validFrom: string
  validTo: string
  daysOfWeek: number[]
  active: boolean
}

interface RoomType {
  id: number
  typeCode: string
  typeName: string
  basePrice: number
}

interface RateCalendarProps {
  roomTypeId: number | null
  month: number // 0-11
  year: number
  rates: Rate[]
  roomTypes: RoomType[]
  onMonthChange?: (month: number, year: number) => void
}

interface DayRateInfo {
  date: Date
  basePrice: number
  effectivePrice: number
  appliedRate: Rate | null
  isSpecialRate: boolean
  discount: number
  discountPercent: number
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

export default function RateCalendar({
  roomTypeId,
  month,
  year,
  rates,
  roomTypes,
  onMonthChange,
}: RateCalendarProps) {
  const [selectedDay, setSelectedDay] = useState<DayRateInfo | null>(null)

  const selectedDate = useMemo(() => new Date(year, month, 1), [year, month])
  const roomType = roomTypes.find((rt) => rt.id === roomTypeId)
  const basePrice = roomType?.basePrice ?? 0

  // Filter rates for selected room type
  const applicableRates = useMemo(() => {
    if (!roomTypeId) return []
    return rates.filter((rate) => rate.roomTypeId === roomTypeId && rate.active)
  }, [rates, roomTypeId])

  // Generate calendar days with rate calculations
  const calendarDays = useMemo(() => {
    // Calculate effective rate for a specific date (defined inside useMemo)
    const getEffectiveRateForDate = (date: Date): DayRateInfo => {
      const dateStr = format(date, 'yyyy-MM-dd')
      const dayOfWeek = getDay(date)

      let appliedRate: Rate | null = null
      let effectivePrice = basePrice

      // Find the first applicable rate (in order of priority - could be enhanced)
      for (const rate of applicableRates) {
        const validFrom = rate.validFrom.split('T')[0]
        const validTo = rate.validTo.split('T')[0]

        // Check if date is within valid range
        if (dateStr >= validFrom && dateStr <= validTo) {
          // Check if day of week is included
          if (rate.daysOfWeek.includes(dayOfWeek)) {
            appliedRate = rate

            if (rate.rateType === 'multiplier') {
              effectivePrice = basePrice * rate.rateValue
            } else {
              effectivePrice = rate.rateValue
            }
            break // Use first matching rate
          }
        }
      }

      const discount = basePrice - effectivePrice
      const discountPercent = basePrice > 0 ? (discount / basePrice) * 100 : 0

      return {
        date,
        basePrice,
        effectivePrice,
        appliedRate,
        isSpecialRate: appliedRate !== null,
        discount,
        discountPercent,
      }
    }

    const monthStart = startOfMonth(selectedDate)
    const monthEnd = endOfMonth(selectedDate)
    const calendarStart = startOfWeek(monthStart, { weekStartsOn: 0 })
    const calendarEnd = endOfWeek(monthEnd, { weekStartsOn: 0 })

    const days: DayRateInfo[] = []
    let day = calendarStart

    while (day <= calendarEnd) {
      days.push(getEffectiveRateForDate(day))
      day = addDays(day, 1)
    }

    return days
  }, [selectedDate, applicableRates, basePrice])

  const handlePrevMonth = () => {
    const newDate = subMonths(selectedDate, 1)
    onMonthChange?.(newDate.getMonth(), newDate.getFullYear())
  }

  const handleNextMonth = () => {
    const newDate = addMonths(selectedDate, 1)
    onMonthChange?.(newDate.getMonth(), newDate.getFullYear())
  }

  const handleDayClick = (dayInfo: DayRateInfo) => {
    setSelectedDay(dayInfo)
  }

  const closeDetail = () => {
    setSelectedDay(null)
  }

  // Get color class based on rate
  const getRateColorClass = (dayInfo: DayRateInfo, isCurrentMonth: boolean) => {
    if (!isCurrentMonth) {
      return 'bg-gray-100 text-gray-400'
    }

    if (!dayInfo.isSpecialRate) {
      return 'bg-white hover:bg-gray-100'
    }

    if (dayInfo.discount > 0) {
      // Discount rate - green shades
      if (dayInfo.discountPercent >= 20) {
        return 'bg-green-500/20 hover:bg-green-500/30 border-green-500'
      }
      return 'bg-green-500/10 hover:bg-green-500/20 border-green-500'
    } else if (dayInfo.discount < 0) {
      // Premium rate - red shades
      if (Math.abs(dayInfo.discountPercent) >= 20) {
        return 'bg-red-500/20 hover:bg-red-500/30 border-red-500'
      }
      return 'bg-red-500/10 hover:bg-red-500/20 border-red-500'
    }

    return 'bg-red-500/10 hover:bg-red-500/20 border-red-500'
  }

  const formatPrice = (price: number) => {
    return price.toLocaleString('th-TH', {
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    })
  }

  if (!roomTypeId) {
    return (
      <div className="bg-white rounded-lg shadow-lg border border-gray-200 p-8 text-center">
        <Calendar className="w-12 h-12 text-gray-400 mx-auto mb-3" />
        <p className="text-gray-500">กรุณาเลือกประเภทห้องเพื่อดูปฏิทินอัตรา</p>
      </div>
    )
  }

  return (
    <div className="bg-white rounded-lg shadow-lg border border-gray-200 overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between p-4 bg-gray-100 border-b border-gray-200">
        <button
          onClick={handlePrevMonth}
          className="p-2 hover:bg-gray-100 rounded-full transition-colors"
          aria-label="เดือนก่อนหน้า"
        >
          <ChevronLeft className="w-5 h-5" />
        </button>
        <div className="text-center">
          <h2 className="text-xl font-semibold text-gray-800">
            {THAI_MONTHS[month]} {year + 543}
          </h2>
          {roomType && (
            <p className="text-sm text-gray-500">
              {roomType.typeName} - ราคาปกติ {formatPrice(basePrice)} บาท
            </p>
          )}
        </div>
        <button
          onClick={handleNextMonth}
          className="p-2 hover:bg-gray-100 rounded-full transition-colors"
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
              index === 0 ? 'text-red-600' : index === 6 ? 'text-red-600' : 'text-gray-700'
            }`}
          >
            {day}
          </div>
        ))}
      </div>

      {/* Calendar Grid */}
      <div className="grid grid-cols-7">
        {calendarDays.map((dayInfo, index) => {
          const isCurrentMonth = isSameMonth(dayInfo.date, selectedDate)
          const isTodayDate = isToday(dayInfo.date)
          const colorClass = getRateColorClass(dayInfo, isCurrentMonth)

          return (
            <div
              key={index}
              onClick={() => isCurrentMonth && handleDayClick(dayInfo)}
              className={`
                min-h-[80px] p-2 border border-gray-200 transition-colors
                ${colorClass}
                ${isCurrentMonth ? 'cursor-pointer' : 'cursor-default'}
                ${isTodayDate ? 'ring-2 ring-red-500 ring-inset' : ''}
              `}
            >
              <div
                className={`text-sm font-medium mb-1 ${
                  isTodayDate ? 'text-red-600' : isCurrentMonth ? '' : 'text-gray-400'
                }`}
              >
                {format(dayInfo.date, 'd')}
              </div>
              {isCurrentMonth && (
                <div className="space-y-0.5">
                  <div
                    className={`text-xs font-semibold ${
                      dayInfo.isSpecialRate
                        ? dayInfo.discount > 0
                          ? 'text-green-400'
                          : dayInfo.discount < 0
                          ? 'text-red-600'
                          : 'text-red-600'
                        : 'text-gray-700'
                    }`}
                  >
                    {formatPrice(dayInfo.effectivePrice)}
                  </div>
                  {dayInfo.isSpecialRate && dayInfo.discount !== 0 && (
                    <div
                      className={`text-[10px] ${
                        dayInfo.discount > 0 ? 'text-green-400' : 'text-red-600'
                      }`}
                    >
                      {dayInfo.discount > 0 ? '-' : '+'}
                      {Math.abs(dayInfo.discountPercent).toFixed(0)}%
                    </div>
                  )}
                </div>
              )}
            </div>
          )
        })}
      </div>

      {/* Legend */}
      <div className="flex flex-wrap items-center gap-4 p-4 bg-gray-100 border-t border-gray-200">
        <div className="flex items-center gap-2">
          <span className="w-4 h-4 bg-white border border-gray-200 rounded"></span>
          <span className="text-sm text-gray-500">ราคาปกติ</span>
        </div>
        <div className="flex items-center gap-2">
          <span className="w-4 h-4 bg-green-500/20 border border-green-500 rounded"></span>
          <span className="text-sm text-gray-500">ลดราคา</span>
        </div>
        <div className="flex items-center gap-2">
          <span className="w-4 h-4 bg-red-500/20 border border-red-500 rounded"></span>
          <span className="text-sm text-gray-500">ราคาพิเศษ (เพิ่ม)</span>
        </div>
        <div className="flex items-center gap-2">
          <span className="w-4 h-4 border-2 border-red-500 rounded"></span>
          <span className="text-sm text-gray-500">วันนี้</span>
        </div>
      </div>

      {/* Day Detail Modal */}
      {selectedDay && (
        <>
          {/* Backdrop */}
          <div
            className="fixed inset-0 bg-black/30 z-40"
            onClick={closeDetail}
          />

          {/* Detail Card */}
          <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
            <div className="bg-white border border-gray-200 rounded-lg shadow-xl w-full max-w-sm overflow-hidden">
              {/* Header */}
              <div className="flex items-center justify-between p-4 bg-gray-100 border-b border-gray-200">
                <div className="flex items-center gap-2">
                  <Calendar className="w-5 h-5 text-gray-500" />
                  <span className="font-semibold text-gray-800">
                    {format(selectedDay.date, 'd')} {THAI_MONTHS[selectedDay.date.getMonth()]}{' '}
                    {selectedDay.date.getFullYear() + 543}
                  </span>
                </div>
                <button
                  onClick={closeDetail}
                  className="p-1 hover:bg-gray-100 rounded-full transition-colors"
                  aria-label="ปิด"
                >
                  <X className="w-5 h-5" />
                </button>
              </div>

              {/* Content */}
              <div className="p-4 space-y-4">
                {/* Room Type */}
                <div>
                  <p className="text-sm text-gray-500">ประเภทห้อง</p>
                  <p className="font-medium text-gray-800">{roomType?.typeName}</p>
                </div>

                {/* Prices */}
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <p className="text-sm text-gray-500">ราคาปกติ</p>
                    <p className="font-medium text-gray-800">
                      {formatPrice(selectedDay.basePrice)} บาท
                    </p>
                  </div>
                  <div>
                    <p className="text-sm text-gray-500">ราคาวันนี้</p>
                    <p
                      className={`font-bold text-lg ${
                        selectedDay.discount > 0
                          ? 'text-green-400'
                          : selectedDay.discount < 0
                          ? 'text-red-600'
                          : 'text-gray-800'
                      }`}
                    >
                      {formatPrice(selectedDay.effectivePrice)} บาท
                    </p>
                  </div>
                </div>

                {/* Discount Info */}
                {selectedDay.discount !== 0 && (
                  <div
                    className={`p-3 rounded-lg ${
                      selectedDay.discount > 0
                        ? 'bg-green-500/10 border border-green-500'
                        : 'bg-red-500/10 border border-red-500'
                    }`}
                  >
                    <div className="flex items-center gap-2">
                      <Percent
                        className={`w-4 h-4 ${
                          selectedDay.discount > 0 ? 'text-green-400' : 'text-red-600'
                        }`}
                      />
                      <span
                        className={`font-medium ${
                          selectedDay.discount > 0 ? 'text-green-400' : 'text-red-600'
                        }`}
                      >
                        {selectedDay.discount > 0 ? 'ลด' : 'เพิ่ม'}{' '}
                        {Math.abs(selectedDay.discountPercent).toFixed(0)}%
                      </span>
                    </div>
                    <p
                      className={`text-sm mt-1 ${
                        selectedDay.discount > 0 ? 'text-green-400' : 'text-red-600'
                      }`}
                    >
                      {selectedDay.discount > 0 ? 'ประหยัด' : 'เพิ่มขึ้น'}{' '}
                      {formatPrice(Math.abs(selectedDay.discount))} บาท
                    </p>
                  </div>
                )}

                {/* Applied Rate */}
                {selectedDay.appliedRate && (
                  <div className="p-3 bg-gray-100 rounded-lg">
                    <div className="flex items-center gap-2 mb-2">
                      <Tag className="w-4 h-4 text-gray-500" />
                      <span className="font-medium text-gray-700">อัตราที่ใช้</span>
                    </div>
                    <p className="text-sm text-gray-500">{selectedDay.appliedRate.rateName}</p>
                    <p className="text-xs text-gray-500 mt-1">
                      {selectedDay.appliedRate.rateType === 'multiplier'
                        ? `ตัวคูณ: ${selectedDay.appliedRate.rateValue}`
                        : `ราคาคงที่: ${formatPrice(selectedDay.appliedRate.rateValue)} บาท`}
                    </p>
                  </div>
                )}

                {!selectedDay.appliedRate && (
                  <div className="p-3 bg-gray-100 rounded-lg">
                    <p className="text-sm text-gray-500">ไม่มีอัตราพิเศษสำหรับวันนี้</p>
                  </div>
                )}
              </div>
            </div>
          </div>
        </>
      )}
    </div>
  )
}
