/**
 * API Endpoint Tests
 * These tests verify the API endpoints return correct data structures.
 */

const API_BASE = 'http://localhost:3003'

describe('API Endpoints', () => {
  describe('GET /api/stats', () => {
    test('should return stats with correct structure', async () => {
      const response = await fetch(`${API_BASE}/api/stats`)
      const data = await response.json()

      expect(response.ok).toBe(true)
      expect(data.success).toBe(true)
      expect(data.data).toBeDefined()

      // Verify stats fields exist and are numbers
      expect(typeof data.data.totalRooms).toBe('number')
      expect(typeof data.data.occupiedRooms).toBe('number')
      expect(typeof data.data.todayCheckIns).toBe('number')
      expect(typeof data.data.todayCheckOuts).toBe('number')
      expect(typeof data.data.activeBookings).toBe('number')
      expect(typeof data.data.totalCustomers).toBe('number')

      // Verify reasonable values
      expect(data.data.totalRooms).toBeGreaterThan(0)
      expect(data.data.totalRooms).toBe(58) // Known value from database
      expect(data.data.totalCustomers).toBeGreaterThan(20000) // Known: ~20447
    })
  })

  describe('GET /api/rooms', () => {
    test('should return all rooms with correct structure', async () => {
      const response = await fetch(`${API_BASE}/api/rooms`)
      const data = await response.json()

      expect(response.ok).toBe(true)
      expect(data.success).toBe(true)
      expect(data.data).toBeDefined()
      expect(Array.isArray(data.data)).toBe(true)
      expect(data.data.length).toBe(58)

      // Verify first room structure
      const room = data.data[0]
      expect(room).toHaveProperty('Room_no')
      expect(room).toHaveProperty('Room_Type')
      expect(room).toHaveProperty('Room_Clean')
      expect(room).toHaveProperty('Room_Use')
      expect(room).toHaveProperty('Room_Book')
      expect(room).toHaveProperty('Room_Manternace')
    })

    test('room flags should be yes/no strings', async () => {
      const response = await fetch(`${API_BASE}/api/rooms`)
      const data = await response.json()

      for (const room of data.data) {
        expect(['yes', 'no']).toContain(room.Room_Clean)
        expect(['yes', 'no']).toContain(room.Room_Use)
        expect(['yes', 'no']).toContain(room.Room_Manternace)
      }
    })
  })

  describe('GET /api/bookings', () => {
    test('should return bookings with pagination', async () => {
      const response = await fetch(`${API_BASE}/api/bookings?limit=5`)
      const data = await response.json()

      expect(response.ok).toBe(true)
      expect(data.success).toBe(true)
      expect(data.data).toBeDefined()
      expect(Array.isArray(data.data)).toBe(true)
      expect(data.data.length).toBeLessThanOrEqual(5)

      // Verify pagination
      expect(data.pagination).toBeDefined()
      expect(data.pagination.page).toBe(1)
      expect(data.pagination.limit).toBe(5)
      expect(data.pagination.total).toBeGreaterThan(0)
      expect(data.pagination.totalPages).toBeGreaterThan(0)
    })

    test('should return correct booking structure', async () => {
      const response = await fetch(`${API_BASE}/api/bookings?limit=1`)
      const data = await response.json()

      const booking = data.data[0]
      expect(booking).toHaveProperty('Book_No')
      expect(booking).toHaveProperty('Book_Date')
      expect(booking).toHaveProperty('Book_Date_in')
      expect(booking).toHaveProperty('Book_Date_out')
      expect(booking).toHaveProperty('Book_Cust_Name')
      expect(booking).toHaveProperty('Book_Status')
      expect(booking).toHaveProperty('Book_Room_Type')

      // Book_Status should be a number
      expect(typeof booking.Book_Status).toBe('number')
    })

    test('pagination should work correctly', async () => {
      const page1 = await fetch(`${API_BASE}/api/bookings?limit=5&page=1`).then(r => r.json())
      const page2 = await fetch(`${API_BASE}/api/bookings?limit=5&page=2`).then(r => r.json())

      // Different pages should have different data
      expect(page1.data[0].Book_No).not.toBe(page2.data[0].Book_No)

      // Pagination info should reflect page number
      expect(page1.pagination.page).toBe(1)
      expect(page2.pagination.page).toBe(2)
    })
  })

  describe('GET /api/customers', () => {
    test('should return customers with pagination', async () => {
      const response = await fetch(`${API_BASE}/api/customers?limit=10`)
      const data = await response.json()

      expect(response.ok).toBe(true)
      expect(data.success).toBe(true)
      expect(data.data).toBeDefined()
      expect(Array.isArray(data.data)).toBe(true)
      expect(data.data.length).toBeLessThanOrEqual(10)

      // Verify pagination
      expect(data.pagination).toBeDefined()
      expect(data.pagination.total).toBeGreaterThan(20000) // Known: ~20447
    })

    test('should return correct customer structure', async () => {
      const response = await fetch(`${API_BASE}/api/customers?limit=1`)
      const data = await response.json()

      const customer = data.data[0]
      expect(customer).toHaveProperty('Cust_no')
      expect(customer).toHaveProperty('Cust_name')
      expect(customer).toHaveProperty('Cust_Type')
      expect(customer).toHaveProperty('Cust_Add_tel')
      expect(customer).toHaveProperty('Cust_IDcard')
      expect(customer).toHaveProperty('C_Address')
    })

    test('search should filter by name', async () => {
      const response = await fetch(`${API_BASE}/api/customers?search=สมชาย&limit=100`)
      const data = await response.json()

      expect(data.success).toBe(true)
      // All results should contain the search term
      for (const customer of data.data) {
        expect(customer.Cust_name.toLowerCase()).toContain('สมชาย'.toLowerCase())
      }
    })
  })

  describe('GET /api/checkins', () => {
    test('should return checkins with pagination', async () => {
      const response = await fetch(`${API_BASE}/api/checkins?limit=10`)
      const data = await response.json()

      expect(response.ok).toBe(true)
      expect(data.success).toBe(true)
      expect(data.data).toBeDefined()
      expect(Array.isArray(data.data)).toBe(true)
      expect(data.data.length).toBeLessThanOrEqual(10)

      // Verify pagination
      expect(data.pagination).toBeDefined()
      expect(data.pagination.total).toBeGreaterThan(0)
    })

    test('should return correct checkin structure', async () => {
      const response = await fetch(`${API_BASE}/api/checkins?limit=1`)
      const data = await response.json()

      const checkin = data.data[0]
      expect(checkin).toHaveProperty('Cin_no')
      expect(checkin).toHaveProperty('Cin_Room_No')
      expect(checkin).toHaveProperty('Cin_Room_In')
      expect(checkin).toHaveProperty('Cin_Room_Out')
      expect(checkin).toHaveProperty('Cin_cust_name')
      expect(checkin).toHaveProperty('Cin_status')
    })

    test('checkins should be ordered by date descending', async () => {
      const response = await fetch(`${API_BASE}/api/checkins?limit=5`)
      const data = await response.json()

      for (let i = 1; i < data.data.length; i++) {
        const prevDate = new Date(data.data[i - 1].Cin_Room_In)
        const currDate = new Date(data.data[i].Cin_Room_In)
        expect(prevDate.getTime()).toBeGreaterThanOrEqual(currDate.getTime())
      }
    })
  })
})
