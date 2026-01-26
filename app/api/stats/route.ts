import { NextResponse } from 'next/server';
import { getPool, sql } from '@/lib/db';

export const dynamic = 'force-dynamic';

export async function GET() {
  try {
    const pool = await getPool();

    // Total rooms count
    const totalRoomsResult = await pool.request().query(`
      SELECT COUNT(*) as count FROM HT_Rooms
    `);

    // Occupied rooms count - rooms that are in use or have a booking
    const occupiedRoomsResult = await pool.request().query(`
      SELECT COUNT(*) as count
      FROM HT_Rooms
      WHERE Room_Use = 'yes' OR (Room_Book IS NOT NULL AND Room_Book <> '')
    `);

    // Today's check-ins count
    const todayCheckInsResult = await pool.request().query(`
      SELECT COUNT(*) as count
      FROM View_CheckIn_Ds
      WHERE CAST(Cin_Room_In AS DATE) = CAST(GETDATE() AS DATE)
    `);

    // Today's check-outs count
    const todayCheckOutsResult = await pool.request().query(`
      SELECT COUNT(*) as count
      FROM View_CheckIn_Ds
      WHERE CAST(Cin_Room_Out AS DATE) = CAST(GETDATE() AS DATE)
    `);

    // Active bookings count - count bookings that are not cancelled
    const activeBookingsResult = await pool.request().query(`
      SELECT COUNT(*) as count
      FROM View_Booking_Ds
      WHERE Book_Status IS NOT NULL
    `);

    // Total customers count
    const totalCustomersResult = await pool.request().query(`
      SELECT COUNT(*) as count FROM View_Customers
    `);

    return NextResponse.json({
      success: true,
      data: {
        totalRooms: totalRoomsResult.recordset[0].count,
        occupiedRooms: occupiedRoomsResult.recordset[0].count,
        todayCheckIns: todayCheckInsResult.recordset[0].count,
        todayCheckOuts: todayCheckOutsResult.recordset[0].count,
        activeBookings: activeBookingsResult.recordset[0].count,
        totalCustomers: totalCustomersResult.recordset[0].count,
      },
    });
  } catch (error) {
    console.error('Error fetching stats:', error);
    return NextResponse.json(
      {
        success: false,
        error: 'Failed to fetch statistics',
      },
      { status: 500 }
    );
  }
}
