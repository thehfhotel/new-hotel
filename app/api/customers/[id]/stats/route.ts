import { NextRequest, NextResponse } from 'next/server';
import { getPool, sql } from '@/lib/db';

export const dynamic = 'force-dynamic';

export async function GET(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    // Customer ID comes as "C0001" format - use as-is for Book_Cust_ID/Cin_cust_no match
    const { id: custId } = await params;

    const pool = await getPool();

    // Query 1: Total bookings and booking stats
    const bookingStatsRequest = pool.request();
    bookingStatsRequest.input('custId', sql.NVarChar, custId);
    const bookingStatsResult = await bookingStatsRequest.query(`
      SELECT
        COUNT(*) as totalBookings,
        MIN(Book_Date_in) as firstVisit,
        MAX(Book_Date_out) as lastVisit,
        AVG(CAST(DATEDIFF(day, Book_Date_in, Book_Date_out) AS FLOAT)) as avgStayDays
      FROM View_Booking_Ds
      WHERE Book_Cust_ID = @custId
    `);

    // Query 2: Total stays (check-ins)
    const staysRequest = pool.request();
    staysRequest.input('custId', sql.NVarChar, custId);
    const staysResult = await staysRequest.query(`
      SELECT COUNT(*) as totalStays
      FROM View_CheckIn_Ds
      WHERE Cin_cust_no = @custId
    `);

    // Query 3: Favorite room type (most booked)
    const favoriteRoomRequest = pool.request();
    favoriteRoomRequest.input('custId', sql.NVarChar, custId);
    const favoriteRoomResult = await favoriteRoomRequest.query(`
      SELECT TOP 1 Book_Room_Type, COUNT(*) as cnt
      FROM View_Booking_Ds
      WHERE Book_Cust_ID = @custId AND Book_Room_Type IS NOT NULL
      GROUP BY Book_Room_Type
      ORDER BY cnt DESC
    `);

    const bookingStats = bookingStatsResult.recordset[0];
    const staysStats = staysResult.recordset[0];
    const favoriteRoom = favoriteRoomResult.recordset[0];

    return NextResponse.json({
      success: true,
      stats: {
        totalBookings: bookingStats?.totalBookings || 0,
        totalStays: staysStats?.totalStays || 0,
        firstVisit: bookingStats?.firstVisit || null,
        lastVisit: bookingStats?.lastVisit || null,
        favoriteRoomType: favoriteRoom?.Book_Room_Type || null,
        avgStayDays: bookingStats?.avgStayDays ? Math.round(bookingStats.avgStayDays * 10) / 10 : null,
      },
    });
  } catch (error) {
    console.error('Error fetching customer stats:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to fetch customer stats' },
      { status: 500 }
    );
  }
}
