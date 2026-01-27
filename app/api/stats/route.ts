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

    // Occupied rooms count - rooms with guests checked in (excludes checkout rooms after 6 AM)
    const occupiedRoomsResult = await pool.request().query(`
      SELECT COUNT(*) as count
      FROM HT_Rooms
      WHERE Room_Use = 'yes'
        AND Room_no NOT IN (
          SELECT DISTINCT c.Cin_Room_No
          FROM View_CheckIn_Ds c
          WHERE CAST(c.Cin_Room_Out AS DATE) = CAST(GETDATE() AS DATE)
            AND DATEPART(HOUR, GETDATE()) >= 6
            AND c.Cin_Room_In = (
              SELECT MAX(c2.Cin_Room_In)
              FROM View_CheckIn_Ds c2
              WHERE c2.Cin_Room_No = c.Cin_Room_No
            )
        )
    `);

    // Checkout rooms count - rooms with checkout today (after 6 AM)
    const checkoutRoomsResult = await pool.request().query(`
      SELECT COUNT(DISTINCT r.Room_no) as count
      FROM HT_Rooms r
      INNER JOIN View_CheckIn_Ds c ON r.Room_no = c.Cin_Room_No
      WHERE r.Room_Use = 'yes'
        AND CAST(c.Cin_Room_Out AS DATE) = CAST(GETDATE() AS DATE)
        AND DATEPART(HOUR, GETDATE()) >= 6
        AND c.Cin_Room_In = (
          SELECT MAX(c2.Cin_Room_In)
          FROM View_CheckIn_Ds c2
          WHERE c2.Cin_Room_No = c.Cin_Room_No
        )
    `);

    // Booked rooms count - rooms with booking but not checked in
    const bookedRoomsResult = await pool.request().query(`
      SELECT COUNT(*) as count
      FROM HT_Rooms
      WHERE Room_Use <> 'yes' AND Room_Book IS NOT NULL AND Room_Book <> ''
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
        checkoutRooms: checkoutRoomsResult.recordset[0].count,
        bookedRooms: bookedRoomsResult.recordset[0].count,
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
