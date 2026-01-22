'use client'

import { useState, useEffect } from 'react'
import { format } from 'date-fns'
import { X, Calendar as CalendarIcon, Filter, User, Home, Clock } from 'lucide-react'
import Calendar, { Booking, CheckIn } from '@/components/Calendar'

interface RoomType {
  id: number
  name: string
}

export default function CalendarPage() {
  const [selectedMonth, setSelectedMonth] = useState(new Date())
  const [bookings, setBookings] = useState<Booking[]>([])
  const [checkins, setCheckins] = useState<CheckIn[]>([])
  const [roomTypes, setRoomTypes] = useState<RoomType[]>([])
  const [selectedRoomType, setSelectedRoomType] = useState<string>('')
  const [loading, setLoading] = useState(true)
  const [selectedDate, setSelectedDate] = useState<Date | null>(null)
  const [selectedBookings, setSelectedBookings] = useState<Booking[]>([])
  const [selectedCheckins, setSelectedCheckins] = useState<CheckIn[]>([])
  const [showModal, setShowModal] = useState(false)

  // Fetch room types
  useEffect(() => {
    const fetchRoomTypes = async () => {
      try {
        const response = await fetch('/api/rooms')
        if (response.ok) {
          const result = await response.json()
          const roomsData = result.data || result || []
          // Extract unique room types
          const types = Array.from(
            new Set((Array.isArray(roomsData) ? roomsData : []).map((room: { Room_Type: string }) => room.Room_Type?.trim()))
          ).filter(Boolean).map((type, index) => ({ id: index, name: type as string }))
          setRoomTypes(types)
        }
      } catch (error) {
        console.error('Error fetching room types:', error)
      }
    }
    fetchRoomTypes()
  }, [])

  // Fetch bookings and check-ins
  useEffect(() => {
    const fetchData = async () => {
      setLoading(true)
      try {
        const year = selectedMonth.getFullYear()
        const month = selectedMonth.getMonth() + 1

        const [bookingsRes, checkinsRes] = await Promise.all([
          fetch(`/api/bookings?year=${year}&month=${month}${selectedRoomType ? `&roomType=${selectedRoomType}` : ''}`),
          fetch(`/api/checkins?year=${year}&month=${month}${selectedRoomType ? `&roomType=${selectedRoomType}` : ''}`),
        ])

        if (bookingsRes.ok) {
          const bookingsData = await bookingsRes.json()
          // API returns { success: true, data: [...] }
          const rawBookings = bookingsData.data || bookingsData.bookings || []
          // Map API data to Booking interface
          const mappedBookings: Booking[] = (Array.isArray(rawBookings) ? rawBookings : []).map((b: {
            Book_No: string
            Book_Cust_Name: string
            Book_Room_Type: string
            Book_Date_in: string
            Book_Date_out: string
            Book_Status: number | string
          }, idx: number) => ({
            id: idx + 1,
            customerId: idx + 1,
            customerName: b.Book_Cust_Name || '',
            roomNumber: b.Book_Room_Type || '',
            roomType: b.Book_Room_Type || '',
            checkInDate: b.Book_Date_in || '',
            checkOutDate: b.Book_Date_out || '',
            status: String(b.Book_Status) === '1' ? 'confirmed' : 'pending',
          }))
          setBookings(mappedBookings)
        }

        if (checkinsRes.ok) {
          const checkinsData = await checkinsRes.json()
          // API returns { success: true, data: [...] }
          const rawCheckins = checkinsData.data || checkinsData.checkins || []
          // Map API data to CheckIn interface
          const mappedCheckins: CheckIn[] = (Array.isArray(rawCheckins) ? rawCheckins : []).map((c: {
            Cin_no: string
            Cin_cust_name: string
            Cin_Room_No: string
            Cin_Room_In: string
            Cin_Room_Out: string
            Cin_status: string
          }, idx: number) => ({
            id: idx + 1,
            customerId: idx + 1,
            customerName: c.Cin_cust_name || '',
            roomNumber: c.Cin_Room_No || '',
            roomType: '',
            checkInTime: c.Cin_Room_In || '',
            checkOutTime: c.Cin_Room_Out || '',
          }))
          setCheckins(mappedCheckins)
        }
      } catch (error) {
        console.error('Error fetching data:', error)
      } finally {
        setLoading(false)
      }
    }

    fetchData()
  }, [selectedMonth, selectedRoomType])

  const handleDateClick = (date: Date, dayBookings: Booking[], dayCheckins: CheckIn[]) => {
    setSelectedDate(date)
    setSelectedBookings(dayBookings)
    setSelectedCheckins(dayCheckins)
    setShowModal(true)
  }

  const closeModal = () => {
    setShowModal(false)
    setSelectedDate(null)
    setSelectedBookings([])
    setSelectedCheckins([])
  }

  const formatThaiDate = (date: Date) => {
    const thaiMonths = [
      'มกราคม', 'กุมภาพันธ์', 'มีนาคม', 'เมษายน', 'พฤษภาคม', 'มิถุนายน',
      'กรกฎาคม', 'สิงหาคม', 'กันยายน', 'ตุลาคม', 'พฤศจิกายน', 'ธันวาคม'
    ]
    return `${date.getDate()} ${thaiMonths[date.getMonth()]} ${date.getFullYear() + 543}`
  }

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <CalendarIcon className="w-8 h-8 text-blue-600" />
          <div>
            <h1 className="text-2xl font-bold text-gray-800">ปฏิทินการจอง</h1>
            <p className="text-gray-600">ดูการจองและเช็คอินรายวัน</p>
          </div>
        </div>
      </div>

      {/* Filters */}
      <div className="bg-white rounded-lg shadow p-4">
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-2">
            <Filter className="w-5 h-5 text-gray-500" />
            <span className="text-sm font-medium text-gray-700">กรองตาม:</span>
          </div>
          <div className="flex-1 max-w-xs">
            <select
              value={selectedRoomType}
              onChange={(e) => setSelectedRoomType(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors"
            >
              <option value="">ประเภทห้องทั้งหมด</option>
              {roomTypes.map((type) => (
                <option key={type.id} value={type.name}>
                  {type.name}
                </option>
              ))}
            </select>
          </div>
          {selectedRoomType && (
            <button
              onClick={() => setSelectedRoomType('')}
              className="text-sm text-blue-600 hover:text-blue-800 transition-colors"
            >
              ล้างตัวกรอง
            </button>
          )}
        </div>
      </div>

      {/* Calendar */}
      {loading ? (
        <div className="bg-white rounded-lg shadow-lg p-12 flex items-center justify-center">
          <div className="flex items-center gap-3">
            <div className="w-6 h-6 border-2 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
            <span className="text-gray-600">กำลังโหลดข้อมูล...</span>
          </div>
        </div>
      ) : (
        <Calendar
          bookings={bookings}
          checkins={checkins}
          selectedMonth={selectedMonth}
          onMonthChange={setSelectedMonth}
          onDateClick={handleDateClick}
        />
      )}

      {/* Summary Stats */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="bg-white rounded-lg shadow p-4">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 bg-green-100 rounded-full flex items-center justify-center">
              <CalendarIcon className="w-5 h-5 text-green-600" />
            </div>
            <div>
              <p className="text-sm text-gray-600">จำนวนการจองในเดือนนี้</p>
              <p className="text-2xl font-bold text-gray-800">{bookings.length}</p>
            </div>
          </div>
        </div>
        <div className="bg-white rounded-lg shadow p-4">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 bg-blue-100 rounded-full flex items-center justify-center">
              <User className="w-5 h-5 text-blue-600" />
            </div>
            <div>
              <p className="text-sm text-gray-600">จำนวนเช็คอินในเดือนนี้</p>
              <p className="text-2xl font-bold text-gray-800">{checkins.length}</p>
            </div>
          </div>
        </div>
        <div className="bg-white rounded-lg shadow p-4">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 bg-purple-100 rounded-full flex items-center justify-center">
              <Home className="w-5 h-5 text-purple-600" />
            </div>
            <div>
              <p className="text-sm text-gray-600">ประเภทห้องที่เลือก</p>
              <p className="text-xl font-bold text-gray-800">
                {selectedRoomType || 'ทั้งหมด'}
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Date Details Modal */}
      {showModal && selectedDate && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-lg shadow-xl w-full max-w-2xl max-h-[90vh] overflow-hidden">
            {/* Modal Header */}
            <div className="flex items-center justify-between p-4 border-b bg-gray-50">
              <div>
                <h2 className="text-xl font-bold text-gray-800">
                  รายละเอียดวันที่ {formatThaiDate(selectedDate)}
                </h2>
                <p className="text-sm text-gray-600">
                  การจอง {selectedBookings.length} รายการ | เช็คอิน {selectedCheckins.length} รายการ
                </p>
              </div>
              <button
                onClick={closeModal}
                className="p-2 hover:bg-gray-200 rounded-full transition-colors"
                aria-label="ปิด"
              >
                <X className="w-5 h-5" />
              </button>
            </div>

            {/* Modal Content */}
            <div className="p-4 overflow-y-auto max-h-[60vh]">
              {/* Bookings Section */}
              {selectedBookings.length > 0 && (
                <div className="mb-6">
                  <h3 className="flex items-center gap-2 text-lg font-semibold text-gray-800 mb-3">
                    <span className="w-3 h-3 bg-green-500 rounded-full"></span>
                    การจอง ({selectedBookings.length})
                  </h3>
                  <div className="space-y-2">
                    {selectedBookings.map((booking) => (
                      <div
                        key={booking.id}
                        className="p-3 border border-gray-200 rounded-lg hover:bg-gray-50"
                      >
                        <div className="flex items-start justify-between">
                          <div>
                            <p className="font-medium text-gray-800">
                              {booking.customerName}
                            </p>
                            <div className="flex items-center gap-4 text-sm text-gray-600 mt-1">
                              <span className="flex items-center gap-1">
                                <Home className="w-4 h-4" />
                                ห้อง {booking.roomNumber}
                              </span>
                              <span>{booking.roomType}</span>
                            </div>
                            <div className="flex items-center gap-1 text-sm text-gray-500 mt-1">
                              <Clock className="w-4 h-4" />
                              {format(new Date(booking.checkInDate), 'dd/MM/yyyy')} -{' '}
                              {format(new Date(booking.checkOutDate), 'dd/MM/yyyy')}
                            </div>
                          </div>
                          <span
                            className={`px-2 py-1 text-xs rounded-full ${
                              booking.status === 'confirmed'
                                ? 'bg-green-100 text-green-700'
                                : booking.status === 'pending'
                                ? 'bg-yellow-100 text-yellow-700'
                                : 'bg-gray-100 text-gray-700'
                            }`}
                          >
                            {booking.status === 'confirmed'
                              ? 'ยืนยันแล้ว'
                              : booking.status === 'pending'
                              ? 'รอยืนยัน'
                              : booking.status}
                          </span>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* Check-ins Section */}
              {selectedCheckins.length > 0 && (
                <div>
                  <h3 className="flex items-center gap-2 text-lg font-semibold text-gray-800 mb-3">
                    <span className="w-3 h-3 bg-blue-500 rounded-full"></span>
                    เช็คอิน ({selectedCheckins.length})
                  </h3>
                  <div className="space-y-2">
                    {selectedCheckins.map((checkin) => (
                      <div
                        key={checkin.id}
                        className="p-3 border border-gray-200 rounded-lg hover:bg-gray-50"
                      >
                        <div className="flex items-start justify-between">
                          <div>
                            <p className="font-medium text-gray-800">
                              {checkin.customerName}
                            </p>
                            <div className="flex items-center gap-4 text-sm text-gray-600 mt-1">
                              <span className="flex items-center gap-1">
                                <Home className="w-4 h-4" />
                                ห้อง {checkin.roomNumber}
                              </span>
                              <span>{checkin.roomType}</span>
                            </div>
                            <div className="flex items-center gap-1 text-sm text-gray-500 mt-1">
                              <Clock className="w-4 h-4" />
                              เช็คอิน: {format(new Date(checkin.checkInTime), 'HH:mm น.')}
                              {checkin.checkOutTime && (
                                <span className="ml-2">
                                  เช็คเอาท์: {format(new Date(checkin.checkOutTime), 'HH:mm น.')}
                                </span>
                              )}
                            </div>
                          </div>
                          <span className="px-2 py-1 text-xs rounded-full bg-blue-100 text-blue-700">
                            เข้าพัก
                          </span>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* Empty State */}
              {selectedBookings.length === 0 && selectedCheckins.length === 0 && (
                <div className="text-center py-8">
                  <CalendarIcon className="w-12 h-12 text-gray-300 mx-auto mb-3" />
                  <p className="text-gray-500">ไม่มีข้อมูลการจองหรือเช็คอินในวันนี้</p>
                </div>
              )}
            </div>

            {/* Modal Footer */}
            <div className="p-4 border-t bg-gray-50">
              <button
                onClick={closeModal}
                className="w-full px-4 py-2 bg-gray-200 hover:bg-gray-300 text-gray-800 rounded-lg transition-colors"
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
