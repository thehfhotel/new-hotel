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
  customer: {
    name: string
  }
  roomCount: number
}

interface ApiCheckIn {
  Cin_no: string
  Cin_Room_No: string
  Cin_Room_In: string
  Cin_Room_Out: string
  Cin_cust_name: string
  Cin_status: string
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
        checkinNo: checkin.Cin_no,
        roomNo: checkin.Cin_Room_No,
        customerName: checkin.Cin_cust_name,
        status: checkin.Cin_status,
      })
    })

    // Build a map of check-in customer names (normalized) with their date ranges
    const checkinCustomers = new Map<string, { start: string; end: string }[]>()
    checkins.forEach(c => {
      if (!c.Cin_cust_name || !c.Cin_Room_In || !c.Cin_Room_Out) return
      const name = c.Cin_cust_name.trim().toLowerCase()
      const range = { start: c.Cin_Room_In.split('T')[0], end: c.Cin_Room_Out.split('T')[0] }
      if (!checkinCustomers.has(name)) {
        checkinCustomers.set(name, [])
      }
      checkinCustomers.get(name)!.push(range)
    })

    // Helper to check if a booking has a matching check-in (same customer, overlapping dates)
    const hasMatchingCheckin = (customerName: string | undefined, bookingStart: string, bookingEnd: string): boolean => {
      if (!customerName) return false
      const name = customerName.trim().toLowerCase()
      const ranges = checkinCustomers.get(name)
      if (!ranges) return false
      // Check if any check-in date range overlaps with the booking
      return ranges.some(r => r.start <= bookingEnd && r.end >= bookingStart)
    }

    // Process bookings - exclude those with matching check-ins (same customer + overlapping dates)
    bookings.forEach((booking, index) => {
      if (!booking.checkIn || !booking.checkOut) return
      // Only show bookings with status "จอง" (pending, not yet checked in)
      if (booking.status !== 'จอง') return

      const id = `booking-${booking.bookNo || index}`
      if (seenIds.has(id)) return
      seenIds.add(id)

      // Skip if this booking has a matching check-in (customer already checked in)
      const bookingStart = booking.checkIn.split('T')[0]
      const bookingEnd = booking.checkOut.split('T')[0]
      if (hasMatchingCheckin(booking.customer?.name, bookingStart, bookingEnd)) return

      const checkIn = parseISO(booking.checkIn)
      const checkOut = parseISO(booking.checkOut)
      const nights = Math.max(differenceInDays(checkOut, checkIn), 1)

      result.push({
        id,
        checkIn,
        checkOut,
        type: 'booking',
        nights,
        bookNo: booking.bookNo,
        bookDate: booking.bookDate ? parseISO(booking.bookDate) : undefined,
        customerName: booking.customer?.name,
        status: booking.status,
        roomCount: booking.roomCount,
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
