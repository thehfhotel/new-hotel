import { PaginationInfo } from './common'

export interface BookingRoom {
  id: number
  roomId: number
  roomNo: string | null
  roomTypeName: string | null
  pricePerNight: number | null
}

export interface Booking {
  id: number
  bookNo: string
  customerId: number
  customerName: string | null
  checkIn: string | null
  checkOut: string | null
  nights: number | null
  adults: number | null
  children: number | null
  status: string
  /** `ht_bookings.book_source` — the coarse legacy origin ('ota', 'loyalty', …). */
  source: string | null
  /**
   * `ht_bookings.book_channel` (migration 076) — provenance, READ-ONLY.
   * `'loyalty'` = booked in the guest app, an OTA slug ('bookingcom', 'agoda',
   * …) = OTA-Desk write-back, `null` = walk-in / phone / manual desk booking.
   * The backend always emits the key (null when absent), so no `undefined`
   * check is needed. Rendered by `components/v2/BookingChannelChip`.
   */
  bookChannel: string | null
  totalAmount: number | null
  depositAmount: number | null
  notes: string | null
  roomCount: number
  createdAt: string | null
}

export interface BookingDetail extends Booking {
  rooms: BookingRoom[]
}

export interface BookingsResponse {
  success: boolean
  data: Booking[]
  pagination: PaginationInfo
}

export interface BookingFormData {
  bookId?: number
  customerId: number
  customerName?: string
  checkIn: string
  checkOut: string
  adults: number
  children: number
  status: string
  source: string
  depositAmount: number | null
  notes: string | null
  rooms: number[]
}

export interface LegacyBooking {
  bookNo: string
  bookDate: string
  checkIn: string
  checkOut: string
  customer: { name: string }
  status: string
  rooms: Array<{ roomNo: string; roomType: string }>
  roomCount: number
}
