/**
 * Mock Data Factories for Hotel Management System Tests
 *
 * These factories create mock data objects with sensible defaults
 * that can be overridden for specific test scenarios.
 */

import type { Booking, CheckIn } from '@/components/Calendar'
import type { Room, RoomStatus } from '@/components/RoomGrid'
import type { CustomerOption } from '@/components/pickers/CustomerPicker'
import type { RoomOption } from '@/components/pickers/RoomPicker'
import type { BookingFormData } from '@/components/forms/BookingForm'
import type {
  InvoiceData,
  InvoiceRoom,
  HotelInfo,
  ReceiptData,
} from '@/types/invoice'
import type {
  InventoryItem,
  InventoryCategory,
  InventoryTransaction,
  TransactionType,
  RoomInventory,
  RoomInventoryItem,
} from '@/types/inventory'

// ============================================================================
// Customer Mock Factory
// ============================================================================

export interface MockCustomer {
  id: number
  firstName: string
  lastName: string
  phone: string
  email: string
  idCard: string
  address?: string
  notes?: string
}

export const createMockCustomer = (overrides?: Partial<MockCustomer>): MockCustomer => ({
  id: 1,
  firstName: 'ทดสอบ',
  lastName: 'ลูกค้า',
  phone: '0812345678',
  email: 'test@example.com',
  idCard: '1234567890123',
  ...overrides,
})

export const createMockCustomerOption = (
  overrides?: Partial<CustomerOption>
): CustomerOption => ({
  id: 1,
  firstName: 'ทดสอบ',
  lastName: 'ลูกค้า',
  phone: '0812345678',
  email: 'test@example.com',
  idCard: '1234567890123',
  ...overrides,
})

// ============================================================================
// Room Mock Factory
// ============================================================================

export const createMockRoom = (overrides?: Partial<Room>): Room => ({
  id: 1,
  roomNumber: '301',
  type: 'Standard',
  details: 'Sea View',
  status: 'available' as RoomStatus,
  floor: 3,
  ...overrides,
})

export const createMockRoomOption = (overrides?: Partial<RoomOption>): RoomOption => ({
  id: 1,
  roomNo: '301',
  roomTypeId: 1,
  roomTypeName: 'Standard',
  floor: 3,
  status: 'available',
  priceWeekday: 1000,
  priceWeekend: 1200,
  ...overrides,
})

export const createMockRoomWithGuest = (overrides?: Partial<Room>): Room => ({
  id: 2,
  roomNumber: '302',
  type: 'Deluxe',
  details: 'City View',
  status: 'occupied' as RoomStatus,
  floor: 3,
  guestName: 'John Doe',
  checkIn: '2026-01-20',
  checkOut: '2026-01-25',
  ...overrides,
})

// ============================================================================
// Booking Mock Factory
// ============================================================================

export const createMockBooking = (overrides?: Partial<Booking>): Booking => ({
  id: 1,
  customerId: 101,
  customerName: 'ทดสอบ ลูกค้า',
  roomNumber: '301',
  roomType: 'Standard',
  checkInDate: '2026-01-15T00:00:00.000Z',
  checkOutDate: '2026-01-20T00:00:00.000Z',
  status: 'confirmed',
  ...overrides,
})

export const createMockBookingFormData = (
  overrides?: Partial<BookingFormData>
): BookingFormData => ({
  customerId: 1,
  customerName: 'ทดสอบ ลูกค้า',
  checkIn: '2026-01-15',
  checkOut: '2026-01-20',
  adults: 2,
  children: 0,
  status: 'pending',
  source: 'walk-in',
  depositAmount: null,
  notes: null,
  rooms: [{ roomId: 1, pricePerNight: 1000 }],
  ...overrides,
})

// ============================================================================
// CheckIn Mock Factory
// ============================================================================

export const createMockCheckIn = (overrides?: Partial<CheckIn>): CheckIn => ({
  id: 1,
  customerId: 101,
  customerName: 'ทดสอบ ลูกค้า',
  roomNumber: '301',
  roomType: 'Standard',
  checkInTime: '2026-01-15T14:00:00.000Z',
  ...overrides,
})

export interface CheckInDetails {
  id: number
  cinNo: string
  customerId: number
  customerName: string | null
  roomId: number
  roomNo: string | null
  checkInTime: string | null
  expectedCheckout: string | null
  ratePerNight: number | null
  status: string
}

export const createMockCheckInDetails = (
  overrides?: Partial<CheckInDetails>
): CheckInDetails => ({
  id: 1,
  cinNo: 'CIN-2026-0001',
  customerId: 101,
  customerName: 'ทดสอบ ลูกค้า',
  roomId: 1,
  roomNo: '301',
  checkInTime: '2026-01-15T14:00:00.000Z',
  expectedCheckout: '2026-01-20T00:00:00.000Z',
  ratePerNight: 1200,
  status: 'active',
  ...overrides,
})

// ============================================================================
// Inventory Mock Factory
// ============================================================================

export const createMockInventoryItem = (
  overrides?: Partial<InventoryItem>
): InventoryItem => ({
  id: 1,
  itemCode: 'MINI-001',
  itemName: 'น้ำดื่ม',
  category: 'Minibar' as InventoryCategory,
  unit: 'ขวด',
  minStock: 50,
  currentStock: 100,
  costPerUnit: 10,
  active: true,
  createdAt: '2026-01-01T00:00:00.000Z',
  updatedAt: '2026-01-15T00:00:00.000Z',
  ...overrides,
})

export const createMockInventoryTransaction = (
  overrides?: Partial<InventoryTransaction>
): InventoryTransaction => ({
  id: 1,
  itemId: 1,
  itemCode: 'MINI-001',
  itemName: 'น้ำดื่ม',
  transactionType: 'IN' as TransactionType,
  quantity: 50,
  previousStock: 50,
  newStock: 100,
  roomId: null,
  roomNumber: null,
  notes: 'รับเข้าสต็อก',
  createdBy: 'admin',
  createdAt: '2026-01-15T10:00:00.000Z',
  ...overrides,
})

export const createMockRoomInventory = (
  overrides?: Partial<RoomInventory>
): RoomInventory => ({
  roomId: 1,
  roomNumber: '301',
  roomType: 'Standard',
  items: [
    {
      itemId: 1,
      itemCode: 'MINI-001',
      itemName: 'น้ำดื่ม',
      category: 'Minibar' as InventoryCategory,
      unit: 'ขวด',
      assignedQuantity: 2,
      verifiedQuantity: null,
      lastVerified: null,
    },
    {
      itemId: 2,
      itemCode: 'AMEN-001',
      itemName: 'สบู่',
      category: 'Amenities' as InventoryCategory,
      unit: 'ก้อน',
      assignedQuantity: 1,
      verifiedQuantity: 1,
      lastVerified: '2026-01-15T08:00:00.000Z',
    },
  ],
  ...overrides,
})

export const createMockRoomInventoryItem = (
  overrides?: Partial<RoomInventoryItem>
): RoomInventoryItem => ({
  itemId: 1,
  itemCode: 'MINI-001',
  itemName: 'น้ำดื่ม',
  category: 'Minibar' as InventoryCategory,
  unit: 'ขวด',
  assignedQuantity: 2,
  verifiedQuantity: null,
  lastVerified: null,
  ...overrides,
})

// ============================================================================
// Invoice & Receipt Mock Factory
// ============================================================================

export const createMockInvoiceRoom = (
  overrides?: Partial<InvoiceRoom>
): InvoiceRoom => ({
  roomNumber: '301',
  roomType: 'Standard',
  ratePerNight: 1200,
  nights: 5,
  subtotal: 6000,
  ...overrides,
})

export const createMockInvoiceData = (
  overrides?: Partial<InvoiceData>
): InvoiceData => ({
  invoiceNumber: 'INV-2026-0001',
  checkInId: 1,
  guestName: 'ทดสอบ ลูกค้า',
  guestIdCard: '1234567890123',
  guestContact: '0812345678',
  checkInDate: '2026-01-15T14:00:00.000Z',
  checkOutDate: '2026-01-20T12:00:00.000Z',
  rooms: [
    {
      roomNumber: '301',
      roomType: 'Standard',
      ratePerNight: 1200,
      nights: 5,
      subtotal: 6000,
    },
  ],
  subtotal: 6000,
  discount: 0,
  vatAmount: 420,
  grandTotal: 6420,
  paymentMethod: 'cash',
  paidAmount: 6420,
  createdAt: '2026-01-20T12:00:00.000Z',
  ...overrides,
})

export const createMockReceiptData = (
  overrides?: Partial<ReceiptData>
): ReceiptData => ({
  ...createMockInvoiceData(),
  receiptNumber: 'REC-2026-0001',
  cashierName: 'พนักงาน',
  ...overrides,
})

export const createMockHotelInfo = (overrides?: Partial<HotelInfo>): HotelInfo => ({
  name: 'โรงแรมทดสอบ',
  address: '123 ถนนทดสอบ ตำบลทดสอบ อำเภอเมือง จังหวัดทดสอบ 10000',
  phone: '02-123-4567',
  taxId: '1234567890123',
  logo: undefined,
  ...overrides,
})

// ============================================================================
// Batch Factories - Create Multiple Items
// ============================================================================

/**
 * Create an array of mock bookings with sequential IDs
 */
export const createMockBookings = (
  count: number,
  overrides?: Partial<Booking>
): Booking[] => {
  return Array.from({ length: count }, (_, index) =>
    createMockBooking({
      id: index + 1,
      customerId: 100 + index + 1,
      customerName: `ลูกค้า ${index + 1}`,
      roomNumber: `30${index + 1}`,
      ...overrides,
    })
  )
}

/**
 * Create an array of mock rooms with different statuses
 */
export const createMockRooms = (count: number): Room[] => {
  const statuses: RoomStatus[] = [
    'available',
    'occupied',
    'maintenance',
    'cleaning',
    'checkout',
    'booked',
  ]

  return Array.from({ length: count }, (_, index) =>
    createMockRoom({
      id: index + 1,
      roomNumber: `30${index + 1}`,
      status: statuses[index % statuses.length],
    })
  )
}

/**
 * Create an array of mock customers
 */
export const createMockCustomers = (count: number): MockCustomer[] => {
  return Array.from({ length: count }, (_, index) =>
    createMockCustomer({
      id: index + 1,
      firstName: `ชื่อ${index + 1}`,
      lastName: `นามสกุล${index + 1}`,
      phone: `08${String(index + 1).padStart(8, '0')}`,
    })
  )
}

/**
 * Create an array of mock inventory items across different categories
 */
export const createMockInventoryItems = (count: number): InventoryItem[] => {
  const categories: InventoryCategory[] = ['Minibar', 'Amenities', 'Linens', 'Equipment']
  const names: Record<InventoryCategory, string[]> = {
    Minibar: ['น้ำดื่ม', 'น้ำอัดลม', 'เบียร์', 'ถั่ว'],
    Amenities: ['สบู่', 'แชมพู', 'ครีมนวด', 'แปรงสีฟัน'],
    Linens: ['ผ้าเช็ดตัว', 'ผ้าปูที่นอน', 'หมอน', 'ผ้าห่ม'],
    Equipment: ['รีโมท', 'ไดร์เป่าผม', 'กาน้ำร้อน', 'ตู้เย็น'],
  }

  return Array.from({ length: count }, (_, index) => {
    const category = categories[index % categories.length]
    const nameList = names[category]
    const nameIndex = Math.floor(index / categories.length) % nameList.length

    return createMockInventoryItem({
      id: index + 1,
      itemCode: `${category.substring(0, 4).toUpperCase()}-${String(index + 1).padStart(3, '0')}`,
      itemName: nameList[nameIndex],
      category,
    })
  })
}
