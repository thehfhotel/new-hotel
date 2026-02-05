/**
 * @jest-environment jsdom
 */

/**
 * Tests for the test utility functions themselves.
 * Ensures all utilities work correctly before using them in component tests.
 */

import {
  // Mock Factories
  createMockCustomer,
  createMockRoom,
  createMockBooking,
  createMockCheckIn,
  createMockInventoryItem,
  createMockInvoiceData,
  createMockHotelInfo,
  createMockBookings,
  createMockRooms,
  createMockCustomers,
  createMockInventoryItems,
  createMockDateRange,
  // Common Mocks
  mockFetch,
  mockFetchError,
  mockToday,
  mockTodayBuddhistYear,
  createMockDateString,
  createMockApiDateString,
  toBuddhistYear,
  // Test Utilities
  containsThaiText,
  isBuddhistEraDate,
  extractBuddhistYear,
  isThaiCurrencyFormat,
  createTestId,
  resetTestIdCounter,
  // Async utilities
  TIMEOUTS,
} from './index'

describe('Mock Factories', () => {
  describe('createMockCustomer', () => {
    it('creates a customer with default Thai values', () => {
      const customer = createMockCustomer()

      expect(customer.id).toBe(1)
      expect(customer.firstName).toBe('ทดสอบ')
      expect(customer.lastName).toBe('ลูกค้า')
      expect(customer.phone).toBe('08***REMOVED***')
      expect(customer.email).toBe('test@example.com')
      expect(customer.idCard).toBe('***REMOVED***90123')
    })

    it('allows overriding default values', () => {
      const customer = createMockCustomer({
        id: 100,
        firstName: 'สมชาย',
        lastName: 'ใจดี',
      })

      expect(customer.id).toBe(100)
      expect(customer.firstName).toBe('สมชาย')
      expect(customer.lastName).toBe('ใจดี')
      // Other values should still be defaults
      expect(customer.phone).toBe('08***REMOVED***')
    })
  })

  describe('createMockRoom', () => {
    it('creates a room with default values', () => {
      const room = createMockRoom()

      expect(room.id).toBe(1)
      expect(room.roomNumber).toBe('301')
      expect(room.type).toBe('Standard')
      expect(room.status).toBe('available')
      expect(room.floor).toBe(3)
    })

    it('allows overriding status', () => {
      const room = createMockRoom({ status: 'occupied' })

      expect(room.status).toBe('occupied')
    })
  })

  describe('createMockBooking', () => {
    it('creates a booking with Thai customer name', () => {
      const booking = createMockBooking()

      expect(booking.id).toBe(1)
      expect(booking.customerName).toBe('ทดสอบ ลูกค้า')
      expect(booking.status).toBe('confirmed')
    })

    it('includes proper date formats', () => {
      const booking = createMockBooking()

      expect(booking.checkInDate).toContain('T00:00:00.000Z')
      expect(booking.checkOutDate).toContain('T00:00:00.000Z')
    })
  })

  describe('createMockCheckIn', () => {
    it('creates a check-in record', () => {
      const checkIn = createMockCheckIn()

      expect(checkIn.id).toBe(1)
      expect(checkIn.customerName).toBe('ทดสอบ ลูกค้า')
      expect(checkIn.checkInTime).toContain('T14:00:00.000Z')
    })
  })

  describe('createMockInventoryItem', () => {
    it('creates an inventory item with Thai name', () => {
      const item = createMockInventoryItem()

      expect(item.itemName).toBe('น้ำดื่ม')
      expect(item.category).toBe('Minibar')
      expect(item.unit).toBe('ขวด')
    })
  })

  describe('createMockInvoiceData', () => {
    it('creates complete invoice data', () => {
      const invoice = createMockInvoiceData()

      expect(invoice.invoiceNumber).toMatch(/^INV-/)
      expect(invoice.guestName).toBe('ทดสอบ ลูกค้า')
      expect(invoice.rooms).toHaveLength(1)
      expect(invoice.grandTotal).toBe(6420)
    })
  })

  describe('createMockHotelInfo', () => {
    it('creates hotel info with Thai name and address', () => {
      const hotel = createMockHotelInfo()

      expect(hotel.name).toBe('โรงแรมทดสอบ')
      expect(hotel.address).toContain('ถนนทดสอบ')
    })
  })

  describe('Batch Factories', () => {
    it('createMockBookings creates multiple bookings', () => {
      const bookings = createMockBookings(5)

      expect(bookings).toHaveLength(5)
      expect(bookings[0].id).toBe(1)
      expect(bookings[4].id).toBe(5)
      expect(bookings[0].customerName).toBe('ลูกค้า 1')
    })

    it('createMockRooms creates rooms with different statuses', () => {
      const rooms = createMockRooms(6)

      expect(rooms).toHaveLength(6)
      expect(rooms[0].status).toBe('available')
      expect(rooms[1].status).toBe('occupied')
      expect(rooms[2].status).toBe('maintenance')
    })

    it('createMockCustomers creates customers with sequential data', () => {
      const customers = createMockCustomers(3)

      expect(customers).toHaveLength(3)
      expect(customers[0].firstName).toBe('ชื่อ1')
      expect(customers[1].firstName).toBe('ชื่อ2')
    })

    it('createMockInventoryItems creates items across categories', () => {
      const items = createMockInventoryItems(8)

      expect(items).toHaveLength(8)
      const categories = new Set(items.map((i) => i.category))
      expect(categories.size).toBe(4) // All 4 categories should be present
    })
  })
})

describe('Fetch Mocks', () => {
  describe('mockFetch', () => {
    it('returns matching response for URL pattern', async () => {
      const fetchMock = mockFetch({
        '/api/customers': { data: [{ id: 1 }], success: true },
      })

      const response = await fetchMock('/api/customers?search=test')
      const data = await response.json()

      expect(data.success).toBe(true)
      expect(data.data).toEqual([{ id: 1 }])
    })

    it('returns 404 for unmatched URLs', async () => {
      const fetchMock = mockFetch({})

      const response = await fetchMock('/api/unknown')
      const data = await response.json()

      expect(response.ok).toBe(false)
      expect(response.status).toBe(404)
      expect(data.success).toBe(false)
    })

    it('allows custom status codes', async () => {
      const fetchMock = mockFetch({
        '/api/error': { error: 'Server error', status: 500, ok: false },
      })

      const response = await fetchMock('/api/error')

      expect(response.ok).toBe(false)
      expect(response.status).toBe(500)
    })
  })

  describe('mockFetchError', () => {
    it('creates a mock that always returns error', async () => {
      const fetchMock = mockFetchError('Custom error', 400)

      const response = await fetchMock('/any/url')
      const data = await response.json()

      expect(response.ok).toBe(false)
      expect(response.status).toBe(400)
      expect(data.error).toBe('Custom error')
    })
  })
})

describe('Date Utilities', () => {
  describe('mockToday', () => {
    it('is set to February 5, 2026', () => {
      expect(mockToday.getFullYear()).toBe(2026)
      expect(mockToday.getMonth()).toBe(1) // February (0-indexed)
      expect(mockToday.getDate()).toBe(5)
    })
  })

  describe('mockTodayBuddhistYear', () => {
    it('is 2569', () => {
      expect(mockTodayBuddhistYear).toBe(2569)
    })
  })

  describe('toBuddhistYear', () => {
    it('converts Gregorian year to Buddhist Era', () => {
      const date = new Date('2026-01-15')
      expect(toBuddhistYear(date)).toBe(2569)
    })
  })

  describe('createMockDateString', () => {
    it('creates ISO date string', () => {
      const dateStr = createMockDateString(2026, 2, 15)
      expect(dateStr).toBe('2026-02-15T00:00:00.000Z')
    })
  })

  describe('createMockApiDateString', () => {
    it('creates API format date string', () => {
      const dateStr = createMockApiDateString(2026, 2, 15)
      expect(dateStr).toBe('2026-02-15')
    })
  })

  describe('createMockDateRange', () => {
    it('creates a date range starting from today', () => {
      const range = createMockDateRange(0, 3)

      expect(range.checkIn).toBe('2026-02-05')
      expect(range.checkOut).toBe('2026-02-08')
    })

    it('creates a future date range', () => {
      const range = createMockDateRange(10, 5)

      expect(range.checkIn).toBe('2026-02-15')
      expect(range.checkOut).toBe('2026-02-20')
    })
  })
})

describe('Assertion Utilities', () => {
  describe('containsThaiText', () => {
    it('returns true for Thai text', () => {
      const element = document.createElement('div')
      element.textContent = 'สวัสดี'
      expect(containsThaiText(element)).toBe(true)
    })

    it('returns false for English text only', () => {
      const element = document.createElement('div')
      element.textContent = 'Hello'
      expect(containsThaiText(element)).toBe(false)
    })

    it('returns true for mixed content', () => {
      const element = document.createElement('div')
      element.textContent = 'Room: ห้อง 301'
      expect(containsThaiText(element)).toBe(true)
    })
  })

  describe('isBuddhistEraDate', () => {
    it('returns true for Buddhist Era year', () => {
      expect(isBuddhistEraDate('15/01/2569')).toBe(true)
      expect(isBuddhistEraDate('25 ก.พ. 2567')).toBe(true)
    })

    it('returns false for Gregorian year', () => {
      expect(isBuddhistEraDate('15/01/2026')).toBe(false)
    })

    it('returns false for invalid format', () => {
      expect(isBuddhistEraDate('no date here')).toBe(false)
    })
  })

  describe('extractBuddhistYear', () => {
    it('extracts year from date string', () => {
      expect(extractBuddhistYear('15/01/2569')).toBe(2569)
      expect(extractBuddhistYear('25 ก.พ. 2567')).toBe(2567)
    })

    it('returns null for invalid format', () => {
      expect(extractBuddhistYear('no date')).toBeNull()
    })
  })

  describe('isThaiCurrencyFormat', () => {
    it('recognizes Thai Baht formats', () => {
      expect(isThaiCurrencyFormat('฿1,000')).toBe(true)
      expect(isThaiCurrencyFormat('1,000 บาท')).toBe(true)
      expect(isThaiCurrencyFormat('฿10,500.50')).toBe(true)
    })

    it('returns false for other formats', () => {
      expect(isThaiCurrencyFormat('$100')).toBe(false)
      expect(isThaiCurrencyFormat(null)).toBe(false)
    })
  })
})

describe('Test ID Utilities', () => {
  beforeEach(() => {
    resetTestIdCounter()
  })

  it('generates sequential IDs', () => {
    expect(createTestId()).toBe(1)
    expect(createTestId()).toBe(2)
    expect(createTestId()).toBe(3)
  })

  it('resets counter', () => {
    createTestId()
    createTestId()
    resetTestIdCounter()
    expect(createTestId()).toBe(1)
  })
})

describe('Timeout Constants', () => {
  it('has expected timeout values', () => {
    expect(TIMEOUTS.DEBOUNCE).toBe(300)
    expect(TIMEOUTS.API_CALL).toBe(1000)
    expect(TIMEOUTS.MODAL_ANIMATION).toBe(200)
    expect(TIMEOUTS.LOADING).toBe(3000)
    expect(TIMEOUTS.EXTENDED).toBe(5000)
  })
})
