'use client'

import { useState, useEffect, useMemo, useRef, useCallback } from 'react'
import { parseISO, differenceInDays, startOfMonth, endOfMonth, addMonths, subMonths, format } from 'date-fns'
import { Calendar as CalendarIcon } from 'lucide-react'
import StayTimeline, { Stay } from '@/components/StayTimeline'

interface ApiBooking {
  bookNo: string
  bookDate: string
  checkIn: string
  checkOut: string
  status: string
}

interface ApiCheckIn {
  Cin_no: string
  Cin_Room_In: string
  Cin_Room_Out: string
}

interface CachedData {
  bookings: ApiBooking[]
  checkins: ApiCheckIn[]
}

export default function CalendarPage() {
  const [selectedDate, setSelectedDate] = useState(new Date())
  const [viewMode, setViewMode] = useState<'week' | 'month'>('week')
  const [loading, setLoading] = useState(true)
  const [dataVersion, setDataVersion] = useState(0)

  // Cache data by month key (yyyy-MM)
  const cacheRef = useRef<Map<string, CachedData>>(new Map())

  // Get cache key for a date (by month)
  const getCacheKey = (date: Date) => format(date, 'yyyy-MM')

  // Fetch data for a specific month
  const fetchMonth = useCallback(async (date: Date, isBackground = false): Promise<CachedData | null> => {
    const key = getCacheKey(date)

    // Return cached data if available
    if (cacheRef.current.has(key)) {
      return cacheRef.current.get(key)!
    }

    try {
      const monthStart = startOfMonth(date)
      const monthEnd = endOfMonth(date)
      const startDate = format(monthStart, 'yyyy-MM-dd')
      const endDate = format(monthEnd, 'yyyy-MM-dd')

      const [bookingsRes, checkinsRes] = await Promise.all([
        fetch(`/api/bookings?startDate=${startDate}&endDate=${endDate}&limit=1000`),
        fetch(`/api/checkins?startDate=${startDate}&endDate=${endDate}&limit=1000`),
      ])

      const bookings = bookingsRes.ok ? (await bookingsRes.json()).data || [] : []
      const checkins = checkinsRes.ok ? (await checkinsRes.json()).data || [] : []

      const data = { bookings, checkins }
      cacheRef.current.set(key, data)

      // Trigger re-render for non-background fetches
      if (!isBackground) {
        setDataVersion(v => v + 1)
      }

      return data
    } catch (error) {
      console.error('Error fetching data:', error)
      return null
    }
  }, [])

  // Initial load and when date changes
  useEffect(() => {
    const loadData = async () => {
      const key = getCacheKey(selectedDate)

      // If already cached, no loading needed
      if (cacheRef.current.has(key)) {
        setLoading(false)
        // Preload adjacent months in background
        fetchMonth(subMonths(selectedDate, 1), true)
        fetchMonth(addMonths(selectedDate, 1), true)
        return
      }

      setLoading(true)

      // Fetch current and adjacent months
      await Promise.all([
        fetchMonth(selectedDate),
        fetchMonth(subMonths(selectedDate, 1), true),
        fetchMonth(addMonths(selectedDate, 1), true),
      ])

      setLoading(false)
    }

    loadData()
  }, [selectedDate, fetchMonth])

  // Get combined data from current and adjacent months
  const allData = useMemo(() => {
    // Include dataVersion in deps to trigger recalc when cache updates
    void dataVersion

    const prevKey = getCacheKey(subMonths(selectedDate, 1))
    const currKey = getCacheKey(selectedDate)
    const nextKey = getCacheKey(addMonths(selectedDate, 1))

    const prev = cacheRef.current.get(prevKey) || { bookings: [], checkins: [] }
    const curr = cacheRef.current.get(currKey) || { bookings: [], checkins: [] }
    const next = cacheRef.current.get(nextKey) || { bookings: [], checkins: [] }

    return {
      bookings: [...prev.bookings, ...curr.bookings, ...next.bookings],
      checkins: [...prev.checkins, ...curr.checkins, ...next.checkins],
    }
  }, [selectedDate, dataVersion])

  // Transform API data to Stay format
  const stays: Stay[] = useMemo(() => {
    const { bookings, checkins } = allData
    const result: Stay[] = []
    const seenIds = new Set<string>()

    // Process check-ins (actual stays)
    checkins.forEach((checkin, index) => {
      if (!checkin.Cin_Room_In || !checkin.Cin_Room_Out) return

      const id = `checkin-${checkin.Cin_no || index}`
      if (seenIds.has(id)) return
      seenIds.add(id)

      const checkIn = parseISO(checkin.Cin_Room_In)
      const checkOut = parseISO(checkin.Cin_Room_Out)
      const nights = Math.max(differenceInDays(checkOut, checkIn), 1)

      result.push({
        id,
        checkIn,
        checkOut,
        type: 'checkin',
        nights,
      })
    })

    // Process bookings - exclude those that have matching check-ins
    const checkinKeys = new Set(
      checkins.map(c => `${c.Cin_Room_In?.split('T')[0]}_${c.Cin_Room_Out?.split('T')[0]}`)
    )

    bookings.forEach((booking, index) => {
      if (!booking.checkIn || !booking.checkOut) return

      const id = `booking-${booking.bookNo || index}`
      if (seenIds.has(id)) return
      seenIds.add(id)

      const bookingKey = `${booking.checkIn.split('T')[0]}_${booking.checkOut.split('T')[0]}`
      if (checkinKeys.has(bookingKey)) return

      const checkIn = parseISO(booking.checkIn)
      const checkOut = parseISO(booking.checkOut)
      const nights = Math.max(differenceInDays(checkOut, checkIn), 1)

      result.push({
        id,
        checkIn,
        checkOut,
        type: 'booking',
        nights,
        bookDate: booking.bookDate ? parseISO(booking.bookDate) : undefined,
      })
    })

    return result
  }, [allData])

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center gap-3">
        <CalendarIcon className="w-8 h-8 text-blue-600" />
        <div>
          <h1 className="text-2xl font-bold text-gray-800">ไทม์ไลน์การเข้าพัก</h1>
          <p className="text-gray-600">ดูรูปแบบการเข้าพักและระยะเวลา</p>
        </div>
      </div>

      {/* Timeline */}
      {loading ? (
        <div className="bg-white rounded-xl shadow-sm border border-gray-100 p-12 flex items-center justify-center">
          <div className="flex items-center gap-3">
            <div className="w-6 h-6 border-2 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
            <span className="text-gray-600">กำลังโหลดข้อมูล...</span>
          </div>
        </div>
      ) : (
        <StayTimeline
          stays={stays}
          selectedDate={selectedDate}
          onDateChange={setSelectedDate}
          viewMode={viewMode}
          onViewModeChange={setViewMode}
        />
      )}
    </div>
  )
}
