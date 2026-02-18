import { PaginationInfo } from './common'

export type RoomStatus = 'available' | 'occupied' | 'booked' | 'maintenance' | 'checkout_pending'

export interface Room {
  id: number
  roomNumber: string
  roomType: string
  roomTypeName: string | null
  floor: number | null
  status: RoomStatus
  isClean: boolean
  isMaintenance: boolean
  notes: string | null
}

export interface RoomType {
  id: number
  typeCode: string
  typeName: string
  basePrice: number
  maxGuests: number | null
}

export interface RoomOption {
  id: number
  roomNumber: string
  roomType: string
  typeName: string
  pricePerNight: number
}

export interface RoomsResponse {
  success: boolean
  data: Room[]
  pagination?: PaginationInfo
}
