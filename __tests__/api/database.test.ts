/**
 * Database Connection Tests
 * These tests verify the database connection and basic queries work correctly.
 */

import { getPool, sql } from '@/lib/db'

describe('Database Connection', () => {
  let pool: Awaited<ReturnType<typeof getPool>>

  beforeAll(async () => {
    pool = await getPool()
  })

  afterAll(async () => {
    // Close the connection pool after tests
    if (pool) {
      await pool.close()
    }
  })

  test('should connect to database successfully', async () => {
    expect(pool).toBeDefined()
    expect(pool.connected).toBe(true)
  })

  test('should query HT_Rooms table', async () => {
    const result = await pool.request().query('SELECT COUNT(*) as count FROM HT_Rooms')
    expect(result.recordset).toBeDefined()
    expect(result.recordset[0].count).toBeGreaterThan(0)
  })

  test('should query View_Customers view', async () => {
    const result = await pool.request().query('SELECT COUNT(*) as count FROM View_Customers')
    expect(result.recordset).toBeDefined()
    expect(result.recordset[0].count).toBeGreaterThan(0)
  })

  test('should query View_Booking_Ds view', async () => {
    const result = await pool.request().query('SELECT COUNT(*) as count FROM View_Booking_Ds')
    expect(result.recordset).toBeDefined()
    expect(result.recordset[0].count).toBeGreaterThan(0)
  })

  test('should query View_CheckIn_Ds view', async () => {
    const result = await pool.request().query('SELECT COUNT(*) as count FROM View_CheckIn_Ds')
    expect(result.recordset).toBeDefined()
    expect(result.recordset[0].count).toBeGreaterThan(0)
  })

  test('should return correct room columns', async () => {
    const result = await pool.request().query(`
      SELECT TOP 1
        Room_no,
        Room_Type,
        Room_Clean,
        Room_Use,
        Room_Book,
        Room_Manternace
      FROM HT_Rooms
    `)

    expect(result.recordset.length).toBe(1)
    const room = result.recordset[0]
    expect(room).toHaveProperty('Room_no')
    expect(room).toHaveProperty('Room_Type')
    expect(room).toHaveProperty('Room_Clean')
    expect(room).toHaveProperty('Room_Use')
    expect(room).toHaveProperty('Room_Book')
    expect(room).toHaveProperty('Room_Manternace')
  })

  test('should return correct booking columns', async () => {
    const result = await pool.request().query(`
      SELECT TOP 1
        Book_No,
        Book_Date,
        Book_Date_in,
        Book_Date_out,
        Book_Cust_Name,
        Book_Status,
        Book_Room_Type
      FROM View_Booking_Ds
    `)

    expect(result.recordset.length).toBe(1)
    const booking = result.recordset[0]
    expect(booking).toHaveProperty('Book_No')
    expect(booking).toHaveProperty('Book_Date')
    expect(booking).toHaveProperty('Book_Date_in')
    expect(booking).toHaveProperty('Book_Date_out')
    expect(booking).toHaveProperty('Book_Cust_Name')
    expect(booking).toHaveProperty('Book_Status')
    expect(booking).toHaveProperty('Book_Room_Type')
  })

  test('should return correct checkin columns', async () => {
    const result = await pool.request().query(`
      SELECT TOP 1
        Cin_no,
        Cin_Room_No,
        Cin_Room_In,
        Cin_Room_Out,
        Cin_cust_name,
        Cin_status
      FROM View_CheckIn_Ds
    `)

    expect(result.recordset.length).toBe(1)
    const checkin = result.recordset[0]
    expect(checkin).toHaveProperty('Cin_no')
    expect(checkin).toHaveProperty('Cin_Room_No')
    expect(checkin).toHaveProperty('Cin_Room_In')
    expect(checkin).toHaveProperty('Cin_Room_Out')
    expect(checkin).toHaveProperty('Cin_cust_name')
    expect(checkin).toHaveProperty('Cin_status')
  })

  test('should return correct customer columns', async () => {
    const result = await pool.request().query(`
      SELECT TOP 1
        Cust_no,
        Cust_name,
        Cust_Type,
        Cust_Add_tel,
        Cust_IDcard,
        C_Address
      FROM View_Customers
    `)

    expect(result.recordset.length).toBe(1)
    const customer = result.recordset[0]
    expect(customer).toHaveProperty('Cust_no')
    expect(customer).toHaveProperty('Cust_name')
    expect(customer).toHaveProperty('Cust_Type')
    expect(customer).toHaveProperty('Cust_Add_tel')
    expect(customer).toHaveProperty('Cust_IDcard')
    expect(customer).toHaveProperty('C_Address')
  })

  test('room flags should be yes/no strings', async () => {
    const result = await pool.request().query(`
      SELECT TOP 10
        Room_Clean,
        Room_Use,
        Room_Manternace
      FROM HT_Rooms
    `)

    for (const room of result.recordset) {
      // Room_Clean should be 'yes' or 'no'
      expect(['yes', 'no']).toContain(room.Room_Clean)
      // Room_Use should be 'yes' or 'no'
      expect(['yes', 'no']).toContain(room.Room_Use)
      // Room_Manternace should be 'yes' or 'no'
      expect(['yes', 'no']).toContain(room.Room_Manternace)
    }
  })
})
