'use client'

import { useState, useEffect, useMemo } from 'react'
import { parseISO, differenceInDays } from 'date-fns'
import { Calendar as CalendarIcon } from 'lucide-react'
import StayTimeline, { Stay } from '@/components/StayTimeline'
import { startOfMonth, endOfMonth } from 'date-fns'

interface ApiBooking {
  bookNo: string
  checkIn: string
  checkOut: string
  status: string
}

interface ApiCheckIn {
  Cin_no: string
  Cin_Room_In: string
  Cin_Room_Out: string
}

export default function CalendarPage() {
  const [selectedDate, setSelectedDate] = useState(new Date())
  const [bookings, setBookings] = useState<ApiBooking[]>([])
  const [checkins, setCheckins] = useState<ApiCheckIn[]>([])
  const [loading, setLoading] = useState(true)

  // Fetch bookings and check-ins for the selected month
  useEffect(() => {
    const fetchData = async () => {
      setLoading(true)
      try {
        // Get 3 months of data centered on selected date for smooth navigation
        const monthStart = startOfMonth(selectedDate)
        const startDate = new Date(monthStart.getFullYear(), monthStart.getMonth() - 1, 1).toISOString().split('T')[0]
        const endDate = new Date(monthStart.getFullYear(), monthStart.getMonth() + 2, 0).toISOString().split('T')[0]

        const [bookingsRes, checkinsRes] = await Promise.all([
          fetch(`/api/bookings?startDate=${startDate}&endDate=${endDate}&limit=1000`),
          fetch(`/api/checkins?startDate=${startDate}&endDate=${endDate}&limit=1000`),
        ])

        if (bookingsRes.ok) {
          const data = await bookingsRes.json()
          setBookings(data.data || [])
        }

        if (checkinsRes.ok) {
          const data = await checkinsRes.json()
          setCheckins(data.data || [])
        }
      } catch (error) {
        console.error('Error fetching data:', error)
      } finally {
        setLoading(false)
      }
    }

    fetchData()
  }, [selectedDate])

  // Transform API data to Stay format
  const stays: Stay[] = useMemo(() => {
    const result: Stay[] = []

    // Process check-ins (actual stays)
    checkins.forEach((checkin, index) => {
      if (!checkin.Cin_Room_In || !checkin.Cin_Room_Out) return

      const checkIn = parseISO(checkin.Cin_Room_In)
      const checkOut = parseISO(checkin.Cin_Room_Out)
      const nights = Math.max(differenceInDays(checkOut, checkIn), 1)

      result.push({
        id: `checkin-${checkin.Cin_no || index}`,
        checkIn,
        checkOut,
        type: 'checkin',
        nights,
      })
    })

    // Process bookings (reservations) - exclude those that have matching check-ins
    const checkinKeys = new Set(
      checkins.map(c => `${c.Cin_Room_In?.split('T')[0]}_${c.Cin_Room_Out?.split('T')[0]}`)
    )

    bookings.forEach((booking, index) => {
      if (!booking.checkIn || !booking.checkOut) return

      const bookingKey = `${booking.checkIn.split('T')[0]}_${booking.checkOut.split('T')[0]}`
      // Skip if there's already a check-in with same dates (avoid double counting)
      if (checkinKeys.has(bookingKey)) return

      const checkIn = parseISO(booking.checkIn)
      const checkOut = parseISO(booking.checkOut)
      const nights = Math.max(differenceInDays(checkOut, checkIn), 1)

      result.push({
        id: `booking-${booking.bookNo || index}`,
        checkIn,
        checkOut,
        type: 'booking',
        nights,
      })
    })

    return result
  }, [bookings, checkins])

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
        />
      )}
    </div>
  )
}
