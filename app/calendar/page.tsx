'use client'

import { useState, useEffect, useMemo } from 'react'
import { format, parseISO, differenceInDays } from 'date-fns'
import { Calendar as CalendarIcon, X, Home, Clock, User } from 'lucide-react'
import StayTimeline, { Stay } from '@/components/StayTimeline'

interface ApiBooking {
  bookNo: string
  checkIn: string
  checkOut: string
  customer: { name: string }
  rooms: { roomNo: string; roomType: string }[]
  status: string
}

interface ApiCheckIn {
  Cin_no: string
  Cin_Room_No: string
  Cin_Room_In: string
  Cin_Room_Out: string
  Cin_cust_name: string
  Cin_status: string
}

export default function CalendarPage() {
  const [selectedMonth, setSelectedMonth] = useState(new Date())
  const [bookings, setBookings] = useState<ApiBooking[]>([])
  const [checkins, setCheckins] = useState<ApiCheckIn[]>([])
  const [loading, setLoading] = useState(true)
  const [selectedStay, setSelectedStay] = useState<Stay | null>(null)

  // Fetch bookings and check-ins for the selected month
  useEffect(() => {
    const fetchData = async () => {
      setLoading(true)
      try {
        const year = selectedMonth.getFullYear()
        const month = selectedMonth.getMonth()

        // Get first day of previous month and last day of next month for overlap
        const startDate = new Date(year, month - 1, 1).toISOString().split('T')[0]
        const endDate = new Date(year, month + 2, 0).toISOString().split('T')[0]

        const [bookingsRes, checkinsRes] = await Promise.all([
          fetch(`/api/bookings?startDate=${startDate}&endDate=${endDate}&limit=500`),
          fetch(`/api/checkins?startDate=${startDate}&endDate=${endDate}&limit=500`),
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
  }, [selectedMonth])

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
        customerName: checkin.Cin_cust_name?.trim() || 'ไม่ระบุชื่อ',
        roomNumber: checkin.Cin_Room_No || '',
        checkIn,
        checkOut,
        type: 'checkin',
        status: checkin.Cin_status,
        nights,
      })
    })

    // Process bookings (future reservations) - only those not yet checked in
    bookings.forEach((booking, index) => {
      if (!booking.checkIn || !booking.checkOut) return

      // Skip if this booking has a matching check-in
      const hasCheckin = checkins.some(c =>
        c.Cin_cust_name?.trim() === booking.customer?.name?.trim() &&
        c.Cin_Room_In?.split('T')[0] === booking.checkIn?.split('T')[0]
      )
      if (hasCheckin) return

      const checkIn = parseISO(booking.checkIn)
      const checkOut = parseISO(booking.checkOut)
      const nights = Math.max(differenceInDays(checkOut, checkIn), 1)

      result.push({
        id: `booking-${booking.bookNo || index}`,
        customerName: booking.customer?.name?.trim() || 'ไม่ระบุชื่อ',
        roomNumber: booking.rooms?.[0]?.roomNo || '',
        checkIn,
        checkOut,
        type: 'booking',
        status: booking.status,
        nights,
      })
    })

    return result
  }, [bookings, checkins])

  const formatThaiDate = (date: Date) => {
    const thaiMonths = [
      'มกราคม', 'กุมภาพันธ์', 'มีนาคม', 'เมษายน', 'พฤษภาคม', 'มิถุนายน',
      'กรกฎาคม', 'สิงหาคม', 'กันยายน', 'ตุลาคม', 'พฤศจิกายน', 'ธันวาคม'
    ]
    return `${date.getDate()} ${thaiMonths[date.getMonth()]} ${date.getFullYear() + 543}`
  }

  const handleStayClick = (stay: Stay) => {
    setSelectedStay(stay)
  }

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <CalendarIcon className="w-8 h-8 text-blue-600" />
          <div>
            <h1 className="text-2xl font-bold text-gray-800">ไทม์ไลน์การเข้าพัก</h1>
            <p className="text-gray-600">ดูการเข้าพักและการจองแบบต่อเนื่อง</p>
          </div>
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
          selectedMonth={selectedMonth}
          onMonthChange={setSelectedMonth}
          onStayClick={handleStayClick}
        />
      )}

      {/* Stay Detail Modal */}
      {selectedStay && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-xl shadow-xl w-full max-w-md overflow-hidden">
            {/* Modal Header */}
            <div className={`flex items-center justify-between p-4 ${selectedStay.type === 'checkin' ? 'bg-blue-50' : 'bg-amber-50'}`}>
              <div className="flex items-center gap-3">
                <div className={`w-10 h-10 rounded-full flex items-center justify-center ${selectedStay.type === 'checkin' ? 'bg-blue-100' : 'bg-amber-100'}`}>
                  <User className={`w-5 h-5 ${selectedStay.type === 'checkin' ? 'text-blue-600' : 'text-amber-600'}`} />
                </div>
                <div>
                  <h2 className="text-lg font-bold text-gray-800">
                    {selectedStay.customerName}
                  </h2>
                  <span className={`text-xs px-2 py-0.5 rounded-full ${selectedStay.type === 'checkin' ? 'bg-blue-100 text-blue-700' : 'bg-amber-100 text-amber-700'}`}>
                    {selectedStay.type === 'checkin' ? 'เช็คอินแล้ว' : 'การจอง'}
                  </span>
                </div>
              </div>
              <button
                onClick={() => setSelectedStay(null)}
                className="p-2 hover:bg-gray-200 rounded-full"
                aria-label="ปิด"
              >
                <X className="w-5 h-5" />
              </button>
            </div>

            {/* Modal Content */}
            <div className="p-4 space-y-4">
              {selectedStay.roomNumber && (
                <div className="flex items-center gap-3">
                  <Home className="w-5 h-5 text-gray-400" />
                  <div>
                    <p className="text-sm text-gray-500">ห้องพัก</p>
                    <p className="font-medium text-gray-800">{selectedStay.roomNumber}</p>
                  </div>
                </div>
              )}

              <div className="flex items-center gap-3">
                <Clock className="w-5 h-5 text-gray-400" />
                <div>
                  <p className="text-sm text-gray-500">ระยะเวลาเข้าพัก</p>
                  <p className="font-medium text-gray-800">
                    {formatThaiDate(selectedStay.checkIn)} - {formatThaiDate(selectedStay.checkOut)}
                  </p>
                  <p className="text-sm text-gray-500">({selectedStay.nights} คืน)</p>
                </div>
              </div>

              {selectedStay.status && (
                <div className="flex items-center gap-3">
                  <CalendarIcon className="w-5 h-5 text-gray-400" />
                  <div>
                    <p className="text-sm text-gray-500">สถานะ</p>
                    <p className="font-medium text-gray-800">{selectedStay.status}</p>
                  </div>
                </div>
              )}
            </div>

            {/* Modal Footer */}
            <div className="p-4 border-t bg-gray-50">
              <button
                onClick={() => setSelectedStay(null)}
                className="w-full px-4 py-2 bg-gray-200 hover:bg-gray-300 text-gray-800 rounded-lg"
              >
                ปิด
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
