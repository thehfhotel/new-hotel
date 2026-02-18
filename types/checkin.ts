import { PaginationInfo } from './common'

export interface CheckIn {
  id: number
  checkinNo: string
  bookingId: number | null
  customerId: number
  customerName: string | null
  roomId: number
  roomNo: string
  checkInDate: string
  checkOutDate: string | null
  status: string
  adults: number
  children: number
  notes: string | null
  createdAt: string
}

export interface CheckInsResponse {
  success: boolean
  data: CheckIn[]
  pagination: PaginationInfo
}
